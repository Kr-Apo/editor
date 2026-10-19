//! API keys are never serialized. Windows Credential Manager protects persistence.
use zeroize::Zeroizing;
pub struct Secret(pub Zeroizing<String>);
impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}
#[cfg(windows)]
mod platform {
    use super::*;
    use windows_sys::Win32::{
        Foundation::{ERROR_NOT_FOUND, GetLastError},
        Security::Credentials::*,
    };
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }
    pub fn load(target: &str) -> Result<Option<Secret>, String> {
        let name = wide(target);
        let mut pointer = std::ptr::null_mut();
        // SAFETY: NUL-terminated UTF-16 input; Windows owns returned allocation
        // until CredFree, and its checked blob size bounds the borrowed slice.
        unsafe {
            if CredReadW(name.as_ptr(), CRED_TYPE_GENERIC, 0, &mut pointer) == 0 {
                return if GetLastError() == ERROR_NOT_FOUND {
                    Ok(None)
                } else {
                    Err("Windows credential vault could not be read".into())
                };
            }
            let c = &*pointer;
            let value = if c.CredentialBlobSize == 0 {
                Ok(String::new())
            } else {
                String::from_utf8(
                    std::slice::from_raw_parts(c.CredentialBlob, c.CredentialBlobSize as usize)
                        .to_vec(),
                )
            };
            CredFree(pointer.cast());
            value
                .map(|s| Some(Secret(Zeroizing::new(s))))
                .map_err(|_| "Invalid credential encoding".into())
        }
    }
    pub fn store(target: &str, value: &str) -> Result<(), String> {
        if value.is_empty() || value.len() > 2560 || value.contains(['\r', '\n', '\0']) {
            return Err("API key is empty or invalid".into());
        }
        let mut name = wide(target);
        let mut user = wide("Ankhimate AI");
        let c = CREDENTIALW {
            Type: CRED_TYPE_GENERIC,
            TargetName: name.as_mut_ptr(),
            CredentialBlobSize: value.len() as u32,
            CredentialBlob: value.as_ptr() as *mut u8,
            Persist: CRED_PERSIST_LOCAL_MACHINE,
            UserName: user.as_mut_ptr(),
            ..unsafe { std::mem::zeroed() }
        };
        // SAFETY: all referenced buffers live through the synchronous OS call.
        if unsafe { CredWriteW(&c, 0) } == 0 {
            Err("Windows could not securely store the API key".into())
        } else {
            Ok(())
        }
    }
    pub fn remove(target: &str) -> Result<(), String> {
        let name = wide(target);
        // SAFETY: valid NUL-terminated name; only our exact application target.
        if unsafe { CredDeleteW(name.as_ptr(), CRED_TYPE_GENERIC, 0) } == 0
            && unsafe { GetLastError() } != ERROR_NOT_FOUND
        {
            Err("Windows could not remove the saved key".into())
        } else {
            Ok(())
        }
    }
}
#[cfg(not(windows))]
mod platform {
    use super::*;
    pub fn load(_: &str) -> Result<Option<Secret>, String> {
        Ok(None)
    }
    pub fn store(_: &str, _: &str) -> Result<(), String> {
        Err("Secure key persistence is currently Windows-only. Use a session key.".into())
    }
    pub fn remove(_: &str) -> Result<(), String> {
        Ok(())
    }
}
pub use platform::{load, remove, store};
