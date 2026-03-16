use crate::ExitedProcess;
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Run a process, capturing stdout and stderr, and wait for it to exit.
pub fn run_process(program: &str, args: &[&str]) -> ExitedProcess {
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let result = cmd.output();

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}{}", stdout.trim(), stderr.trim());
            ExitedProcess {
                exit_code: output.status.code().unwrap_or(-1),
                output: combined,
            }
        }
        Err(e) => ExitedProcess {
            exit_code: -1,
            output: e.to_string(),
        },
    }
}

/// Run a process and wait for it to exit (no output capture).
pub fn run_unmanaged_process(program: &str, args: &[&str]) {
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let _ = cmd.status();
}

/// Collect the names of DLL modules loaded in a given process.
/// Used to detect whether a browser has Flash DLLs loaded.
pub fn collect_modules(pid: u32) -> Vec<String> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::ProcessStatus::{
        EnumProcessModulesEx, GetModuleFileNameExW, LIST_MODULES_ALL,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
    };

    let mut modules = Vec::new();

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if handle.is_null() {
            return modules;
        }
        #[allow(unused_imports)]
        use std::ptr;

        // HMODULE is *mut c_void on windows-sys 0.59
        let mut h_modules: [*mut std::ffi::c_void; 1024] =
            [std::ptr::null_mut(); 1024];
        let mut cb_needed: u32 = 0;

        let ok = EnumProcessModulesEx(
            handle,
            h_modules.as_mut_ptr().cast(),
            std::mem::size_of_val(&h_modules) as u32,
            &mut cb_needed,
            LIST_MODULES_ALL,
        );

        if ok != 0 {
            let count =
                cb_needed as usize / std::mem::size_of::<*mut std::ffi::c_void>();
            for item in h_modules.iter().take(count.min(h_modules.len())) {
                let mut name_buf = [0u16; 512];
                let len = GetModuleFileNameExW(
                    handle,
                    *item as _,
                    name_buf.as_mut_ptr(),
                    name_buf.len() as u32,
                );
                if len > 0 {
                    let full_path =
                        String::from_utf16_lossy(&name_buf[..len as usize]);
                    if let Some(file_name) = full_path.rsplit('\\').next() {
                        modules.push(file_name.to_string());
                    }
                }
            }
        }

        CloseHandle(handle);
    }

    modules
}
