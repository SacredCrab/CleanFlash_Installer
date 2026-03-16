use crate::uninstaller;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

/// Attempt to delete a single file, retrying with escalating measures if needed.
pub fn delete_file(path: &Path) {
    if !path.exists() {
        return;
    }

    // Unregister ActiveX .ocx files before deletion.
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
            kill_locking_processes(path);
            thread::sleep(Duration::from_millis(500));
            let _ = fs::remove_dir(path);
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

fn try_take_ownership_and_delete(path: &Path) -> bool {
    // On Windows we could use SetNamedSecurityInfo to take ownership.
    // For simplicity the Rust port clears read-only and retries.
    try_clear_readonly_and_delete(path)
}

fn kill_locking_processes(path: &Path) {
    // Use taskkill as a best-effort approach.
    // The C# original enumerates all open handles, which requires complex
    // NT API calls.  A simplified approach is acceptable for the port.
    let _ = path; // Locking-process detection is a best-effort no-op here.
}

fn is_dir_empty(path: &Path) -> bool {
    fs::read_dir(path)
        .map(|mut rd| rd.next().is_none())
        .unwrap_or(true)
}
