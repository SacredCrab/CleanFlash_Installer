use crate::update_checker;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// Lazily-initialized system path info, analogous to C# SystemInfo class.
pub struct SystemInfo {
    pub system32_path: PathBuf,
    pub system64_path: PathBuf,
    pub program32_path: PathBuf,
    pub flash_program32_path: PathBuf,
    pub macromed32_path: PathBuf,
    pub macromed64_path: PathBuf,
    pub flash32_path: PathBuf,
    pub flash64_path: PathBuf,
    pub version: String,
    pub version_path: String,
    pub version_comma: String,
    pub is_64bit: bool,
    replacements: HashMap<String, String>,
}

impl SystemInfo {
    pub fn new() -> Self {
        let system32_path = get_syswow64_path();
        let system64_path = get_system32_path();
        let program32_path = get_program_files_x86();
        let flash_program32_path = program32_path.join("Flash Player");
        let macromed32_path = system32_path.join("Macromed");
        let macromed64_path = system64_path.join("Macromed");
        let flash32_path = macromed32_path.join("Flash");
        let flash64_path = macromed64_path.join("Flash");
        let version = update_checker::FLASH_VERSION.to_string();
        let version_path = version.replace('.', "_");
        let version_comma = version.replace('.', ",");
        let is_64bit = cfg!(target_pointer_width = "64")
            || env::var("PROCESSOR_ARCHITEW6432").is_ok();

        let arch = if is_64bit { "64" } else { "32" };

        let mut replacements = HashMap::new();
        replacements.insert(
            "${SYSTEM_32_PATH}".into(),
            system32_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert(
            "${SYSTEM_64_PATH}".into(),
            system64_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert(
            "${PROGRAM_32_PATH}".into(),
            program32_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert(
            "${PROGRAM_FLASH_32_PATH}".into(),
            flash_program32_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert(
            "${FLASH_32_PATH}".into(),
            flash32_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert(
            "${FLASH_64_PATH}".into(),
            flash64_path.to_string_lossy().replace('\\', "\\\\"),
        );
        replacements.insert("${VERSION}".into(), version.clone());
        replacements.insert("${VERSION_PATH}".into(), version_path.clone());
        replacements.insert("${VERSION_COMMA}".into(), version_comma.clone());
        replacements.insert("${ARCH}".into(), arch.into());

        Self {
            system32_path,
            system64_path,
            program32_path,
            flash_program32_path,
            macromed32_path,
            macromed64_path,
            flash32_path,
            flash64_path,
            version,
            version_path,
            version_comma,
            is_64bit,
            replacements,
        }
    }

    pub fn system_paths(&self) -> Vec<&Path> {
        if self.is_64bit {
            vec![&self.system32_path, &self.system64_path]
        } else {
            vec![&self.system32_path]
        }
    }

    pub fn macromed_paths(&self) -> Vec<&Path> {
        if self.is_64bit {
            vec![&self.macromed32_path, &self.macromed64_path]
        } else {
            vec![&self.macromed32_path]
        }
    }

    pub fn fill_string(&self, s: &str) -> String {
        let mut result = s.to_string();
        for (key, value) in &self.replacements {
            result = result.replace(key.as_str(), value);
        }
        result
    }

    #[cfg(windows)]
    pub fn is_legacy_windows(&self) -> bool {
        // Windows version < 6.2 (before Windows 8).
        unsafe {
            let mut info: windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW =
                std::mem::zeroed();
            info.dwOSVersionInfoSize =
                std::mem::size_of::<windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW>()
                    as u32;
            // RtlGetVersion always succeeds and isn't deprecated like GetVersionEx.
            rtl_get_version(&mut info);
            info.dwMajorVersion < 6
                || (info.dwMajorVersion == 6 && info.dwMinorVersion < 2)
        }
    }

    #[cfg(not(windows))]
    pub fn is_legacy_windows(&self) -> bool {
        false
    }
}

#[cfg(windows)]
extern "system" {
    fn RtlGetVersion(
        lp_version_information: *mut windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW,
    ) -> i32;
}

#[cfg(windows)]
unsafe fn rtl_get_version(
    info: &mut windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW,
) {
    RtlGetVersion(info as *mut _);
}

/// Global convenience: fill replacement strings using a default SystemInfo.
pub fn fill_string(s: &str) -> String {
    SYSTEM_INFO.with(|si| si.fill_string(s))
}

thread_local! {
    static SYSTEM_INFO: SystemInfo = SystemInfo::new();
}

pub fn with_system_info<F, R>(f: F) -> R
where
    F: FnOnce(&SystemInfo) -> R,
{
    SYSTEM_INFO.with(f)
}

fn get_system32_path() -> PathBuf {
    PathBuf::from(env::var("SYSTEMROOT").unwrap_or_else(|_| r"C:\Windows".into()))
        .join("System32")
}

fn get_syswow64_path() -> PathBuf {
    let root = env::var("SYSTEMROOT").unwrap_or_else(|_| r"C:\Windows".into());
    let wow64 = PathBuf::from(&root).join("SysWOW64");
    if wow64.exists() {
        wow64
    } else {
        PathBuf::from(&root).join("System32")
    }
}

fn get_program_files_x86() -> PathBuf {
    if let Ok(pf86) = env::var("PROGRAMFILES(X86)") {
        PathBuf::from(pf86)
    } else if let Ok(pf) = env::var("PROGRAMFILES") {
        PathBuf::from(pf)
    } else {
        PathBuf::from(r"C:\Program Files")
    }
}
