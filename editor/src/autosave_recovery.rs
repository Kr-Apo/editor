//! Recovery discovery and non-destructive, version-specific Ignore receipts.
use super::{Recovery, path_for};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const RECEIPT: &[u8] = b"Ankhimate ignored recovery v1\n";

#[derive(Clone)]
pub struct RecoveryStore {
    pub(crate) untitled: PathBuf,
    legacy: PathBuf,
    decisions: PathBuf,
}

#[derive(Default)]
pub struct Scan {
    /// Best candidate first; valid payloads precede corrupt ordinary sidecars.
    pub copies: Vec<Recovery>,
    pub warnings: Vec<String>,
}

impl Default for RecoveryStore {
    fn default() -> Self {
        #[cfg(test)]
        {
            // Never read or write the maintainer's recovery/config directory.
            static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
            Self::isolated(
                ROOT.get_or_init(|| tempfile::tempdir().unwrap().keep())
                    .clone(),
            )
        }
        #[cfg(not(test))]
        {
            let untitled = std::env::temp_dir().join("ankhimate-recovery");
            let decisions = directories::ProjectDirs::from("", "", "ankhimate")
                .map(|d| d.config_dir().join("recovery-decisions-v1"))
                .unwrap_or_else(|| untitled.join("decisions-v1"));
            Self {
                untitled,
                legacy: path_for(None).unwrap(),
                decisions,
            }
        }
    }
}

impl RecoveryStore {
    #[cfg(test)]
    pub(crate) fn isolated(root: PathBuf) -> Self {
        Self {
            untitled: root.join("copies"),
            legacy: root.join("untitled.ankh.autosave"),
            decisions: root.join("decisions"),
        }
    }

    pub fn scan(&self, project: Option<&Path>, primary_valid: bool, include_ignored: bool) -> Scan {
        let mut result = Scan::default();
        let primary_time = project
            .filter(|_| primary_valid)
            .and_then(|p| std::fs::metadata(p).ok()?.modified().ok());
        let mut candidates = Vec::new();
        if let Some(project) = project {
            if let Some(path) = path_for(Some(project)) {
                candidates.push((path, None));
            }
            candidates.extend(
                ankhimate_formats::container::recoverable_writes(project)
                    .into_iter()
                    .map(|p| (p, Some(project.to_path_buf()))),
            );
        } else {
            candidates.push((self.legacy.clone(), None));
            match std::fs::read_dir(&self.untitled) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                let p = entry.path();
                                if p.file_name().is_some_and(|n| {
                                    let n = n.to_string_lossy();
                                    n.starts_with("untitled-") && n.ends_with(".ankh.autosave")
                                }) {
                                    candidates.push((p, None));
                                }
                            }
                            Err(e) => result.warnings.push(format!("Recovery discovery: {e}")),
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => result.warnings.push(format!("Recovery discovery: {e}")),
            }
        }
        let mut ranked = Vec::new();
        for (path, origin) in candidates {
            let metadata = match regular_metadata(&path) {
                Ok(m) => m,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => {
                    result
                        .warnings
                        .push(format!("Recovery {}: {e}", path.display()));
                    continue;
                }
            };
            let modified = match metadata.modified() {
                Ok(t) => t,
                Err(e) => {
                    result.warnings.push(format!("Recovery timestamp: {e}"));
                    continue;
                }
            };
            if primary_time.is_some_and(|t| t >= modified) {
                continue;
            }
            let version = match fingerprint(&path) {
                Ok(v) => v,
                Err(e) => {
                    result
                        .warnings
                        .push(format!("Recovery {}: {e}", path.display()));
                    continue;
                }
            };
            if !include_ignored {
                match self.is_ignored(&version) {
                    Ok(true) => continue,
                    Ok(false) => {}
                    Err(e) => result
                        .warnings
                        .push(format!("Ignore decision unavailable: {e}")),
                }
            }
            let valid =
                ankhimate_formats::load_with_assets(&path, origin.as_deref().unwrap_or(&path))
                    .is_ok();
            if origin.is_some() && !valid {
                continue;
            }
            ranked.push((
                valid,
                modified,
                Recovery {
                    autosave: path,
                    project: project.map(Path::to_path_buf),
                    asset_origin: origin,
                    version,
                },
            ));
        }
        ranked.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| b.1.cmp(&a.1))
                .then_with(|| a.2.autosave.cmp(&b.2.autosave))
        });
        result.copies = ranked.into_iter().map(|(_, _, r)| r).collect();
        result
    }

    fn is_ignored(&self, version: &str) -> std::io::Result<bool> {
        match std::fs::symlink_metadata(&self.decisions) {
            Ok(_) => regular_directory(&self.decisions)?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(e) => return Err(e),
        }
        let path = self.decisions.join(version);
        match regular_metadata(&path) {
            Ok(m) if m.len() == RECEIPT.len() as u64 => Ok(std::fs::read(path)? == RECEIPT),
            Ok(_) => Ok(false), // Interrupted/corrupt receipts never suppress a copy.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn ignore(&self, recovery: &Recovery) -> std::io::Result<()> {
        recovery.ensure_unchanged()?;
        std::fs::create_dir_all(&self.decisions)?;
        // Reject redirected receipt roots. Never replace any existing marker.
        regular_directory(&self.decisions)?;
        let path = self.decisions.join(&recovery.version);
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => {
                file.write_all(RECEIPT)?;
                file.sync_all()
            }
            Err(e)
                if e.kind() == std::io::ErrorKind::AlreadyExists
                    && self.is_ignored(&recovery.version)? =>
            {
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Recovery {
    pub fn ensure_unchanged(&self) -> std::io::Result<()> {
        if fingerprint(&self.autosave)? != self.version {
            return Err(std::io::Error::other(
                "Recovery changed since it was offered; review copies again",
            ));
        }
        Ok(())
    }
}

fn not_redirected(path: &Path) -> std::io::Result<std::fs::Metadata> {
    let m = std::fs::symlink_metadata(path)?;
    let redirected = m.file_type().is_symlink();
    #[cfg(windows)]
    let redirected = {
        use std::os::windows::fs::MetadataExt;
        redirected || m.file_attributes() & 0x400 != 0
    };
    if redirected {
        return Err(std::io::Error::other("Redirected recovery path refused"));
    }
    Ok(m)
}

fn regular_directory(path: &Path) -> std::io::Result<()> {
    if !not_redirected(path)?.is_dir() {
        return Err(std::io::Error::other("Decision root is not a directory"));
    }
    Ok(())
}

fn regular_metadata(path: &Path) -> std::io::Result<std::fs::Metadata> {
    let m = not_redirected(path)?;
    if !m.is_file() {
        return Err(std::io::Error::other("Recovery path is not a regular file"));
    }
    Ok(m)
}

fn fingerprint(path: &Path) -> std::io::Result<String> {
    let before = regular_metadata(path)?;
    let modified = before.modified()?;
    let canonical = std::fs::canonicalize(path)?;
    let mut hash = Sha256::new();
    hash.update(b"Ankhimate recovery state v1\0");
    hash.update(canonical.as_os_str().as_encoded_bytes());
    hash.update([0]);
    hash.update(format!("{modified:?}:{}:", before.len()).as_bytes());
    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    let after = regular_metadata(path)?;
    if after.len() != before.len() || after.modified()? != modified {
        return Err(std::io::Error::other("Recovery changed while being read"));
    }
    Ok(format!("{:x}", hash.finalize()))
}

#[cfg(test)]
#[path = "autosave_recovery_tests.rs"]
mod tests;
