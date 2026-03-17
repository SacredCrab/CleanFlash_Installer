#[cfg(windows)]
use crate::uninstaller;
use std::fs;
use std::path::Path;
#[cfg(windows)]
use std::thread;
#[cfg(windows)]
use std::time::Duration;

/// Attempt to delete a single file, retrying with escalating measures if needed.
pub fn delete_file(path: &Path) {
    if !path.exists() {
        return;
    }

    // Unregister ActiveX .ocx files before deletion (Windows only).
    #[cfg(windows)]
    if let Some(ext) = path.extension() {
        if ext.eq_ignore_ascii_case("ocx") {
            let _ = uninstaller::unregister_activex(&path.to_string_lossy());
        }
    }

    // First attempt: clear read-only and delete.
    if try_clear_readonly_and_delete(path) {
        return;
    }

    // Retry loop with ownership acquisition.
    #[cfg(windows)]
    {
        for _ in 0..10 {
            if try_take_ownership_and_delete(path) {
                return;
            }
            thread::sleep(Duration::from_millis(500));
        }

        // Last resort: kill any processes using the file, then delete.
        kill_locking_processes(path);
        thread::sleep(Duration::from_millis(500));
        let _ = fs::remove_file(path);
    }
}

/// Recursively delete all files (optionally matching `filename`) under `base_dir`.
pub fn recursive_delete(base_dir: &Path, filename: Option<&str>) {
    if !base_dir.exists() {
        return;
    }

    let entries: Vec<_> = match fs::read_dir(base_dir) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            recursive_delete(&path, filename);
        } else if path.is_file() {
            // Sanity check: path must start with the original base_dir.
            if !path.starts_with(base_dir) {
                continue;
            }

            let should_delete = match filename {
                Some(name) => path
                    .file_name()
                    .map(|f| f == name)
                    .unwrap_or(false),
                None => true,
            };

            if should_delete {
                delete_file(&path);
            }
        }
    }
}

/// Delete all files in a folder, then try to remove the folder itself.
pub fn wipe_folder(path: &Path) {
    if !path.exists() {
        return;
    }

    recursive_delete(path, None);

    // If folder is now empty, remove it.
    if is_dir_empty(path) {
        if fs::remove_dir(path).is_err() {
            #[cfg(windows)]
            {
                kill_locking_processes(path);
                thread::sleep(Duration::from_millis(500));
                let _ = fs::remove_dir(path);
            }
        }
    }
}

fn try_clear_readonly_and_delete(path: &Path) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        let mut perms = meta.permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        perms.set_readonly(false);
        let _ = fs::set_permissions(path, perms);
    }
    fs::remove_file(path).is_ok()
}

#[cfg(windows)]
fn try_take_ownership_and_delete(path: &Path) -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree};
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenUser, DACL_SECURITY_INFORMATION,
        OWNER_SECURITY_INFORMATION, TOKEN_QUERY, TOKEN_USER,
    };
    use windows_sys::Win32::Security::Authorization::{
        SetEntriesInAclW, SetNamedSecurityInfoW, EXPLICIT_ACCESS_W,
        SE_FILE_OBJECT, SET_ACCESS, TRUSTEE_IS_SID, TRUSTEE_IS_USER, TRUSTEE_W,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    // Ensure SeTakeOwnershipPrivilege is enabled (idempotent).
    crate::winapi_helpers::allow_modifications();

    let path_wide: Vec<u16> = path
        .to_string_lossy()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return try_clear_readonly_and_delete(path);
        }

        // Retrieve the current user's SID from the process token.
        let mut buf = vec![0u8; 512];
        let mut returned = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenUser,
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
            &mut returned,
        );
        CloseHandle(token);

        if ok == 0 {
            return try_clear_readonly_and_delete(path);
        }

        let token_user = &*(buf.as_ptr() as *const TOKEN_USER);
        let sid = token_user.User.Sid;

        // Transfer ownership of the file to the current user.
        SetNamedSecurityInfoW(
            path_wide.as_ptr() as *mut _,
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION,
            sid,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        // Build a new DACL that grants FullControl to the current user.
        let mut ea = EXPLICIT_ACCESS_W {
            grfAccessPermissions: 0x001F_01FF, // FILE_ALL_ACCESS
            grfAccessMode: SET_ACCESS,
            grfInheritance: 0, // NO_INHERITANCE
            Trustee: TRUSTEE_W {
                pMultipleTrustee: std::ptr::null_mut(),
                MultipleTrusteeOperation: 0, // NO_MULTIPLE_TRUSTEE
                TrusteeForm: TRUSTEE_IS_SID,
                TrusteeType: TRUSTEE_IS_USER,
                ptstrName: sid as *mut u16,
            },
        };
        let mut new_dacl: *mut windows_sys::Win32::Security::ACL = std::ptr::null_mut();
        SetEntriesInAclW(1, &mut ea, std::ptr::null_mut(), &mut new_dacl);

        if !new_dacl.is_null() {
            SetNamedSecurityInfoW(
                path_wide.as_ptr() as *mut _,
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                new_dacl,
                std::ptr::null_mut(),
            );
            LocalFree(new_dacl as *mut _);
        }
    }

    try_clear_readonly_and_delete(path)
}

#[cfg(windows)]
fn kill_locking_processes(path: &Path) {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::RestartManager::{
        RmEndSession, RmGetList, RmRegisterResources, RmStartSession, RM_PROCESS_INFO,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, WaitForSingleObject, PROCESS_TERMINATE,
    };

    let path_wide: Vec<u16> = path
        .to_string_lossy()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut session: u32 = 0;
        // CCH_RM_SESSION_KEY = 32 chars; +1 for null terminator.
        let mut session_key = [0u16; 33];
        if RmStartSession(&mut session, 0, session_key.as_mut_ptr()) != 0 {
            return;
        }

        let file_ptr = path_wide.as_ptr();
        let files = [file_ptr];
        RmRegisterResources(
            session,
            1,
            files.as_ptr(),
            0,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        );

        let mut n_needed: u32 = 0;
        let mut n_info: u32 = 10;
        let mut procs: [RM_PROCESS_INFO; 10] = std::mem::zeroed();
        let mut reboot_reasons: u32 = 0;
        RmGetList(
            session,
            &mut n_needed,
            &mut n_info,
            procs.as_mut_ptr(),
            &mut reboot_reasons,
        );

        for proc_info in procs.iter().take(n_info as usize) {
            let pid = proc_info.Process.dwProcessId;
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            if !handle.is_null() {
                TerminateProcess(handle, 1);
                WaitForSingleObject(handle, 5000);
                CloseHandle(handle);
            }
        }

        RmEndSession(session);
    }
}

fn is_dir_empty(path: &Path) -> bool {
    fs::read_dir(path)
        .map(|mut rd| rd.next().is_none())
        .unwrap_or(true)
}
