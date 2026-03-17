use crate::install_flags::{self, InstallFlags};
use clean_flash_common::{
    native_host, process_utils, registry, resources, system_info, InstallError, ProgressCallback,
};
use std::env;
use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

/// Metadata for a single installable component.
struct InstallEntry {
    install_text: &'static str,
    required_flags: u32,
    target_directory: PathBuf,
    registry_instructions: Option<&'static str>,
}

/// Register an ActiveX OCX via regsvr32 (unregister first, then register).
pub fn register_activex(filename: &str) -> Result<(), InstallError> {
    let path = Path::new(filename);
    let dir = path.parent().unwrap_or(Path::new("."));
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let _ = env::set_current_dir(dir);

    let process = process_utils::run_process("regsvr32.exe", &["/s", "/u", &file_name]);
    if !process.is_successful() {
        return Err(InstallError::new(format!(
            "Failed to unregister ActiveX plugin: error code {}\n\n{}",
            process.exit_code, process.output
        )));
    }

    let process = process_utils::run_process("regsvr32.exe", &["/s", &file_name]);
    if !process.is_successful() {
        return Err(InstallError::new(format!(
            "Failed to register ActiveX plugin: error code {}\n\n{}",
            process.exit_code, process.output
        )));
    }

    Ok(())
}

/// Create a Windows shortcut (.lnk) using a PowerShell one-liner.
fn create_shortcut(
    folder: &Path,
    executable: &Path,
    name: &str,
    description: &str,
) -> Result<(), InstallError> {
    let lnk_path = folder.join(format!("{}.lnk", name));
    let exe_str = executable.to_string_lossy();
    let lnk_str = lnk_path.to_string_lossy();

    // Use PowerShell to create the shortcut via WScript.Shell COM.
    let script = format!(
        "$ws = New-Object -ComObject WScript.Shell; \
         $s = $ws.CreateShortcut('{}'); \
         $s.TargetPath = '{}'; \
         $s.Description = '{}'; \
         $s.IconLocation = '{}'; \
         $s.Save()",
        lnk_str, exe_str, description, exe_str
    );

    let result = process_utils::run_process("powershell.exe", &["-NoProfile", "-Command", &script]);
    if !result.is_successful() {
        return Err(InstallError::new(format!(
            "Failed to create shortcut: {}",
            result.output
        )));
    }
    Ok(())
}

/// Extract the embedded 7z archive and install files to the correct locations.
fn install_from_archive(
    archive_bytes: &[u8],
    form: &dyn ProgressCallback,
    flags: &mut InstallFlags,
) -> Result<(), InstallError> {
    let si = system_info::SystemInfo::new();
    let flash32_path = si.flash32_path.clone();
    let flash64_path = si.flash64_path.clone();
    let system32_path = si.system32_path.clone();
    let flash_program32_path = si.flash_program32_path.clone();
    let native_host_dir = native_host::get_native_host_install_dir();

    let mut registry_to_apply: Vec<&str> = vec![resources::INSTALL_GENERAL];

    if si.is_64bit {
        flags.set_flag(install_flags::X64);
        registry_to_apply.push(resources::INSTALL_GENERAL_64);
    }

    let entries: Vec<(&str, InstallEntry)> = vec![
        (
            "controlpanel",
            InstallEntry {
                install_text: "Installing Flash Player utilities...",
                required_flags: install_flags::NONE,
                target_directory: system32_path.clone(),
                registry_instructions: None,
            },
        ),
        (
            "uninstaller",
            InstallEntry {
                install_text: "Extracting uninstaller...",
                required_flags: install_flags::NONE,
                target_directory: flash_program32_path.clone(),
                registry_instructions: None,
            },
        ),
        (
            "standalone",
            InstallEntry {
                install_text: "Installing 32-bit Standalone Flash Player...",
                required_flags: install_flags::PLAYER,
                target_directory: flash_program32_path.clone(),
                registry_instructions: None,
            },
        ),
        (
            "ocx32",
            InstallEntry {
                install_text: "Installing 32-bit Flash Player for Internet Explorer...",
                required_flags: install_flags::ACTIVEX,
                target_directory: flash32_path.clone(),
                registry_instructions: None,
            },
        ),
        (
            "np32",
            InstallEntry {
                install_text: "Installing 32-bit Flash Player for Firefox...",
                required_flags: install_flags::NETSCAPE,
                target_directory: flash32_path.clone(),
                registry_instructions: Some(resources::INSTALL_NP),
            },
        ),
        (
            "pp32",
            InstallEntry {
                install_text: "Installing 32-bit Flash Player for Chrome...",
                required_flags: install_flags::PEPPER,
                target_directory: flash32_path.clone(),
                registry_instructions: Some(resources::INSTALL_PP),
            },
        ),
        (
            "ocx64",
            InstallEntry {
                install_text: "Installing 64-bit Flash Player for Internet Explorer...",
                required_flags: install_flags::ACTIVEX | install_flags::X64,
                target_directory: flash64_path.clone(),
                registry_instructions: None,
            },
        ),
        (
            "np64",
            InstallEntry {
                install_text: "Installing 64-bit Flash Player for Firefox...",
                required_flags: install_flags::NETSCAPE | install_flags::X64,
                target_directory: flash64_path.clone(),
                registry_instructions: Some(resources::INSTALL_NP_64),
            },
        ),
        (
            "pp64",
            InstallEntry {
                install_text: "Installing 64-bit Flash Player for Chrome...",
                required_flags: install_flags::PEPPER | install_flags::X64,
                target_directory: flash64_path.clone(),
                registry_instructions: Some(resources::INSTALL_PP_64),
            },
        ),
        (
            "native-host-64",
            InstallEntry {
                install_text: "Installing native messaging host (64-bit)...",
                required_flags: install_flags::NATIVE_HOST | install_flags::X64,
                target_directory: native_host_dir.clone(),
                registry_instructions: None,
            },
        ),
        (
            "native-host-32",
            InstallEntry {
                install_text: "Installing native messaging host (32-bit)...",
                required_flags: install_flags::NATIVE_HOST,
                target_directory: native_host_dir.clone(),
                registry_instructions: None,
            },
        ),
    ];

    let legacy = si.is_legacy_windows();

    // Extract archive using sevenz-rust2.
    sevenz_rust2::decompress_with_extract_fn(
        Cursor::new(archive_bytes),
        ".",
        |entry, reader, _dest| {
            // Skip directory entries — they have no file content.
            if entry.is_directory() {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            let entry_name = entry.name().to_string();
            let parts: Vec<&str> = entry_name.split('/').collect();
            if parts.is_empty() {
                return Ok(true);
            }

            let filename = parts[0];
            let install_key = filename.split('-').next().unwrap_or(filename);
            let is_pp32 = install_key == "pp32";
            let is_pp64 = install_key == "pp64";
            let should_extract_pepper_for_native_host =
                flags.is_set(install_flags::NATIVE_HOST)
                    && ((si.is_64bit && is_pp64) || (!si.is_64bit && is_pp32));

            // Find the matching entry: try exact match on full dirname first,
            // then fall back to first segment before '-'.
            let Some((_key, install_entry)) = entries
                .iter()
                .find(|(k, _)| *k == filename)
                .or_else(|| entries.iter().find(|(k, _)| *k == install_key))
            else {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            };

            // Check required flags.
            let should_install_by_flags = install_entry.required_flags == install_flags::NONE
                || flags.is_set(install_entry.required_flags);
            if install_entry.required_flags != install_flags::NONE
                && !flags.is_set(install_entry.required_flags)
                && !(should_extract_pepper_for_native_host && (is_pp32 || is_pp64))
            {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            // For native-host: on 64-bit use only native-host-64, on 32-bit use only native-host-32.
            if filename == "native-host-32" && si.is_64bit {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }
            if filename == "native-host-64" && !si.is_64bit {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            // Check debug flag match (skip native-host entries from this check).
            if install_entry.required_flags != install_flags::NONE
                && (install_entry.required_flags & install_flags::NATIVE_HOST) == 0
            {
                let is_debug_file = filename.contains("-debug");
                if flags.is_set(install_flags::DEBUG) != is_debug_file {
                    io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                    return Ok(true);
                }
            }

            // Check legacy flag for ActiveX entries.
            if (install_entry.required_flags & install_flags::ACTIVEX) != 0 {
                let is_legacy_file = filename.contains("-legacy");
                if legacy != is_legacy_file {
                    io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                    return Ok(true);
                }
            }

            let extract_for_native_host_only = !should_install_by_flags
                && should_extract_pepper_for_native_host
                && (is_pp32 || is_pp64);

            if extract_for_native_host_only {
                form.update_progress_label(
                    "Extracting Pepper files required by modern browser support...",
                    true,
                );
            } else {
                form.update_progress_label(install_entry.install_text, true);
            }

            let target_directory = if extract_for_native_host_only {
                native_host_dir.clone()
            } else {
                install_entry.target_directory.clone()
            };

            // Ensure target directory exists.
            let _ = fs::create_dir_all(&target_directory);

            // Extract file: use just the file name (strip any path prefix).
            let out_name = parts.last().unwrap_or(&filename);
            let out_path = target_directory.join(out_name);

            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).map_err(sevenz_rust2::Error::from)?;
            fs::write(&out_path, &buf).map_err(sevenz_rust2::Error::from)?;

            Ok(true)
        },
    )
    .map_err(|e| InstallError::new(format!("Failed to extract archive: {}", e)))?;

    // Create Player shortcuts.
    if flags.is_set(install_flags::PLAYER) {
        let is_debug = flags.is_set(install_flags::DEBUG);
        let name = if is_debug {
            "Flash Player (Debug)"
        } else {
            "Flash Player"
        };
        let description = format!(
            "Standalone Flash Player {}{}",
            clean_flash_common::update_checker::FLASH_VERSION,
            if is_debug { " (Debug)" } else { "" }
        );
        let exe_name = if is_debug {
            "flashplayer_sa_debug.exe"
        } else {
            "flashplayer_sa.exe"
        };
        let executable = flash_program32_path.join(exe_name);

        if flags.is_set(install_flags::PLAYER_START_MENU) {
            if let Some(start_menu) = get_start_menu() {
                let _ = create_shortcut(&start_menu, &executable, name, &description);
            }
        }

        if flags.is_set(install_flags::PLAYER_DESKTOP) {
            if let Some(desktop) = get_desktop() {
                let _ = create_shortcut(&desktop, &executable, name, &description);
            }
        }
    }

    // Collect registry entries for enabled components.
    for (_key, entry) in &entries {
        if flags.is_set(entry.required_flags) {
            if let Some(reg) = entry.registry_instructions {
                registry_to_apply.push(reg);
            }
        }
    }

    form.update_progress_label("Applying registry changes...", true);
    let refs: Vec<&str> = registry_to_apply.iter().copied().collect();
    registry::apply_registry(&refs)?;

    // Register ActiveX OCX files.
    if flags.is_set(install_flags::ACTIVEX) {
        form.update_progress_label(
            "Activating 32-bit Flash Player for Internet Explorer...",
            true,
        );
        let ocx32 = flash32_path.join(format!("Flash32_{}.ocx", si.version_path));
        register_activex(&ocx32.to_string_lossy())?;

        if si.is_64bit {
            form.update_progress_label(
                "Activating 64-bit Flash Player for Internet Explorer...",
                true,
            );
            let ocx64 = flash64_path.join(format!("Flash64_{}.ocx", si.version_path));
            register_activex(&ocx64.to_string_lossy())?;
        }
    }

    // Install native messaging host manifests for detected browsers.
    if flags.is_set(install_flags::NATIVE_HOST) {
        native_host::install_native_host(form)?;
    }

    Ok(())
}

/// Main install entry point.
pub fn install(
    form: &dyn ProgressCallback,
    flags: &mut InstallFlags,
) -> Result<(), InstallError> {
    if flags.is_none_set() {
        return Ok(());
    }

    // The 7z archive is embedded in the binary via include_bytes!.
    // For the port, we expect it at a known resource path; if not present,
    // this is a no-op placeholder.
    let archive_bytes: &[u8] = include_bytes!("../cleanflash-win.7z");

    if archive_bytes.is_empty() {
        // Nothing to extract; still apply the rest of the steps.
        return Ok(());
    }

    install_from_archive(archive_bytes, form, flags)
}

fn get_start_menu() -> Option<PathBuf> {
    env::var("APPDATA")
        .ok()
        .map(|p| {
            PathBuf::from(p)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
        })
}

fn get_desktop() -> Option<PathBuf> {
    env::var("USERPROFILE")
        .ok()
        .map(|p| PathBuf::from(p).join("Desktop"))
}
