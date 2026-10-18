//! Single-file publication with a flushed sibling stage and recoverable backup.
//! Renames are not a durable multi-file transaction; see project-save-safety.md.
use std::fs::{self, Metadata, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

fn link(metadata: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}

pub(crate) fn validate(path: &Path) -> io::Result<()> {
    validate_target(path, true)
}

pub(crate) fn validate_asset(path: &Path) -> io::Result<()> {
    validate_target(path, false)
}

pub(crate) fn validate_project(path: &Path) -> io::Result<()> {
    validate(path)?;
    let m = fs::symlink_metadata(parent(path))?;
    if !m.is_dir() || link(&m) {
        return Err(io::Error::other(
            "project parent must be an existing ordinary directory",
        ));
    }
    Ok(())
}

fn validate_target(path: &Path, writable: bool) -> io::Result<()> {
    if path.file_name().and_then(|n| n.to_str()).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "a nonempty Unicode filename is required",
        ));
    }
    for component in path.components() {
        #[cfg(windows)]
        if let Component::Prefix(prefix) = component {
            use std::path::Prefix;
            if !matches!(
                prefix.kind(),
                Prefix::Disk(_)
                    | Prefix::VerbatimDisk(_)
                    | Prefix::UNC(_, _)
                    | Prefix::VerbatimUNC(_, _)
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Windows device namespaces are not project paths",
                ));
            }
        }
        if let Component::Normal(name) = component {
            let name = name
                .to_str()
                .ok_or_else(|| io::Error::other("non-Unicode path component"))?;
            let stem = name
                .split('.')
                .next()
                .unwrap_or_default()
                .trim_end()
                .to_uppercase();
            let device = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"].contains(&stem.as_str())
                || ["COM", "LPT"].iter().any(|p| {
                    stem.strip_prefix(p).is_some_and(|n| {
                        matches!(
                            n,
                            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                        )
                    })
                });
            if name.trim() != name
                || name.ends_with('.')
                || device
                || name
                    .chars()
                    .any(|c| c.is_control() || "<>:\"|?*".contains(c))
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid portable Windows path component: {name:?}"),
                ));
            }
        }
    }
    let parent = parent(path);
    for ancestor in parent.ancestors() {
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        match fs::symlink_metadata(ancestor) {
            Ok(m) if !m.is_dir() || link(&m) => {
                return Err(io::Error::other(
                    "save parents must be ordinary directories; symlinks/reparse points are refused",
                ));
            }
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    match fs::symlink_metadata(path) {
        Ok(m) if !m.is_file() || link(&m) || (writable && m.permissions().readonly()) => {
            Err(io::Error::other(
                "save target must be an ordinary file; directory, read-only write targets and linked targets are refused",
            ))
        }
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

fn parent(path: &Path) -> &Path {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Prepare,
    Flush,
    Backup,
    Publish,
    Restore,
    Cleanup,
}

pub(crate) fn write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    write_with(path, bytes, |_| Ok(()))
}

fn write_with(
    path: &Path,
    bytes: &[u8],
    mut before: impl FnMut(Step) -> io::Result<()>,
) -> io::Result<()> {
    validate(path)?;
    let directory = fs::canonicalize(parent(path))?;
    let destination = directory.join(path.file_name().unwrap());
    let lock_path = directory.join(format!(
        ".{}.ankh-write-lock",
        path.file_name().unwrap().to_string_lossy()
    ));
    let lock = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_path)
        .map_err(|e| {
            io::Error::new(
                e.kind(),
                format!(
                    "save lock {}: {e}; inspect a stale lock before retrying",
                    lock_path.display()
                ),
            )
        })?;
    drop(lock);
    let mut stage = None;
    let mut backed_up = false;
    let attempt = (|| -> io::Result<()> {
        validate(&destination)?;
        for _ in 0..100 {
            let candidate = directory.join(format!(
                ".ankh-save-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&candidate) {
                Ok(()) => {
                    stage = Some(candidate);
                    break;
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e),
            }
        }
        let staging = stage
            .as_ref()
            .ok_or_else(|| io::Error::other("cannot allocate save staging directory"))?;
        let manifest = serde_json::json!({"save_version": 1, "destination": destination.to_str().ok_or_else(|| io::Error::other("non-Unicode save destination"))?});
        let mut inventory = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(staging.join("manifest.json"))?;
        serde_json::to_writer(&mut inventory, &manifest)?;
        inventory.sync_all()?;
        drop(inventory);
        before(Step::Prepare)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(staging.join("new"))?;
        file.write_all(bytes)?;
        before(Step::Flush)?;
        file.sync_all()?;
        drop(file);
        validate(&destination)?;
        if destination.exists() {
            before(Step::Backup)?;
            fs::rename(&destination, staging.join("previous"))?;
            backed_up = true;
        }
        before(Step::Publish)?;
        match fs::symlink_metadata(&destination) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(io::Error::other(
                    "save destination was occupied during publication",
                ));
            }
            Err(e) => return Err(e),
        }
        fs::rename(staging.join("new"), &destination)?;
        Ok(())
    })();
    let mut cleanup_errors = Vec::new();
    if let Err(cause) = attempt {
        let mut restored = true;
        if backed_up {
            let staging = stage.as_ref().unwrap();
            let restore = (|| {
                before(Step::Restore)?;
                match fs::symlink_metadata(&destination) {
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                    Ok(_) => return Err(io::Error::other("destination occupied; backup retained")),
                    Err(e) => return Err(e),
                }
                fs::rename(staging.join("previous"), &destination)
            })();
            if let Err(e) = restore {
                restored = false;
                cleanup_errors.push(format!("restore: {e}"));
            }
        }
        if restored
            && let Some(staging) = &stage
            && let Err(e) = cleanup(staging)
        {
            cleanup_errors.push(format!("cleanup: {e}"));
        }
        if let Err(e) = fs::remove_file(&lock_path) {
            cleanup_errors.push(format!("lock cleanup: {e}"));
        }
        return Err(io::Error::new(
            cause.kind(),
            format!(
                "save {} failed: {cause}; {}; recovery directory: {}; {}",
                destination.display(),
                if restored {
                    "previous destination preserved/restored"
                } else {
                    "ROLLBACK INCOMPLETE; previous backup retained"
                },
                stage
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "not created".into()),
                cleanup_errors.join("; ")
            ),
        ));
    }
    if let Some(staging) = &stage
        && let Err(e) = before(Step::Cleanup).and_then(|_| cleanup(staging))
    {
        cleanup_errors.push(format!("{}: {e}", staging.display()));
    }
    if let Err(e) = fs::remove_file(&lock_path) {
        cleanup_errors.push(format!("{}: {e}", lock_path.display()));
    }
    if !cleanup_errors.is_empty() {
        return Err(io::Error::other(format!(
            "new project published, but save cleanup failed: {}; inspect retained recovery files",
            cleanup_errors.join("; ")
        )));
    }
    Ok(())
}

fn cleanup(stage: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(stage)?;
    if !metadata.is_dir() || link(&metadata) {
        return Err(io::Error::other("save staging directory changed"));
    }
    for name in ["new", "previous", "manifest.json"] {
        match fs::remove_file(stage.join(name)) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    fs::remove_dir(stage)
}

/// Inventory candidates left by interrupted writes. Caller validates their
/// project payload and asks before recovering; never restore/delete silently.
pub(crate) fn recoverable(path: &Path) -> Vec<PathBuf> {
    if validate_asset(path).is_err() {
        return vec![];
    }
    let Ok(directory) = fs::canonicalize(parent(path)) else {
        return vec![];
    };
    let Some(name) = path.file_name() else {
        return vec![];
    };
    let expected = directory.join(name).to_string_lossy().to_string();
    let Ok(entries) = fs::read_dir(&directory) else {
        return vec![];
    };
    let mut result = Vec::new();
    for entry in entries.flatten() {
        let stage = entry.path();
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with(".ankh-save-")
        {
            continue;
        }
        let Ok(m) = fs::symlink_metadata(&stage) else {
            continue;
        };
        if !m.is_dir() || link(&m) {
            continue;
        }
        let Ok(bytes) = fs::read(stage.join("manifest.json")) else {
            continue;
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            continue;
        };
        let Some(destination) = value["destination"].as_str() else {
            continue;
        };
        #[cfg(windows)]
        let matches = destination.to_lowercase() == expected.to_lowercase();
        #[cfg(not(windows))]
        let matches = destination == expected;
        if value["save_version"] != 1 || !matches {
            continue;
        }
        for name in ["new", "previous"] {
            let candidate = stage.join(name);
            if fs::symlink_metadata(&candidate).is_ok_and(|m| m.is_file() && !link(&m)) {
                result.push(candidate);
            }
        }
    }
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prepare_flush_backup_and_publish_failures_preserve_the_original() {
        for failure in [Step::Prepare, Step::Flush, Step::Backup, Step::Publish] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("rig.ankh");
            fs::write(&path, b"original").unwrap();
            let err = write_with(&path, b"new", |step| {
                if step == failure {
                    Err(io::Error::other("injected"))
                } else {
                    Ok(())
                }
            })
            .unwrap_err();
            assert!(err.to_string().contains("preserved/restored"));
            assert_eq!(fs::read(&path).unwrap(), b"original");
            assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
        }
    }
    #[test]
    fn failed_restore_retains_backup_and_manifest_for_recovery() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        fs::write(&path, b"original").unwrap();
        let err = write_with(&path, b"new", |step| {
            if matches!(step, Step::Publish | Step::Restore) {
                Err(io::Error::other("injected"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(err.to_string().contains("ROLLBACK INCOMPLETE"));
        let files = recoverable(&path);
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| fs::read(p).unwrap() == b"original"));
        assert!(files.iter().all(|p| {
            err.to_string()
                .contains(&p.parent().unwrap().display().to_string())
        }));
    }
    #[test]
    fn cleanup_failure_reports_publication_and_preserves_previous_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        fs::write(&path, b"original").unwrap();
        let err = write_with(&path, b"new", |step| {
            if step == Step::Cleanup {
                Err(io::Error::other("injected"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(err.to_string().contains("new project published"));
        assert_eq!(fs::read(&path).unwrap(), b"new");
        assert!(
            recoverable(&path)
                .iter()
                .any(|p| fs::read(p).unwrap() == b"original")
        );
    }
    #[cfg(windows)]
    #[test]
    fn windows_file_lock_failure_preserves_original_and_removes_stage() {
        use std::os::windows::fs::OpenOptionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        fs::write(&path, b"original").unwrap();
        let held = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(3)
            .open(&path)
            .unwrap();
        assert!(write(&path, b"new").is_err());
        drop(held);
        assert_eq!(fs::read(&path).unwrap(), b"original");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }
    #[test]
    fn invalid_paths_and_occupied_lock_fail_before_publication() {
        let dir = tempfile::tempdir().unwrap();
        for name in [
            "CON.ankh",
            "file:stream",
            "rig. ",
            "aux.json",
            "quote\".ankh",
        ] {
            assert!(write(&dir.path().join(name), b"new").is_err());
        }
        let path = dir.path().join("rig.ankh");
        fs::write(dir.path().join(".rig.ankh.ankh-write-lock"), b"other owner").unwrap();
        assert!(write(&path, b"new").is_err());
        assert_eq!(
            fs::read(dir.path().join(".rig.ankh.ankh-write-lock")).unwrap(),
            b"other owner"
        );
        assert!(!path.exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_device_namespaces_are_rejected_before_io() {
        for path in [
            r"\\.\C:\rig.ankh",
            r"\\?\GLOBALROOT\Device\HarddiskVolume1\rig.ankh",
        ] {
            assert_eq!(
                validate(Path::new(path)).unwrap_err().kind(),
                io::ErrorKind::InvalidInput
            );
        }
    }

    #[test]
    fn readonly_primary_and_permission_denied_preparation_keep_valid_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        fs::write(&path, b"original").unwrap();
        let permissions = fs::metadata(&path).unwrap().permissions();
        let mut readonly = permissions.clone();
        readonly.set_readonly(true);
        fs::set_permissions(&path, readonly).unwrap();
        assert!(write(&path, b"new").is_err());
        fs::set_permissions(&path, permissions).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"original");
        let error = write_with(&path, b"new", |step| {
            if step == Step::Prepare {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected ACL denial",
                ))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(fs::read(&path).unwrap(), b"original");
    }

    #[test]
    fn rollback_never_overwrites_a_newly_occupied_destination() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        fs::write(&path, b"original").unwrap();
        let err = write_with(&path, b"new", |step| {
            if step == Step::Publish {
                fs::write(&path, b"another writer").unwrap();
                Err(io::Error::other("interference"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(err.to_string().contains("ROLLBACK INCOMPLETE"));
        assert_eq!(fs::read(&path).unwrap(), b"another writer");
        assert!(
            recoverable(&path)
                .iter()
                .any(|p| fs::read(p).unwrap() == b"original")
        );
    }
}
