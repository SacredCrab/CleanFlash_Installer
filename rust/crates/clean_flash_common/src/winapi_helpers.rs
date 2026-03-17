/// Enable SeRestorePrivilege and SeTakeOwnershipPrivilege for the current process.
#[cfg(windows)]
pub fn allow_modifications() {
    let _ = modify_privilege("SeRestorePrivilege\0", true);
    let _ = modify_privilege("SeTakeOwnershipPrivilege\0", true);
}

#[cfg(not(windows))]
pub fn allow_modifications() {
    // No privilege modifications needed on Unix.
}

#[cfg(windows)]
fn modify_privilege(name: &str, enable: bool) -> Result<(), ()> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::Security::{
        AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES,
        SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token: *mut std::ffi::c_void = std::ptr::null_mut();
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        ) == 0
        {
            return Err(());
        }

        let wide_name: Vec<u16> = name.encode_utf16().collect();
        let mut luid = std::mem::zeroed();
        if LookupPrivilegeValueW(std::ptr::null(), wide_name.as_ptr(), &mut luid) == 0 {
            CloseHandle(token);
            return Err(());
        }

        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: if enable { SE_PRIVILEGE_ENABLED } else { 0 },
            }],
        };

        let result = AdjustTokenPrivileges(
            token,
            0,
            &mut tp,
            std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        CloseHandle(token);

        if result == 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}
