//! Publish a prepared batch into a new directory; individual writers stay unchanged.
use crate::{
    run::{ExportError, Plan},
    staged,
};
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn error(path: &Path, reason: impl ToString) -> ExportError {
    ExportError::Io {
        path: path.display().to_string(),
        reason: reason.to_string(),
    }
}
/// One portable folder component, never a path or transaction name.
pub fn folder_name(name: &str) -> Result<(), String> {
    if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
        return Err("Choose one new folder name, not a path".into());
    }
    staged::validate_path(name)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
/// Advisory check only: existing directories (even empty), files and reparse points refuse.
pub fn inspect(plan: &Plan, destination: &Path) -> Result<PathBuf, ExportError> {
    if !cfg!(any(windows, target_os = "linux")) {
        return Err(error(
            destination,
            "safe no-replace batch publication is not supported on this platform",
        ));
    }
    staged::validate_paths(plan)?;
    if !destination.is_absolute() {
        return Err(error(destination, "batch destination must be absolute"));
    }
    let leaf = destination
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| error(destination, "invalid batch folder name"))?;
    folder_name(leaf).map_err(|e| error(destination, e))?;
    let parent = destination
        .parent()
        .ok_or_else(|| error(destination, "missing parent directory"))?;
    staged::validate_directory(parent).map_err(|e| error(parent, e))?;
    let metadata = fs::symlink_metadata(parent).map_err(|e| error(parent, e))?;
    if !metadata.is_dir() || staged::is_link(&metadata) {
        return Err(error(
            parent,
            "batch parent must be an existing ordinary directory",
        ));
    }
    let path = fs::canonicalize(parent)
        .map_err(|e| error(parent, e))?
        .join(leaf);
    match fs::symlink_metadata(&path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Ok(_) => {
            return Err(error(
                &path,
                "batch destination already exists; no replacement is permitted",
            ));
        }
        Err(e) => return Err(error(&path, e)),
    }
    Ok(path)
}
/// All planned files must be ready inside an exclusively created sibling before rename.
/// Failure cleans only this owned tree; residual child transactions/backups are retained.
pub fn write_new_directory(
    plan: &Plan,
    destination: &Path,
    prepare: impl FnOnce(&Path) -> Result<(), String>,
    cancelled: impl Fn() -> bool,
) -> Result<(), ExportError> {
    let destination = inspect(plan, destination)?;
    let parent = destination.parent().unwrap();
    if cancelled() {
        return Err(error(&destination, "batch cancelled before preparation"));
    }
    let mut stage = None;
    for _ in 0..128 {
        let candidate = parent.join(format!(
            ".ankhimate-batch-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match fs::create_dir(&candidate) {
            Ok(()) => {
                stage = Some(candidate);
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(error(&candidate, e)),
        }
    }
    let stage = stage.ok_or_else(|| {
        error(
            parent,
            "could not allocate a unique batch staging directory",
        )
    })?;
    let attempt: Result<(), String> = (|| {
        prepare(&stage)?;
        if cancelled() {
            return Err("batch cancelled before publication".into());
        }
        let (files, _) = inventory(&stage)?;
        if files
            .iter()
            .any(|p| fs::metadata(p).map_or(true, |m| m.len() == 0))
        {
            return Err("empty or unreadable prepared output file".into());
        }
        let actual: BTreeSet<_> = files
            .iter()
            .map(|p| {
                p.strip_prefix(&stage)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        let expected: BTreeSet<_> = staged::validate_paths(plan)
            .map_err(|e| e.to_string())?
            .into_iter()
            .collect();
        if actual != expected {
            return Err("prepared batch does not match the complete planned file set".into());
        }
        // Recheck after all rendering, immediately before the same-filesystem rename.
        inspect(plan, &destination).map_err(|e| e.to_string())?;
        if cancelled() {
            return Err("batch cancelled before publication".into());
        }
        rename_new(&stage, &destination)
            .map_err(|e| format!("batch directory publication failed: {e}"))?;
        Ok(())
    })();
    if let Err(reason) = attempt {
        let cleanup = cleanup_owned(&stage, parent);
        let suffix = match cleanup {
            Ok(()) => "temporary batch cleaned; nothing published".to_string(),
            Err(e) => format!(
                "temporary batch retained at {}: {e}; inspect recovery data before removal",
                stage.display()
            ),
        };
        return Err(error(&destination, format!("{reason}; {suffix}")));
    }
    Ok(())
}
fn inventory(root: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), String> {
    fn walk(root: &Path, files: &mut Vec<PathBuf>, dirs: &mut Vec<PathBuf>) -> Result<(), String> {
        let m = fs::symlink_metadata(root).map_err(|e| e.to_string())?;
        if !m.is_dir() || staged::is_link(&m) {
            return Err("unexpected link/reparse point or directory replacement; retained".into());
        }
        dirs.push(root.to_path_buf());
        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            let m = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
            if path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_lowercase()
                .starts_with(".ankhimate-export")
            {
                return Err(
                    "child export transaction/backups remain; retained for recovery".into(),
                );
            }
            if staged::is_link(&m) {
                return Err("unexpected link/reparse point; retained".into());
            }
            if m.is_dir() {
                walk(&path, files, dirs)?;
            } else if m.is_file() {
                files.push(path);
            } else {
                return Err("unexpected file type; retained".into());
            }
        }
        Ok(())
    }
    let mut files = vec![];
    let mut dirs = vec![];
    walk(root, &mut files, &mut dirs)?;
    Ok((files, dirs))
}
fn cleanup_owned(stage: &Path, parent: &Path) -> Result<(), String> {
    if stage.parent() != Some(parent)
        || !stage
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with(".ankhimate-batch-")
    {
        return Err("refusing cleanup outside owned sibling directory".into());
    }
    let (files, dirs) = inventory(stage)?;
    for path in files {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    for path in dirs.into_iter().rev() {
        fs::remove_dir(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// std::fs::rename may replace an empty directory. Never fall back to replacement.
#[cfg(windows)]
fn rename_new(source: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "MoveFileExW"]
        fn move_file_ex(source: *const u16, destination: *const u16, flags: u32) -> i32;
    }
    let wide = |p: &Path| -> io::Result<Vec<u16>> {
        let mut units: Vec<_> = p.as_os_str().encode_wide().collect();
        if units.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path contains NUL",
            ));
        }
        units.push(0);
        Ok(units)
    };
    let source = wide(source)?;
    let destination = wide(destination)?;
    // SAFETY: owned, NUL-terminated UTF-16 buffers outlive the synchronous call.
    // Flags 0 prohibit replacement and cross-volume copy. Canonical sibling paths.
    if unsafe { move_file_ex(source.as_ptr(), destination.as_ptr(), 0) } == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[cfg(target_os = "linux")]
fn rename_new(source: &Path, destination: &Path) -> io::Result<()> {
    use std::{ffi::CString, os::unix::ffi::OsStrExt};
    unsafe extern "C" {
        fn renameat2(
            olddirfd: i32,
            oldpath: *const std::ffi::c_char,
            newdirfd: i32,
            newpath: *const std::ffi::c_char,
            flags: u32,
        ) -> i32;
    }
    let source = CString::new(source.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let destination = CString::new(destination.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    // SAFETY: valid owned C strings; AT_FDCWD (-100) and RENAME_NOREPLACE (1).
    // An unavailable syscall/filesystem feature fails; no replacing fallback.
    if unsafe { renameat2(-100, source.as_ptr(), -100, destination.as_ptr(), 1) } != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[cfg(not(any(windows, target_os = "linux")))]
fn rename_new(_source: &Path, _destination: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "no-replace batch publication unavailable",
    ))
}
