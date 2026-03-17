use crate::{native_host, InstallError, ProgressCallback};
use std::fs;

/// Perform the full uninstallation sequence on Linux.
///
/// This removes the native messaging host binary, the Pepper (pp64) files
/// installed alongside it, and the browser manifests.
pub fn uninstall(form: &dyn ProgressCallback) -> Result<(), InstallError> {
    form.update_progress_label("Removing native messaging host...", true);
    native_host::uninstall_native_host(form);

    // Remove the entire install directory (host binary + pp64 files).
    let install_dir = native_host::get_native_host_install_dir();
    if install_dir.exists() {
        form.update_progress_label("Removing installed files...", true);
        let _ = fs::remove_dir_all(&install_dir);
    }

    Ok(())
}
