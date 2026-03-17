use crate::install_flags::{self, InstallFlags};
use clean_flash_common::{native_host, InstallError, ProgressCallback};
use std::fs;
use std::io::{self, Cursor};

/// Extract native-host and pp64 entries from the embedded 7z archive,
/// placing them next to each other in the native host install directory.
fn install_from_archive(
    archive_bytes: &[u8],
    form: &dyn ProgressCallback,
) -> Result<(), InstallError> {
    let native_host_dir = native_host::get_native_host_install_dir();
    let _ = fs::create_dir_all(&native_host_dir);

    form.update_progress_label("Extracting native messaging host files...", true);

    sevenz_rust2::decompress_with_extract_fn(
        Cursor::new(archive_bytes),
        ".",
        |entry, reader, _dest| {
            if entry.is_directory() {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            let entry_name = entry.name().to_string();
            let parts: Vec<&str> = entry_name.split('/').collect();
            if parts.is_empty() {
                return Ok(true);
            }

            let dirname = parts[0];
            let install_key = dirname.split('-').next().unwrap_or(dirname);

            // Only extract native-host-64, pp64, and uninstaller entries.
            let dominated = dirname == "native-host-64"
                || install_key == "pp64"
                || install_key == "uninstaller";
            if !dominated {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            // Skip debug variants of pp64.
            if install_key == "pp64" && dirname.contains("-debug") {
                io::copy(reader, &mut io::sink()).map_err(sevenz_rust2::Error::from)?;
                return Ok(true);
            }

            if dirname == "native-host-64" {
                form.update_progress_label("Installing native messaging host...", true);
            } else if install_key == "pp64" {
                form.update_progress_label("Installing Pepper plugin files...", true);
            } else if install_key == "uninstaller" {
                form.update_progress_label("Extracting uninstaller...", true);
            }

            let out_name = parts.last().unwrap_or(&dirname);
            let out_path = native_host_dir.join(out_name);

            let mut buf = Vec::new();
            reader
                .read_to_end(&mut buf)
                .map_err(sevenz_rust2::Error::from)?;
            fs::write(&out_path, &buf).map_err(sevenz_rust2::Error::from)?;

            // Make binaries executable.
            #[cfg(unix)]
            if dirname == "native-host-64" || install_key == "uninstaller" {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&out_path, fs::Permissions::from_mode(0o755));
            }

            Ok(true)
        },
    )
    .map_err(|e| InstallError::new(format!("Failed to extract archive: {}", e)))?;

    Ok(())
}

/// Main install entry point for Linux.
pub fn install(
    form: &dyn ProgressCallback,
    flags: &mut InstallFlags,
) -> Result<(), InstallError> {
    if flags.is_none_set() {
        return Ok(());
    }

    let archive_bytes: &[u8] = include_bytes!("../cleanflash.7z");

    if archive_bytes.is_empty() {
        return Ok(());
    }

    if flags.is_set(install_flags::NATIVE_HOST) {
        install_from_archive(archive_bytes, form)?;
        native_host::install_native_host(form)?;
    }

    Ok(())
}
