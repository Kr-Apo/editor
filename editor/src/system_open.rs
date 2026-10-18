//! Explicit file-association opening. Native APIs only; never a shell command string.
use std::path::Path;
pub const fn available() -> bool {
    cfg!(windows)
}
#[cfg(test)]
pub fn open_file(_path: &Path) -> Result<(), String> {
    panic!("Native reader launch is forbidden in automated tests; inject an opener")
}
#[cfg(not(test))]
pub fn open_file(path: &Path) -> Result<(), String> {
    if !path.is_absolute()
        || !path.is_file()
        || path
            .extension()
            .and_then(|s| s.to_str())
            .is_none_or(|s| !s.eq_ignore_ascii_case("mp4") && !s.eq_ignore_ascii_case("gif"))
    {
        return Err("Only a published absolute MP4/GIF file may be opened".into());
    }
    #[cfg(windows)]
    {
        windows::open(path)
    }
    #[cfg(not(windows))]
    {
        Err("Opening exported files is unsupported on this platform".into())
    }
}
#[cfg(all(windows, not(test)))]
mod windows {
    use super::*;
    use std::{ffi::c_void, os::windows::ffi::OsStrExt, ptr};
    #[repr(C)]
    struct ShellExecuteInfo {
        size: u32,
        mask: u32,
        window: *mut c_void,
        verb: *const u16,
        file: *const u16,
        parameters: *const u16,
        directory: *const u16,
        show: i32,
        instance: *mut c_void,
        id_list: *mut c_void,
        class: *const u16,
        class_key: *mut c_void,
        hot_key: u32,
        icon_or_monitor: *mut c_void,
        process: *mut c_void,
    }
    #[link(name = "Shell32")]
    unsafe extern "system" {
        fn ShellExecuteExW(info: *mut ShellExecuteInfo) -> i32;
    }
    #[link(name = "Ole32")]
    unsafe extern "system" {
        fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> i32;
        fn CoUninitialize();
    }
    struct Apartment;
    impl Drop for Apartment {
        fn drop(&mut self) {
            // SAFETY: constructed only after a successful CoInitializeEx on this thread.
            unsafe {
                CoUninitialize();
            }
        }
    }
    pub(super) fn open(path: &Path) -> Result<(), String> {
        let mut file: Vec<u16> = path.as_os_str().encode_wide().collect();
        if file.contains(&0) {
            return Err("Invalid NUL in exported path".into());
        }
        file.push(0);
        let verb: [u16; 5] = [b'o' as u16, b'p' as u16, b'e' as u16, b'n' as u16, 0];
        // SAFETY: null reserved pointer; STA + disable OLE1 DDE on our dedicated worker.
        let hr = unsafe { CoInitializeEx(ptr::null_mut(), 0x2 | 0x4) };
        if hr < 0 {
            return Err(format!(
                "Cannot initialize Windows file opening: HRESULT 0x{:08x}",
                hr as u32
            ));
        }
        let _apartment = Apartment;
        let mut info = ShellExecuteInfo {
            size: std::mem::size_of::<ShellExecuteInfo>() as u32,
            mask: 0x100 | 0x400, // NOASYNC + FLAG_NO_UI; no environment expansion, no process handle.
            window: ptr::null_mut(),
            verb: verb.as_ptr(),
            file: file.as_ptr(),
            parameters: ptr::null(),
            directory: ptr::null(),
            show: 1,
            instance: ptr::null_mut(),
            id_list: ptr::null_mut(),
            class: ptr::null(),
            class_key: ptr::null_mut(),
            hot_key: 0,
            icon_or_monitor: ptr::null_mut(),
            process: ptr::null_mut(),
        };
        // SAFETY: repr(C) matches SHELLEXECUTEINFOW; buffers live through this synchronous
        // call, are UTF-16 NUL-terminated, parameters are null. No command string or shell.
        if unsafe { ShellExecuteExW(&mut info) } == 0 {
            return Err(format!(
                "Windows could not open the published file: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(()) // OS accepted the request; does not certify playback in the associated app.
    }
}
