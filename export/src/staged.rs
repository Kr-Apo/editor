//! A same-filesystem export transaction. No filesystem-wide atomicity is claimed.
use crate::run::{ExportError, Plan};
use crate::template::confine;
use std::fs::{self, Metadata, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const LOCK: &str = ".ankhimate-export.lock";
static NEXT_STAGE: AtomicU64 = AtomicU64::new(0);

fn error(path: &Path, reason: impl ToString) -> ExportError {
    ExportError::Io {
        path: path.display().to_string(),
        reason: reason.to_string(),
    }
}

pub(crate) fn is_link(metadata: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // Includes junctions, not only symlinks. Do not follow reparse points.
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}

/// Use a portable subset of Windows names even on other hosts. Case aliases and
/// file/directory conflicts must not appear only when an export reaches Windows.
pub(crate) fn validate_paths(plan: &Plan) -> Result<Vec<String>, ExportError> {
    let mut result: Vec<String> = Vec::new();
    let mut folded: Vec<String> = Vec::new();
    for raw in plan.paths() {
        let normalized = validate_path(raw)?;
        let key = normalized.to_lowercase();
        if let Some(i) = folded.iter().position(|previous| {
            previous == &key
                || previous.starts_with(&format!("{key}/"))
                || key.starts_with(&format!("{previous}/"))
        }) {
            return Err(ExportError::DuplicatePath {
                path: normalized,
                first: result[i].clone(),
                second: raw.to_string(),
            });
        }
        result.push(normalized);
        folded.push(key);
    }
    Ok(result)
}

pub(crate) fn validate_path(raw: &str) -> Result<String, ExportError> {
    let slash = raw.replace('\\', "/");
    for segment in slash.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            continue;
        }
        let stem = segment
            .split('.')
            .next()
            .unwrap_or_default()
            .trim_end()
            .to_uppercase();
        let reserved = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"].contains(&stem.as_str())
            || ["COM", "LPT"].iter().any(|prefix| {
                stem.strip_prefix(prefix).is_some_and(|n| {
                    matches!(
                        n,
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                    )
                })
            });
        if segment.trim() != segment
            || segment.ends_with('.')
            || reserved
            || segment.to_lowercase().starts_with(".ankhimate-export")
            || segment
                .chars()
                .any(|c| c.is_control() || "<>:\"|?*".contains(c))
        {
            return Err(error(
                Path::new(raw),
                "invalid portable output name (Windows reserved name, character, suffix or export transaction namespace)",
            ));
        }
    }
    confine("export output", raw).map_err(ExportError::from)
}

// Inspect without following links, including every existing ancestor of root.
pub(crate) fn validate_directory(path: &Path) -> io::Result<()> {
    for ancestor in path.ancestors() {
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        match fs::symlink_metadata(ancestor) {
            Ok(metadata) if !metadata.is_dir() || is_link(&metadata) => {
                return Err(io::Error::other(format!(
                    "{} is not an ordinary directory (links/reparse points are refused)",
                    ancestor.display()
                )));
            }
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn validate_destination(path: &Path) -> io::Result<bool> {
    validate_directory(path.parent().expect("destination has parent"))?;
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.is_file() || is_link(&metadata) || metadata.permissions().readonly() {
                return Err(io::Error::other(
                    "destination must be an ordinary writable file; directories, links/reparse points and read-only files are refused",
                ));
            }
            Ok(true)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

// Canonicalize the existing prefix as well as comparing portable spellings.
// This catches aliases such as Windows 8.3 names of existing files/directories.
fn destination_key(path: &Path) -> io::Result<String> {
    let mut prefix = path;
    let mut suffix = Vec::new();
    loop {
        match fs::canonicalize(prefix) {
            Ok(mut canonical) => {
                for component in suffix.iter().rev() {
                    canonical.push(component);
                }
                return Ok(canonical
                    .to_string_lossy()
                    .replace('\\', "/")
                    .to_lowercase());
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if let Some(component) = prefix.file_name() {
                    suffix.push(component.to_os_string());
                }
                prefix = prefix
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or(Path::new("."));
            }
            Err(e) => return Err(e),
        }
    }
}

/// Record only directories created by this invocation. Rollback uses remove_dir
/// (empty only), never a recursive deletion of a user-selected directory.
fn create_directories(path: &Path, created: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() && !is_link(&metadata) => return Ok(()),
        Ok(_) => return Err(io::Error::other("parent is not an ordinary directory")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    if let Some(parent) = path.parent() {
        create_directories(parent, created)?;
    }
    fs::create_dir(path)?;
    created.push(path.to_path_buf());
    Ok(())
}

// Delete only transaction-owned filenames, never recursively sweep a directory.
// Keep the manifest until backup cleanup succeeds, so retained backups are named.
fn cleanup_stage(staging: &Path, count: usize) -> io::Result<()> {
    validate_directory(staging)?;
    let names = (0..count)
        .flat_map(|i| [format!("new-{i}"), format!("old-{i}")])
        .chain(std::iter::once("manifest.json".to_string()));
    for name in names {
        match fs::remove_file(staging.join(name)) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    fs::remove_dir(staging)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Stage(usize),
    Flush(usize),
    Backup(usize),
    Publish(usize),
    RemovePublished(usize),
    Restore(usize),
    Cleanup,
}

struct Entry {
    destination: PathBuf,
    staged: PathBuf,
    backup: PathBuf,
    existed: bool,
    backed_up: bool,
    published: bool,
}

pub(crate) fn write(plan: &Plan, root: &Path) -> Result<(), ExportError> {
    write_with(plan, root, |_| Ok(()))
}

pub(crate) fn write_reviewed(
    plan: &Plan,
    root: &Path,
    replacements: &[String],
) -> Result<(), ExportError> {
    write_transaction(plan, root, |_| Ok(()), None, Some(replacements))
}

/// Read-only advisory inspection, using the publisher's own path validators.
/// It acquires no lock and creates nothing. Publication still repeats all checks.
pub(crate) fn inspect(plan: &Plan, root: &Path) -> Result<(), ExportError> {
    if root.as_os_str().is_empty() {
        return Err(error(root, "empty output directory"));
    }
    let paths = validate_paths(plan)?;
    validate_directory(root).map_err(|e| error(root, e))?;
    if fs::symlink_metadata(root.join(LOCK)).is_ok() {
        return Err(error(
            root,
            "export transaction lock already exists; inspect the previous transaction",
        ));
    }
    let mut resolved: Vec<String> = Vec::new();
    for path in &paths {
        let destination = root.join(path);
        validate_destination(&destination).map_err(|e| error(&destination, e))?;
        let key = destination_key(&destination).map_err(|e| error(&destination, e))?;
        if resolved.iter().any(|p| {
            p == &key || p.starts_with(&format!("{key}/")) || key.starts_with(&format!("{p}/"))
        }) {
            return Err(error(&destination, "aliased output paths"));
        }
        resolved.push(key);
    }
    for (path, bytes) in paths.iter().zip(
        plan.files
            .iter()
            .map(|f| f.contents.as_bytes())
            .chain(plan.binaries.iter().map(|(_, b)| b.as_slice())),
    ) {
        if Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            serde_json::from_slice::<serde_json::Value>(bytes)
                .map_err(|e| error(&root.join(path), format!("invalid JSON: {e}")))?;
        }
    }
    Ok(())
}

// The private hook injects OS failures before individual operations in tests;
// production always calls real fs operations, with no simulated public mode.
fn write_with(
    plan: &Plan,
    root: &Path,
    before: impl FnMut(Operation) -> io::Result<()>,
) -> Result<(), ExportError> {
    write_transaction(plan, root, before, None, None)
}

pub(crate) fn write_generated(
    plan: &Plan,
    root: &Path,
    generate: impl FnMut(usize, &str) -> Result<Vec<u8>, String>,
    cancelled: impl Fn() -> bool,
) -> Result<(), ExportError> {
    write_generated_with(
        plan,
        root,
        |op| {
            if matches!(
                op,
                Operation::Stage(_)
                    | Operation::Flush(_)
                    | Operation::Publish(_)
                    | Operation::Backup(_)
            ) && cancelled()
            {
                Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "export cancelled",
                ))
            } else {
                Ok(())
            }
        },
        generate,
    )
}

fn write_generated_with(
    plan: &Plan,
    root: &Path,
    before: impl FnMut(Operation) -> io::Result<()>,
    mut generate: impl FnMut(usize, &str) -> Result<Vec<u8>, String>,
) -> Result<(), ExportError> {
    write_transaction(plan, root, before, Some(&mut generate), None)
}

type Generator<'a> = &'a mut dyn FnMut(usize, &str) -> Result<Vec<u8>, String>;

fn write_transaction(
    plan: &Plan,
    root: &Path,
    mut before: impl FnMut(Operation) -> io::Result<()>,
    mut generate: Option<Generator<'_>>,
    replacements: Option<&[String]>,
) -> Result<(), ExportError> {
    if root.as_os_str().is_empty() {
        return Err(error(root, "output directory is empty; nothing written"));
    }
    let paths = validate_paths(plan)?;
    let contents = plan
        .files
        .iter()
        .map(|f| f.contents.as_bytes())
        .chain(plan.binaries.iter().map(|(_, bytes)| bytes.as_slice()));
    for (path, bytes) in paths.iter().zip(contents) {
        if Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            serde_json::from_slice::<serde_json::Value>(bytes).map_err(|e| {
                error(
                    &root.join(path),
                    format!("invalid JSON: {e}; nothing written"),
                )
            })?;
        }
    }
    validate_directory(root)
        .map_err(|e| error(root, format!("validation: {e}; nothing written")))?;
    let existed: Vec<bool> = paths
        .iter()
        .map(|path| {
            let destination = root.join(path);
            validate_destination(&destination)
                .map_err(|e| error(&destination, format!("validation: {e}; nothing written")))
        })
        .collect::<Result<_, _>>()?;
    if let Some(approved) = replacements {
        let actual: Vec<_> = paths
            .iter()
            .zip(&existed)
            .filter(|(_, exists)| **exists)
            .map(|(p, _)| p.clone())
            .collect();
        if actual != approved {
            return Err(error(
                root,
                "replacement inventory changed since preflight; nothing written",
            ));
        }
    }
    if generate.is_some() && existed.iter().any(|exists| *exists) {
        return Err(error(
            root,
            "PNG sequence output already exists; choose a new folder. Nothing was replaced",
        ));
    }
    let mut resolved: Vec<String> = Vec::new();
    for (i, path) in paths.iter().enumerate() {
        let destination = root.join(path);
        let key = destination_key(&destination)
            .map_err(|e| error(&destination, format!("validation: {e}; nothing written")))?;
        if let Some(previous) = resolved.iter().position(|p| {
            p == &key || p.starts_with(&format!("{key}/")) || key.starts_with(&format!("{p}/"))
        }) {
            return Err(ExportError::DuplicatePath {
                path: paths[i].clone(),
                first: paths[previous].clone(),
                second: path.clone(),
            });
        }
        resolved.push(key);
    }
    if paths.is_empty() {
        return Ok(());
    }

    let mut directories = Vec::new();
    let mut stage: Option<PathBuf> = None;
    let lock_path = root.join(LOCK);
    let mut owns_lock = false;
    let mut entries: Vec<Entry> = Vec::new();
    let mut phase = "preparation";
    let mut active_path = root.to_path_buf();
    let attempt = (|| -> io::Result<()> {
        create_directories(root, &mut directories)?;
        // Closed immediately: Windows rename/delete does not rely on Unix's
        // ability to manipulate a file while it is open.
        let lock = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
            .map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!(
                        "export lock {}: {e}; another export or stale lock may exist",
                        lock_path.display()
                    ),
                )
            })?;
        owns_lock = true;
        drop(lock);
        for _ in 0..100 {
            let candidate = root.join(format!(
                ".ankhimate-export-{}-{}",
                std::process::id(),
                NEXT_STAGE.fetch_add(1, Ordering::Relaxed)
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
            .ok_or_else(|| io::Error::other("cannot allocate a unique export staging directory"))?;
        // Backups use stable indices; persist the destination mapping before any
        // replacement so retained files remain identifiable without the Plan.
        let manifest = serde_json::json!({
            "transaction_version": 1,
            "files": paths.iter().zip(&existed).enumerate().map(|(i, (path, existed))| serde_json::json!({
                "path": path, "existed": existed,
                "staged": format!("new-{i}"), "backup": format!("old-{i}")
            })).collect::<Vec<_>>()
        });
        let mut manifest_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(staging.join("manifest.json"))?;
        serde_json::to_writer(&mut manifest_file, &manifest)?;
        manifest_file.sync_all()?;
        drop(manifest_file);
        let bytes = plan
            .files
            .iter()
            .map(|f| f.contents.as_bytes())
            .chain(plan.binaries.iter().map(|(_, bytes)| bytes.as_slice()));
        for (i, ((path, existed), bytes)) in paths.iter().zip(existed).zip(bytes).enumerate() {
            active_path = root.join(path);
            let staged = staging.join(format!("new-{i}"));
            before(Operation::Stage(i))?;
            let generated = if i >= plan.files.len() {
                generate
                    .as_mut()
                    .map(|generate| generate(i - plan.files.len(), path))
                    .transpose()
                    .map_err(io::Error::other)?
            } else {
                None
            };
            let bytes = generated.as_deref().unwrap_or(bytes);
            if generate.is_some() && bytes.is_empty() {
                return Err(io::Error::other("generated output contains no image data"));
            }
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&staged)?;
            file.write_all(bytes)?;
            before(Operation::Flush(i))?;
            file.sync_all()?;
            drop(file);
            entries.push(Entry {
                destination: active_path.clone(),
                staged,
                backup: staging.join(format!("old-{i}")),
                existed,
                backed_up: false,
                published: false,
            });
        }
        // Parent creation is still preparation: no destination is replaced.
        for entry in &entries {
            active_path = entry.destination.clone();
            create_directories(entry.destination.parent().unwrap(), &mut directories)?;
        }
        phase = "publication";
        for (i, entry) in entries.iter_mut().enumerate() {
            active_path = entry.destination.clone();
            if validate_destination(&entry.destination)? != entry.existed {
                return Err(io::Error::other(
                    "destination appeared/disappeared during preparation",
                ));
            }
            if entry.existed {
                before(Operation::Backup(i))?;
                fs::rename(&entry.destination, &entry.backup)?;
                entry.backed_up = true;
            }
            before(Operation::Publish(i))?;
            fs::rename(&entry.staged, &entry.destination)?;
            entry.published = true;
        }
        Ok(())
    })();

    if let Err(failure) = attempt {
        let mut rollback_errors = Vec::new();
        for (i, entry) in entries.iter_mut().enumerate().rev() {
            if entry.published {
                match before(Operation::RemovePublished(i))
                    .and_then(|_| fs::remove_file(&entry.destination))
                {
                    Ok(()) => entry.published = false,
                    Err(e) => {
                        rollback_errors.push(format!("remove {}: {e}", entry.destination.display()))
                    }
                }
            }
            if entry.backed_up {
                // Do not replace an unexpected file on Unix either. A user or
                // another application may have created it while we rolled back.
                let restore = (|| {
                    before(Operation::Restore(i))?;
                    match fs::symlink_metadata(&entry.destination) {
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            fs::rename(&entry.backup, &entry.destination)
                        }
                        Ok(_) => Err(io::Error::other("destination occupied; backup retained")),
                        Err(e) => Err(e),
                    }
                })();
                if let Err(e) = restore {
                    rollback_errors.push(format!("restore {}: {e}", entry.destination.display()));
                }
            }
        }
        let mut reason = format!("{phase} failed: {failure}");
        if rollback_errors.is_empty() {
            reason.push_str("; rollback completed, previous output files preserved");
            if let Some(staging) = &stage
                && let Err(e) = cleanup_stage(staging, paths.len())
            {
                rollback_errors.push(format!("clean staging {}: {e}", staging.display()));
            }
        } else if let Some(staging) = &stage {
            reason.push_str(&format!(
                "; ROLLBACK INCOMPLETE; backups retained in {}",
                staging.display()
            ));
        }
        if owns_lock && let Err(e) = fs::remove_file(&lock_path) {
            rollback_errors.push(format!("remove export lock {}: {e}", lock_path.display()));
        }
        for directory in directories.iter().rev() {
            if let Err(e) = fs::remove_dir(directory)
                && e.kind() != io::ErrorKind::DirectoryNotEmpty
            {
                rollback_errors.push(format!("clean directory {}: {e}", directory.display()));
            }
        }
        if !rollback_errors.is_empty() {
            reason.push_str(&format!(
                "; recovery/cleanup errors: {}",
                rollback_errors.join("; ")
            ));
        }
        return Err(error(&active_path, reason));
    }

    // The publication has succeeded. Cleanup failure must not pretend rollback
    // occurred: all new outputs are valid, and remaining backups can be inspected.
    let mut cleanup_errors = Vec::new();
    let staging = stage.as_ref().unwrap();
    if let Err(e) = before(Operation::Cleanup).and_then(|_| cleanup_stage(staging, paths.len())) {
        cleanup_errors.push(format!("{}: {e}", staging.display()));
    }
    if let Err(e) = fs::remove_file(&lock_path) {
        cleanup_errors.push(format!("{}: {e}", lock_path.display()));
    }
    if !cleanup_errors.is_empty() {
        return Err(error(
            root,
            format!(
                "all outputs published, but transaction cleanup failed: {}; inspect retained transaction files before retrying",
                cleanup_errors.join("; ")
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::RenderedFile;

    #[test]
    fn generated_files_are_staged_and_cancelled_publication_is_rolled_back() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("keep.txt"), b"previous export").unwrap();
        let plan = Plan {
            binaries: vec![
                ("frame_000000.png".into(), vec![]),
                ("frame_000001.png".into(), vec![]),
            ],
            ..Default::default()
        };
        let result = write_generated_with(
            &plan,
            dir.path(),
            |op| {
                if op == Operation::Publish(1) {
                    Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "export cancelled",
                    ))
                } else {
                    Ok(())
                }
            },
            |_, _| Ok(vec![1, 2, 3]),
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("rollback completed")
        );
        assert_eq!(
            fs::read(dir.path().join("keep.txt")).unwrap(),
            b"previous export"
        );
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn generated_files_never_replace_existing_outputs_and_generation_failure_preserves_them() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("frame_000000.png"), b"valid previous PNG").unwrap();
        let plan = Plan {
            binaries: vec![("frame_000000.png".into(), vec![])],
            ..Default::default()
        };
        assert!(
            write_generated_with(
                &plan,
                dir.path(),
                |_| Ok(()),
                |_, _| panic!("collision must fail before generation")
            )
            .is_err()
        );
        let plan = Plan {
            binaries: vec![("new.png".into(), vec![]), ("another.png".into(), vec![])],
            ..Default::default()
        };
        assert!(
            write_generated_with(
                &plan,
                dir.path(),
                |_| Ok(()),
                |index, _| {
                    if index == 0 {
                        Ok(vec![1])
                    } else {
                        Err("render failed".into())
                    }
                }
            )
            .is_err()
        );
        assert_eq!(
            fs::read(dir.path().join("frame_000000.png")).unwrap(),
            b"valid previous PNG"
        );
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    fn fixture() -> (tempfile::TempDir, Plan) {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("skeleton.json"), "old skeleton").unwrap();
        fs::write(dir.path().join("atlas.png"), b"old texture").unwrap();
        fs::write(dir.path().join("orphan.txt"), "user file").unwrap();
        let plan = Plan {
            files: vec![
                RenderedFile {
                    path: "skeleton.json".into(),
                    contents: "\"new skeleton\"".into(),
                },
                RenderedFile {
                    path: "animations/walk.json".into(),
                    contents: "\"new animation\"".into(),
                },
            ],
            binaries: vec![("atlas.png".into(), b"new texture".to_vec())],
        };
        (dir, plan)
    }

    fn assert_restored(dir: &Path) {
        assert_eq!(
            fs::read(dir.join("skeleton.json")).unwrap(),
            b"old skeleton"
        );
        assert_eq!(fs::read(dir.join("atlas.png")).unwrap(), b"old texture");
        assert_eq!(fs::read(dir.join("orphan.txt")).unwrap(), b"user file");
        assert_eq!(
            fs::read_dir(dir).unwrap().count(),
            3,
            "new outputs, parents, locks and staging must be removed"
        );
    }

    #[test]
    fn failure_at_every_stage_backup_and_publication_restores_original_bytes() {
        for failure in [
            Operation::Stage(0),
            Operation::Stage(1),
            Operation::Stage(2),
            Operation::Flush(0),
            Operation::Flush(1),
            Operation::Flush(2),
            Operation::Backup(0),
            Operation::Backup(2),
            Operation::Publish(0),
            Operation::Publish(1),
            Operation::Publish(2),
        ] {
            let (dir, plan) = fixture();
            let err = write_with(&plan, dir.path(), |op| {
                if op == failure {
                    Err(io::Error::other("injected filesystem failure"))
                } else {
                    Ok(())
                }
            })
            .unwrap_err();
            assert!(
                err.to_string().contains("rollback completed"),
                "{failure:?}: {err}"
            );
            assert_restored(dir.path());
        }
    }

    #[test]
    fn failed_restore_keeps_original_bytes_and_reports_backup_location() {
        let (dir, plan) = fixture();
        let err = write_with(&plan, dir.path(), |op| {
            if matches!(op, Operation::Publish(2) | Operation::Restore(0)) {
                Err(io::Error::other("injected restore failure"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        let message = err.to_string();
        assert!(message.contains("ROLLBACK INCOMPLETE"), "{message}");
        let staging = fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".ankhimate-export-")
            })
            .unwrap();
        assert!(message.contains(&staging.display().to_string()));
        assert_eq!(fs::read(staging.join("old-0")).unwrap(), b"old skeleton");
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(staging.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["files"][0]["path"], "skeleton.json");
        assert_eq!(manifest["files"][0]["backup"], "old-0");
        assert_eq!(manifest["files"][1]["existed"], false);
        assert_eq!(
            fs::read(dir.path().join("atlas.png")).unwrap(),
            b"old texture"
        );
        assert!(!dir.path().join("skeleton.json").exists());
        assert!(!dir.path().join("animations").exists());
    }

    #[test]
    fn failed_removal_does_not_overwrite_new_file_or_discard_its_backup() {
        let (dir, plan) = fixture();
        let err = write_with(&plan, dir.path(), |op| {
            if matches!(op, Operation::Publish(2) | Operation::RemovePublished(0)) {
                Err(io::Error::other("injected remove failure"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(err.to_string().contains("ROLLBACK INCOMPLETE"));
        assert_eq!(
            fs::read(dir.path().join("skeleton.json")).unwrap(),
            b"\"new skeleton\""
        );
        let staging = fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".ankhimate-export-")
            })
            .unwrap();
        assert_eq!(fs::read(staging.join("old-0")).unwrap(), b"old skeleton");
    }

    #[test]
    fn cleanup_failure_explicitly_reports_that_all_outputs_were_published() {
        let (dir, plan) = fixture();
        let err = write_with(&plan, dir.path(), |op| {
            if op == Operation::Cleanup {
                Err(io::Error::other("injected cleanup failure"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(err.to_string().contains("all outputs published"));
        assert_eq!(
            fs::read(dir.path().join("skeleton.json")).unwrap(),
            b"\"new skeleton\""
        );
        assert_eq!(
            fs::read(dir.path().join("animations/walk.json")).unwrap(),
            b"\"new animation\""
        );
        assert_eq!(
            fs::read(dir.path().join("atlas.png")).unwrap(),
            b"new texture"
        );
        assert!(!dir.path().join(LOCK).exists());
    }

    #[test]
    fn occupied_lock_is_preserved_and_export_changes_nothing() {
        let (dir, plan) = fixture();
        fs::write(dir.path().join(LOCK), "owned elsewhere").unwrap();
        let err = write(&plan, dir.path()).unwrap_err();
        assert!(err.to_string().contains("export lock"));
        assert_eq!(fs::read(dir.path().join(LOCK)).unwrap(), b"owned elsewhere");
        fs::remove_file(dir.path().join(LOCK)).unwrap();
        assert_restored(dir.path());
    }

    #[test]
    fn failed_preparation_removes_only_directories_it_created() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("new/nested/export");
        let plan = Plan {
            files: vec![RenderedFile {
                path: "a.txt".into(),
                contents: "new".into(),
            }],
            binaries: vec![],
        };
        write_with(&plan, &root, |_| Err(io::Error::other("staging failed"))).unwrap_err();
        assert!(!dir.path().join("new").exists());
        assert!(dir.path().exists());
    }

    #[cfg(windows)]
    #[test]
    fn a_real_windows_sharing_violation_rolls_back_previous_publications() {
        use std::os::windows::fs::OpenOptionsExt;
        let (dir, plan) = fixture();
        // Allow read/write but no FILE_SHARE_DELETE: writing bytes directly would
        // succeed, whereas replacing/renaming this destination must fail.
        let held = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(3)
            .open(dir.path().join("atlas.png"))
            .unwrap();
        let err = write(&plan, dir.path()).unwrap_err();
        assert!(err.to_string().contains("publication failed"), "{err}");
        drop(held);
        assert_restored(dir.path());
    }
}
