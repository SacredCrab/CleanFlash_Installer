/// Disable WoW64 file system redirection. Returns a cookie to restore later.
#[cfg(windows)]
pub fn disable_redirection() -> *mut std::ffi::c_void {
    let mut old_value: *mut std::ffi::c_void = std::ptr::null_mut();
    unsafe {
        Wow64DisableWow64FsRedirection(&mut old_value);
    }
    old_value
}

/// Re-enable WoW64 file system redirection using the cookie from `disable_redirection`.
#[cfg(windows)]
pub fn enable_redirection(old_value: *mut std::ffi::c_void) {
    unsafe {
        Wow64RevertWow64FsRedirection(old_value);
    }
}

#[cfg(windows)]
extern "system" {
    fn Wow64DisableWow64FsRedirection(old_value: *mut *mut std::ffi::c_void) -> i32;
    fn Wow64RevertWow64FsRedirection(old_value: *mut std::ffi::c_void) -> i32;
}

#[cfg(not(windows))]
pub fn disable_redirection() -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}

#[cfg(not(windows))]
pub fn enable_redirection(_old_value: *mut std::ffi::c_void) {}
