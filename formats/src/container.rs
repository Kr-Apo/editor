//! Ankh v1 binary envelope and external asset storage.

use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

pub const MAGIC: &[u8; 4] = b"ANKH";
pub const VERSION: u16 = 1;
const CODEC_MESSAGEPACK: u8 = 1;
const FLAG_DEFLATE: u8 = 1;
const HEADER_LEN: usize = 16;

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not an Ankh v1 binary file")]
    BadMagic,
    #[error("unsupported Ankh version {0}")]
    UnsupportedVersion(u16),
    #[error("unsupported Ankh codec {0}")]
    UnsupportedCodec(u8),
    #[error("invalid Ankh payload length")]
    BadLength,
    #[error("Ankh payload checksum mismatch")]
    BadChecksum,
    #[error("asset path is not confined: {0}")]
    UnsafeAssetPath(String),
}

pub fn encode(payload: &[u8]) -> Result<Vec<u8>, ContainerError> {
    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(payload)?;
    let compressed = encoder.finish()?;
    let (flags, body) = if compressed.len() < payload.len() {
        (FLAG_DEFLATE, compressed.as_slice())
    } else {
        (0, payload)
    };
    let mut out = Vec::with_capacity(HEADER_LEN + body.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.push(CODEC_MESSAGEPACK);
    out.push(flags);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&crc32fast::hash(payload).to_le_bytes());
    out.extend_from_slice(body);
    Ok(out)
}

pub fn decode(bytes: &[u8]) -> Result<Vec<u8>, ContainerError> {
    if bytes.len() < HEADER_LEN || &bytes[..4] != MAGIC {
        return Err(ContainerError::BadMagic);
    }
    let version = u16::from_le_bytes([bytes[4], bytes[5]]);
    if version != VERSION {
        return Err(ContainerError::UnsupportedVersion(version));
    }
    if bytes[6] != CODEC_MESSAGEPACK {
        return Err(ContainerError::UnsupportedCodec(bytes[6]));
    }
    let flags = bytes[7];
    let raw_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let checksum = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
    let payload = if flags & FLAG_DEFLATE != 0 {
        let mut decoded = Vec::with_capacity(raw_len);
        flate2::read::DeflateDecoder::new(&bytes[HEADER_LEN..]).read_to_end(&mut decoded)?;
        decoded
    } else {
        bytes[HEADER_LEN..].to_vec()
    };
    if payload.len() != raw_len {
        return Err(ContainerError::BadLength);
    }
    if crc32fast::hash(&payload) != checksum {
        return Err(ContainerError::BadChecksum);
    }
    Ok(payload)
}

pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), ContainerError> {
    crate::safe_write::write(path, bytes)?;
    Ok(())
}

pub fn recoverable_writes(path: &Path) -> Vec<PathBuf> {
    crate::safe_write::recoverable(path)
}

pub fn asset_root(project: &Path) -> PathBuf {
    let name = project
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project");
    let stem = name
        .strip_suffix(".ankh.min.json")
        .or_else(|| name.strip_suffix(".ankh.json"))
        .or_else(|| name.strip_suffix(".ankh"))
        .unwrap_or(name);
    project.with_file_name(format!("{stem}.assets"))
}

pub fn confined_asset(root: &Path, uri: &str) -> Result<PathBuf, ContainerError> {
    let relative = Path::new(uri);
    let bytes = uri.as_bytes();
    let has_drive_prefix = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    let has_unsafe_portable_component = uri
        .split(['/', '\\'])
        .any(|part| matches!(part, "." | ".."));
    if relative.is_absolute()
        || has_drive_prefix
        || uri.starts_with('\\')
        || has_unsafe_portable_component
        || relative
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(ContainerError::UnsafeAssetPath(uri.into()));
    }
    Ok(root.join(relative))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn envelope_round_trips_and_detects_corruption() {
        let raw = vec![b'a'; 10_000];
        let encoded = encode(&raw).unwrap();
        assert!(encoded.len() < raw.len());
        assert_eq!(decode(&encoded).unwrap(), raw);
        let mut corrupt = encoded;
        *corrupt.last_mut().unwrap() ^= 1;
        assert!(decode(&corrupt).is_err());
    }
    #[test]
    fn asset_paths_cannot_escape() {
        let root = Path::new("assets");
        assert!(confined_asset(root, "ab/hash.png").is_ok());
        assert!(confined_asset(root, "../secret.png").is_err());
        assert!(confined_asset(root, "C:/secret.png").is_err());
        assert!(confined_asset(root, r"C:\secret.png").is_err());
        assert!(confined_asset(root, r"ab\..\secret.png").is_err());
        assert!(confined_asset(root, r"\\server\secret.png").is_err());
    }

    #[test]
    fn saving_never_clobbers_preexisting_temporary_or_backup_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        let temp = dir.path().join(".rig.ankh.tmp");
        let backup = dir
            .path()
            .join(format!(".rig.ankh.bak-{}", std::process::id()));
        std::fs::write(&path, b"old project").unwrap();
        std::fs::write(&temp, b"other session temp").unwrap();
        std::fs::write(&backup, b"other session backup").unwrap();
        let result = atomic_write(&path, b"new project");
        assert_eq!(std::fs::read(&temp).unwrap(), b"other session temp");
        assert_eq!(std::fs::read(&backup).unwrap(), b"other session backup");
        result.unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"new project");
    }

    #[test]
    fn a_directory_destination_is_never_moved_or_replaced() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rig.ankh");
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("user.txt"), b"user data").unwrap();
        assert!(atomic_write(&path, b"new project").is_err());
        assert!(path.is_dir());
        assert_eq!(std::fs::read(path.join("user.txt")).unwrap(), b"user data");
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    }
}
