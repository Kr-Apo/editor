//! Single new-file publication for an external encoder. Existing exporters are unchanged.
use crate::staged;
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);

/// Advisory only; ordinary directories, portable leaf name, no existing destination.
pub fn inspect(destination: &Path) -> Result<PathBuf, String> {
    if !destination.is_absolute() {
        return Err("Output file must be absolute".into());
    }
    let leaf = destination
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Invalid output filename")?;
    crate::batch::folder_name(leaf)?;
    let parent = destination.parent().ok_or("Missing output parent")?;
    staged::validate_directory(parent).map_err(|e| e.to_string())?;
    if !fs::metadata(parent).map_err(|e| e.to_string())?.is_dir() {
        return Err("Output parent must exist".into());
    }
    let result = fs::canonicalize(parent)
        .map_err(|e| e.to_string())?
        .join(leaf);
    match fs::symlink_metadata(&result) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(result),
        Ok(_) => Err(format!(
            "Destination already exists; no replacement: {}",
            result.display()
        )),
        Err(e) => Err(e.to_string()),
    }
}

/// Owned, exclusive sibling directory; never sweep other files or backups.
pub struct PreparedFile {
    root: PathBuf,
    pub path: PathBuf,
    destination: PathBuf,
    owned: Vec<PathBuf>,
}
impl PreparedFile {
    pub fn new(destination: &Path) -> Result<Self, String> {
        Self::with_auxiliary(destination, "encoded.mp4", &[])
    }
    /// Explicit allowlist of owned temporary files; never accept arbitrary descendants.
    pub fn with_auxiliary(
        destination: &Path,
        filename: &str,
        auxiliary: &[&str],
    ) -> Result<Self, String> {
        let destination = inspect(destination)?;
        let names: Vec<_> = std::iter::once(filename)
            .chain(auxiliary.iter().copied())
            .collect();
        let mut folded = std::collections::BTreeSet::new();
        for name in &names {
            crate::batch::folder_name(name)?;
            if !folded.insert(name.to_lowercase()) {
                return Err("Duplicate staging filename".into());
            }
        }
        for _ in 0..128 {
            let root = destination.parent().unwrap().join(format!(
                ".ankhimate-video-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    return Ok(Self {
                        path: root.join(filename),
                        owned: names.iter().map(|name| root.join(name)).collect(),
                        root,
                        destination,
                    });
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.to_string()),
            }
        }
        Err("Cannot allocate an exclusive video staging directory".into())
    }
    pub fn auxiliary(&self, name: &str) -> Option<&Path> {
        self.owned
            .iter()
            .find(|p| p.file_name().and_then(|s| s.to_str()) == Some(name))
            .map(|p| p.as_path())
    }
    /// Atomically create a new name using a hard link. Never replaces a race winner.
    /// Filesystems without hard-link support fail safely; no overwrite fallback.
    pub fn publish(&self) -> Result<(), String> {
        let target = inspect(&self.destination)?;
        staged::validate_directory(&self.root).map_err(|e| e.to_string())?;
        let meta = fs::symlink_metadata(&self.path).map_err(|e| e.to_string())?;
        if !meta.is_file() || staged::is_link(&meta) || meta.len() == 0 {
            return Err("Invalid staged video file".into());
        }
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|e| format!("Cannot flush staged video: {e}"))?;
        fs::hard_link(&self.path, target).map_err(|e| {
            format!("Cannot publish video without replacement (hard-link support required): {e}")
        })
    }
    /// Explicit cleanup so failures can be reported. Unknown entries are retained.
    pub fn cleanup(&self) -> Result<(), String> {
        let attempt = (|| -> io::Result<()> {
            staged::validate_directory(&self.root)?;
            for entry in fs::read_dir(&self.root)? {
                let entry = entry?;
                let meta = fs::symlink_metadata(entry.path())?;
                if !self.owned.contains(&entry.path()) || !meta.is_file() || staged::is_link(&meta)
                {
                    return Err(io::Error::other(
                        "unexpected entry, link or backup; retaining staging directory",
                    ));
                }
            }
            for path in &self.owned {
                if path.try_exists()? {
                    fs::remove_file(path)?;
                }
            }
            fs::remove_dir(&self.root)
        })();
        attempt.map_err(|e| format!("Temporary output retained at {}: {e}", self.root.display()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_and_racing_destinations_preserve_bytes() {
        let d = tempfile::tempdir().unwrap();
        let output = d.path().join("film.mp4");
        let stage = PreparedFile::new(&output).unwrap();
        fs::write(&stage.path, b"new video").unwrap();
        fs::write(&output, b"old video").unwrap();
        assert!(stage.publish().is_err());
        stage.cleanup().unwrap();
        assert_eq!(fs::read(output).unwrap(), b"old video");
    }
    #[test]
    fn clean_only_owned_file_and_report_unknown_backup() {
        let d = tempfile::tempdir().unwrap();
        let stage = PreparedFile::new(&d.path().join("film.mp4")).unwrap();
        fs::write(stage.root.join("backup"), b"keep").unwrap();
        let error = stage.cleanup().unwrap_err();
        assert!(error.contains("retained"));
        assert_eq!(fs::read(stage.root.join("backup")).unwrap(), b"keep");
    }
    #[test]
    fn portable_paths_and_missing_parents_refuse() {
        let d = tempfile::tempdir().unwrap();
        for name in ["CON.mp4", "bad?.mp4", "trailing .mp4 ", "missing/out.mp4"] {
            assert!(inspect(&d.path().join(name)).is_err());
        }
        assert!(inspect(Path::new("relative.mp4")).is_err());
    }
    #[test]
    fn publish_new_file_then_remove_only_temporary_link() {
        let d = tempfile::tempdir().unwrap();
        let output = d.path().join("été film.mp4");
        let stage = PreparedFile::new(&output).unwrap();
        fs::write(&stage.path, b"video").unwrap();
        stage.publish().unwrap();
        stage.cleanup().unwrap();
        assert_eq!(fs::read(output).unwrap(), b"video");
    }
}
#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::os::windows::fs::OpenOptionsExt;
    #[test]
    fn locked_stage_failure_retains_owned_file_and_preserves_unrelated_backup() {
        let d = tempfile::tempdir().unwrap();
        let output = d.path().join("film.mp4");
        let backup = d.path().join("user.backup");
        fs::write(&backup, b"precious").unwrap();
        let stage = PreparedFile::new(&output).unwrap();
        fs::write(&stage.path, b"encoded").unwrap();
        let lock = fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&stage.path)
            .unwrap();
        assert!(stage.publish().is_err());
        assert!(stage.cleanup().unwrap_err().contains("retained"));
        assert!(!output.exists());
        assert_eq!(fs::read(&backup).unwrap(), b"precious");
        drop(lock);
        stage.cleanup().unwrap();
    }
}
#[cfg(test)]
mod auxiliary_tests {
    use super::*;
    #[test]
    fn gif_publish_cleans_declared_palette_and_only_its_owned_files() {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("film.gif");
        let stage = PreparedFile::with_auxiliary(&out, "encoded.gif", &["palette.png"]).unwrap();
        fs::write(&stage.path, b"GIF result").unwrap();
        fs::write(stage.auxiliary("palette.png").unwrap(), b"palette").unwrap();
        stage.publish().unwrap();
        stage.cleanup().unwrap();
        assert_eq!(fs::read(&out).unwrap(), b"GIF result");
        assert_eq!(fs::read_dir(d.path()).unwrap().count(), 1);
    }
    #[test]
    fn unknown_backup_prevents_deleting_either_declared_file() {
        let d = tempfile::tempdir().unwrap();
        let stage = PreparedFile::with_auxiliary(
            &d.path().join("film.gif"),
            "encoded.gif",
            &["palette.png"],
        )
        .unwrap();
        fs::write(&stage.path, b"GIF").unwrap();
        let palette = stage.auxiliary("palette.png").unwrap();
        fs::write(palette, b"palette").unwrap();
        fs::write(stage.root.join("backup"), b"old backup").unwrap();
        assert!(stage.cleanup().unwrap_err().contains("retained"));
        assert!(stage.path.exists());
        assert!(palette.exists());
        assert_eq!(fs::read(stage.root.join("backup")).unwrap(), b"old backup");
    }
    #[test]
    fn auxiliary_paths_and_case_aliases_refuse_before_staging() {
        let d = tempfile::tempdir().unwrap();
        for auxiliary in [vec!["../palette.png"], vec!["Encoded.gif"], vec!["NUL.png"]] {
            assert!(
                PreparedFile::with_auxiliary(&d.path().join("film.gif"), "encoded.gif", &auxiliary)
                    .is_err()
            );
        }
        assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
    }
}
