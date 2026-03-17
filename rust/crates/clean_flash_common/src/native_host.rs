//! Install and uninstall the native messaging host for modern browsers.
//!
//! The native messaging host allows the Clean Flash Player browser extension
//! to communicate with the local flash-player-host binary via stdio.

use crate::{InstallError, ProgressCallback};
use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST_NAME: &str = "org.cleanflash.flash_player";
const FIREFOX_MANIFEST_FILENAME: &str = "clean_flash_firefox.json";
const CHROME_MANIFEST_FILENAME: &str = "clean_flash_chrome.json";
const FIREFOX_ALLOWED_EXTENSION: &str = "flash-player@cleanflash.org";
const ALLOWED_ORIGIN: &str = "chrome-extension://dcikaadaeajidejkoekdflmfdgeoldcb/";

#[cfg(windows)]
const HOST_BINARY_NAME: &str = "flash-player-host.exe";

#[cfg(not(windows))]
const HOST_BINARY_NAME: &str = "flash-player-host";

#[derive(Clone, Copy)]
enum BrowserKind {
    Firefox,
    ChromeLike,
}

#[cfg(windows)]
struct WindowsBrowser {
    detect_keys: &'static [&'static str],
    native_messaging_reg_path: &'static str,
    kind: BrowserKind,
}

/// Browser registry key paths on Windows (HKCU/HKLM) for native messaging hosts.
#[cfg(windows)]
const WINDOWS_BROWSERS: &[WindowsBrowser] = &[
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Google\Chrome"],
        native_messaging_reg_path: r"SOFTWARE\Google\Chrome\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Microsoft\Edge"],
        native_messaging_reg_path: r"SOFTWARE\Microsoft\Edge\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\BraveSoftware\Brave-Browser"],
        native_messaging_reg_path: r"SOFTWARE\BraveSoftware\Brave-Browser\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Vivaldi"],
        native_messaging_reg_path: r"SOFTWARE\Vivaldi\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Opera Software"],
        native_messaging_reg_path: r"SOFTWARE\Opera Software\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Chromium"],
        native_messaging_reg_path: r"SOFTWARE\Chromium\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\The Browser Company\Arc"],
        native_messaging_reg_path: r"SOFTWARE\The Browser Company\Arc\NativeMessagingHosts",
        kind: BrowserKind::ChromeLike,
    },
    WindowsBrowser {
        detect_keys: &[r"SOFTWARE\Mozilla\Mozilla Firefox", r"SOFTWARE\Mozilla"],
        native_messaging_reg_path: r"SOFTWARE\Mozilla\NativeMessagingHosts",
        kind: BrowserKind::Firefox,
    },
];

struct BrowserManifestTarget {
    detect_path: PathBuf,
    manifest_dir: PathBuf,
    kind: BrowserKind,
}

/// Build the JSON manifest content for the native messaging host.
fn build_manifest_json(host_path: &Path, kind: BrowserKind) -> String {
    // Escape backslashes in the path for JSON.
    let path_str = host_path.to_string_lossy().replace('\\', "\\\\");
    match kind {
        BrowserKind::Firefox => format!(
            "{{\n  \"name\": \"{}\",\n  \"description\": \"Flash Player Native Messaging Host\",\n  \"path\": \"{}\",\n  \"type\": \"stdio\",\n  \"allowed_extensions\": [\"{}\"]\n}}\n",
            MANIFEST_NAME, path_str, FIREFOX_ALLOWED_EXTENSION,
        ),
        BrowserKind::ChromeLike => format!(
            "{{\n  \"name\": \"{}\",\n  \"description\": \"Flash Player Native Messaging Host\",\n  \"path\": \"{}\",\n  \"type\": \"stdio\",\n  \"allowed_origins\": [\"{}\"]\n}}\n",
            MANIFEST_NAME, path_str, ALLOWED_ORIGIN,
        ),
    }
}

/// Get the path where the native host exe should be installed on Windows.
/// On a 64-bit system it goes into the System32\Macromed\Flash folder (the 64-bit one).
/// On a 32-bit system it goes into the SysWOW64 (or System32) Macromed\Flash folder.
#[cfg(windows)]
pub fn get_native_host_install_dir() -> PathBuf {
    crate::system_info::with_system_info(|si| {
        if si.is_64bit {
            si.flash64_path.clone()
        } else {
            si.flash32_path.clone()
        }
    })
}

#[cfg(not(windows))]
pub fn get_native_host_install_dir() -> PathBuf {
    if cfg!(target_os = "macos") {
        // Install to ~/Library/Application Support/Clean Flash instead
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join("Library/Application Support/Clean Flash")
        } else {
            PathBuf::from("/Library/Application Support/Clean Flash")
        }
    } else if cfg!(target_os = "linux") {
        // Linux
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".cleanflash")
        } else {
            PathBuf::from("/tmp/.cleanflash")
        }
    } else {
        unreachable!()
    }
}

/// Get the full path to the installed native host executable.
pub fn get_native_host_exe_path() -> PathBuf {
    get_native_host_install_dir().join(HOST_BINARY_NAME)
}

fn manifest_filename_for_kind(kind: BrowserKind) -> &'static str {
    match kind {
        BrowserKind::Firefox => FIREFOX_MANIFEST_FILENAME,
        BrowserKind::ChromeLike => CHROME_MANIFEST_FILENAME,
    }
}

#[cfg(not(windows))]
fn get_non_windows_browser_targets(home: &Path) -> Vec<BrowserManifestTarget> {
    if cfg!(target_os = "macos") {
        vec![
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Google/Chrome"),
                manifest_dir: home.join("Library/Application Support/Google/Chrome/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Microsoft Edge"),
                manifest_dir: home.join("Library/Application Support/Microsoft Edge/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/BraveSoftware/Brave-Browser"),
                manifest_dir: home.join("Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Vivaldi"),
                manifest_dir: home.join("Library/Application Support/Vivaldi/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Chromium"),
                manifest_dir: home.join("Library/Application Support/Chromium/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Arc"),
                manifest_dir: home.join("Library/Application Support/Arc/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/Mozilla"),
                manifest_dir: home.join("Library/Application Support/Mozilla/NativeMessagingHosts"),
                kind: BrowserKind::Firefox,
            },
            BrowserManifestTarget {
                detect_path: home.join("Library/Application Support/net.imput.helium"),
                manifest_dir: home.join("Library/Application Support/net.imput.helium/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            }
        ]
    } else {
        vec![
            BrowserManifestTarget {
                detect_path: home.join(".config/google-chrome"),
                manifest_dir: home.join(".config/google-chrome/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/microsoft-edge"),
                manifest_dir: home.join(".config/microsoft-edge/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/BraveSoftware/Brave-Browser"),
                manifest_dir: home.join(".config/BraveSoftware/Brave-Browser/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/vivaldi"),
                manifest_dir: home.join(".config/vivaldi/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/chromium"),
                manifest_dir: home.join(".config/chromium/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/arc"),
                manifest_dir: home.join(".config/arc/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".config/net.imput.helium"),
                manifest_dir: home.join(".config/net.imput.helium/NativeMessagingHosts"),
                kind: BrowserKind::ChromeLike,
            },
            BrowserManifestTarget {
                detect_path: home.join(".mozilla"),
                manifest_dir: home.join(".mozilla/native-messaging-hosts"),
                kind: BrowserKind::Firefox,
            }
        ]
    }
}

#[cfg(windows)]
fn set_windows_manifest_registry_value(nmh_base_key: &str, manifest_path: &str) {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegSetValueExW, HKEY_CURRENT_USER,
        KEY_WOW64_64KEY, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
    };

    let reg_key = format!("{}\\{}", nmh_base_key, MANIFEST_NAME);
    let key_wide: Vec<u16> = reg_key.encode_utf16().chain(std::iter::once(0)).collect();
    let val_wide: Vec<u16> = manifest_path
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut hkey = std::ptr::null_mut();
        let mut disposition: u32 = 0;
        let result = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            key_wide.as_ptr(),
            0,
            std::ptr::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE | KEY_WOW64_64KEY,
            std::ptr::null(),
            &mut hkey,
            &mut disposition,
        );
        if result == 0 {
            RegSetValueExW(
                hkey,
                std::ptr::null(),
                0,
                REG_SZ as u32,
                val_wide.as_ptr() as *const u8,
                (val_wide.len() * 2) as u32,
            );
            RegCloseKey(hkey);
        }
    }
}

#[cfg(windows)]
fn delete_windows_manifest_registry_value(nmh_base_key: &str) {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegDeleteTreeW, RegOpenKeyExW, HKEY_CURRENT_USER,
        KEY_WOW64_64KEY, KEY_WRITE,
    };

    unsafe {
        let parent_wide: Vec<u16> = nmh_base_key
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let name_wide: Vec<u16> = MANIFEST_NAME
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let mut parent_hkey = std::ptr::null_mut();
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            parent_wide.as_ptr(),
            0,
            KEY_WRITE | KEY_WOW64_64KEY,
            &mut parent_hkey,
        );
        if result == 0 {
            RegDeleteTreeW(parent_hkey, name_wide.as_ptr());
            RegCloseKey(parent_hkey);
        }
    }
}

/// Install the native messaging host manifests for all detected browsers.
#[cfg(windows)]
pub fn install_native_host(form: &dyn ProgressCallback) -> Result<(), InstallError> {
    form.update_progress_label("Installing native messaging host for modern browsers...", true);

    let host_exe = get_native_host_exe_path();
    let chrome_manifest_json = build_manifest_json(&host_exe, BrowserKind::ChromeLike);
    let firefox_manifest_json = build_manifest_json(&host_exe, BrowserKind::Firefox);

    // Create manifest directory inside the install dir.
    let manifest_dir = get_native_host_install_dir().join("manifests");
    let _ = fs::create_dir_all(&manifest_dir);
    let chrome_manifest_path = manifest_dir.join(CHROME_MANIFEST_FILENAME);
    fs::write(&chrome_manifest_path, chrome_manifest_json.as_bytes()).map_err(|e| {
        InstallError::new(format!(
            "Failed to write Chrome native messaging manifest: {}",
            e
        ))
    })?;
    let firefox_manifest_path = manifest_dir.join(FIREFOX_MANIFEST_FILENAME);
    fs::write(&firefox_manifest_path, firefox_manifest_json.as_bytes()).map_err(|e| {
        InstallError::new(format!(
            "Failed to write Firefox native messaging manifest: {}",
            e
        ))
    })?;

    let chrome_manifest_path_str = chrome_manifest_path.to_string_lossy().to_string();
    let firefox_manifest_path_str = firefox_manifest_path.to_string_lossy().to_string();

    // For each browser that is detected via registry, register the manifest.
    for browser in WINDOWS_BROWSERS {
        if !browser
            .detect_keys
            .iter()
            .any(|detect_key| windows_registry_key_exists(detect_key))
        {
            continue;
        }

        let manifest_path = match browser.kind {
            BrowserKind::Firefox => &firefox_manifest_path_str,
            BrowserKind::ChromeLike => &chrome_manifest_path_str,
        };
        set_windows_manifest_registry_value(browser.native_messaging_reg_path, manifest_path);
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn install_native_host(form: &dyn ProgressCallback) -> Result<(), InstallError> {
    form.update_progress_label("Installing native messaging host for modern browsers...", true);

    let host_exe = get_native_host_exe_path();
    let chrome_manifest_json = build_manifest_json(&host_exe, BrowserKind::ChromeLike);
    let firefox_manifest_json = build_manifest_json(&host_exe, BrowserKind::Firefox);

    // Determine browser config directories that exist.
    let home = std::env::var("HOME")
        .map_err(|_| InstallError::new("HOME environment variable not set"))?;
    let home = PathBuf::from(home);

    // Check known browser paths and install manifests only for those that exist.
    for target in get_non_windows_browser_targets(&home) {
        if !target.detect_path.exists() {
            continue;
        }

        let _ = fs::create_dir_all(&target.manifest_dir);
        let manifest_filename = manifest_filename_for_kind(target.kind);
        let manifest_json = match target.kind {
            BrowserKind::Firefox => &firefox_manifest_json,
            BrowserKind::ChromeLike => &chrome_manifest_json,
        };
        let manifest_path = target.manifest_dir.join(manifest_filename);
        let _ = fs::write(&manifest_path, manifest_json.as_bytes());
    }

    Ok(())
}

/// Uninstall the native messaging host: remove manifests and registry entries.
#[cfg(windows)]
pub fn uninstall_native_host(form: &dyn ProgressCallback) {
    form.update_progress_label("Removing native messaging host...", true);

    // Remove manifest files.
    let install_dir = get_native_host_install_dir();
    let manifest_dir = install_dir.join("manifests");
    let _ = fs::remove_file(manifest_dir.join(CHROME_MANIFEST_FILENAME));
    let _ = fs::remove_file(manifest_dir.join(FIREFOX_MANIFEST_FILENAME));
    let _ = fs::remove_dir(&manifest_dir);

    // Remove the host exe.
    let host_exe = install_dir.join(HOST_BINARY_NAME);
    crate::file_util::delete_file(&host_exe);

    // Remove registry entries for all browsers.
    for browser in WINDOWS_BROWSERS {
        delete_windows_manifest_registry_value(browser.native_messaging_reg_path);
    }
}

#[cfg(not(windows))]
pub fn uninstall_native_host(form: &dyn ProgressCallback) {
    form.update_progress_label("Removing native messaging host...", true);

    // Remove the manifests.
    let home = match std::env::var("HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => return,
    };

    for target in get_non_windows_browser_targets(&home) {
        let _ = fs::remove_file(target.manifest_dir.join(CHROME_MANIFEST_FILENAME));
        let _ = fs::remove_file(target.manifest_dir.join(FIREFOX_MANIFEST_FILENAME));
    }

    // Remove the host folder and everything inside it.
    let install_dir = get_native_host_install_dir();
    let host_exe = install_dir.join(HOST_BINARY_NAME);
    let _ = fs::remove_file(&host_exe);
    let _ = fs::remove_dir_all(&install_dir);
}

/// Check if a registry key exists under HKCU.
#[cfg(windows)]
fn windows_registry_key_exists(subkey: &str) -> bool {
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
        KEY_READ, KEY_WOW64_64KEY,
    };

    let key_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

    // Check HKCU first, then HKLM.
    for &root in &[HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        unsafe {
            let mut hkey = std::ptr::null_mut();
            let result = RegOpenKeyExW(root, key_wide.as_ptr(), 0, KEY_READ | KEY_WOW64_64KEY, &mut hkey);
            if result == 0 {
                RegCloseKey(hkey);
                return true;
            }
        }
    }
    false
}
