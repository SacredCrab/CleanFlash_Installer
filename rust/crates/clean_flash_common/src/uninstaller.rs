use crate::{
    file_util, process_utils, registry, resources, system_info, winapi_helpers, InstallError,
    ProgressCallback,
};
use std::env;
use std::path::{Path, PathBuf};

const PROCESSES_TO_KILL: &[&str] = &[
    "fcbrowser",
    "fcbrowsermanager",
    "fclogin",
    "fctips",
    "flashcenter",
    "flashcenterservice",
    "flashcenteruninst",
    "flashplay",
    "update",
    "wow_helper",
    "dummy_cmd",
    "flashhelperservice",
    "flashplayerapp",
    "flashplayer_sa",
    "flashplayer_sa_debug",
];

const CONDITIONAL_PROCESSES: &[&str] = &[
    "plugin-container",
    "opera",
    "iexplore",
    "chrome",
    "chromium",
    "brave",
    "vivaldi",
    "msedge",
];

/// Unregister an ActiveX OCX file via regsvr32.
pub fn unregister_activex(filename: &str) -> Result<(), InstallError> {
    winapi_helpers::allow_modifications();

    let path = Path::new(filename);
    let dir = path.parent().unwrap_or(Path::new("."));
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let _prev = env::current_dir();
    let _ = env::set_current_dir(dir);

    let process = process_utils::run_process("regsvr32.exe", &["/s", "/u", &file_name]);
    if !process.is_successful() {
        return Err(InstallError::new(format!(
            "Failed to unregister ActiveX plugin: error code {}\n\n{}",
            process.exit_code, process.output
        )));
    }
    Ok(())
}

fn uninstall_registry() -> Result<(), InstallError> {
    system_info::with_system_info(|si| {
        if si.is_64bit {
            registry::apply_registry(&[
                resources::UNINSTALL_REGISTRY,
                resources::UNINSTALL_REGISTRY_64,
            ])
        } else {
            registry::apply_registry(&[resources::UNINSTALL_REGISTRY])
        }
    })
}

fn delete_task(task: &str) {
    process_utils::run_unmanaged_process("schtasks.exe", &["/delete", "/tn", task, "/f"]);
}

fn stop_service(service: &str) {
    process_utils::run_unmanaged_process("net.exe", &["stop", service]);
}

fn delete_service(service: &str) {
    stop_service(service);
    process_utils::run_unmanaged_process("sc.exe", &["delete", service]);
}

fn delete_flash_center() {
    // Remove Flash Center from Program Files.
    let pf = env::var("PROGRAMFILES").unwrap_or_default();
    file_util::wipe_folder(&PathBuf::from(&pf).join("FlashCenter"));

    if let Ok(pf86) = env::var("PROGRAMFILES(X86)") {
        file_util::wipe_folder(&PathBuf::from(&pf86).join("FlashCenter"));
    }

    // Remove start menu shortcuts.
    if let Some(appdata) = env::var("PROGRAMDATA").ok() {
        file_util::wipe_folder(
            &PathBuf::from(&appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Flash Center"),
        );
    }

    // Remove Flash Center cache / user data.
    if let Some(local) = env::var("LOCALAPPDATA").ok() {
        file_util::wipe_folder(&PathBuf::from(&local).join("Flash_Center"));
    }

    // Remove common start menu shortcuts.
    if let Some(appdata) = env::var("APPDATA").ok() {
        file_util::wipe_folder(
            &PathBuf::from(&appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Flash Center"),
        );
    }

    // Remove Desktop shortcuts.
    if let Some(desktop) = get_common_desktop() {
        file_util::delete_file(&desktop.join("Flash Center.lnk"));
    }
    if let Some(desktop) = dirs_desktop() {
        file_util::delete_file(&desktop.join("Flash Player.lnk"));
    }

    // Remove Flash Player from Program Files.
    system_info::with_system_info(|si| {
        file_util::wipe_folder(&si.flash_program32_path);
    });

    // Clean up temp folder spyware remnants.
    let temp = env::temp_dir();
    if let Ok(entries) = std::fs::read_dir(&temp) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.len() == 11 && name.ends_with(".tmp") && entry.path().is_dir() {
                let _ = file_util::wipe_folder(&entry.path());
            }
        }
    }
}

fn delete_flash_player() {
    system_info::with_system_info(|si| {
        // Remove Macromed folders.
        for dir in si.macromed_paths() {
            file_util::recursive_delete(dir, None);
        }

        // Remove Flash Player control panel apps.
        for sys_dir in si.system_paths() {
            file_util::delete_file(&sys_dir.join("FlashPlayerApp.exe"));
            file_util::delete_file(&sys_dir.join("FlashPlayerCPLApp.cpl"));
        }
    });
}

fn should_kill_conditional_process(name: &str, pid: u32) -> bool {
    if !CONDITIONAL_PROCESSES
        .iter()
        .any(|p| p.eq_ignore_ascii_case(name))
    {
        return false;
    }

    let modules = process_utils::collect_modules(pid);
    modules.iter().any(|m| {
        let lower = m.to_lowercase();
        lower.starts_with("flash32")
            || lower.starts_with("flash64")
            || lower.starts_with("libpepflash")
            || lower.starts_with("npswf")
    })
}

fn stop_processes() {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, WaitForSingleObject, PROCESS_QUERY_INFORMATION,
        PROCESS_TERMINATE, PROCESS_VM_READ,
    };

    // Enumerate all processes via the snapshot API.
    let pids = enumerate_processes();

    for (pid, name) in &pids {
        let lower = name.to_lowercase();
        let should_kill = PROCESSES_TO_KILL.iter().any(|p| *p == lower)
            || should_kill_conditional_process(&lower, *pid);

        if !should_kill {
            continue;
        }

        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, *pid);
            if !handle.is_null() {
                TerminateProcess(handle, 1);
                WaitForSingleObject(handle, 5000);
                CloseHandle(handle);
            }
        }
    }
}

fn enumerate_processes() -> Vec<(u32, String)> {
    use windows_sys::Win32::System::ProcessStatus::{EnumProcesses, GetModuleBaseNameW};
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
    use windows_sys::Win32::Foundation::CloseHandle;

    let mut results = Vec::new();
    let mut pids = [0u32; 4096];
    let mut bytes_returned: u32 = 0;

    unsafe {
        if EnumProcesses(
            pids.as_mut_ptr(),
            std::mem::size_of_val(&pids) as u32,
            &mut bytes_returned,
        ) == 0
        {
            return results;
        }

        let count = bytes_returned as usize / std::mem::size_of::<u32>();

        for &pid in &pids[..count] {
            if pid == 0 {
                continue;
            }

            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
            if handle.is_null() {
                continue;
            }

            let mut name_buf = [0u16; 260];
            let len = GetModuleBaseNameW(handle, std::ptr::null_mut(), name_buf.as_mut_ptr(), 260);
            CloseHandle(handle);

            if len > 0 {
                let name = String::from_utf16_lossy(&name_buf[..len as usize]);
                // Strip .exe suffix for matching.
                let base = name.strip_suffix(".exe").unwrap_or(&name).to_string();
                results.push((pid, base));
            }
        }
    }

    results
}

/// Perform the full uninstallation sequence.
pub fn uninstall(form: &dyn ProgressCallback) -> Result<(), InstallError> {
    winapi_helpers::allow_modifications();

    form.update_progress_label("Stopping Flash auto-updater task...", true);
    delete_task("Adobe Flash Player Updater");

    form.update_progress_label("Stopping Flash auto-updater service...", true);
    delete_service("AdobeFlashPlayerUpdateSvc");

    form.update_progress_label("Stopping Flash Center services...", true);
    delete_service("Flash Helper Service");
    form.tick_progress();
    delete_service("FlashCenterService");

    form.update_progress_label("Exiting all browsers...", true);
    stop_processes();

    form.update_progress_label("Cleaning up registry...", true);
    uninstall_registry()?;

    form.update_progress_label("Removing Flash Center...", true);
    delete_flash_center();

    form.update_progress_label("Removing Flash Player...", true);
    delete_flash_player();

    Ok(())
}

// Helper to get common desktop path.
fn get_common_desktop() -> Option<PathBuf> {
    env::var("PUBLIC")
        .ok()
        .map(|p| PathBuf::from(p).join("Desktop"))
}

fn dirs_desktop() -> Option<PathBuf> {
    env::var("USERPROFILE")
        .ok()
        .map(|p| PathBuf::from(p).join("Desktop"))
}
