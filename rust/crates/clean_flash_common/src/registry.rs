use crate::{process_utils, system_info, InstallError};
use std::fs;
use std::io::Write;

/// Apply registry contents by writing a .reg file and importing with reg.exe.
#[cfg(windows)]
pub fn apply_registry(entries: &[&str]) -> Result<(), InstallError> {
    let combined = entries.join("\n\n");
    let filled = system_info::fill_string(&combined);

    let content = format!("Windows Registry Editor Version 5.00\n\n{}", filled);

    let temp_dir = std::env::temp_dir();
    // Include the process ID to avoid collisions when two instances run concurrently,
    // matching the unique-file guarantee of C#'s Path.GetTempFileName().
    let reg_file = temp_dir.join(format!("cleanflash_reg_{}.tmp", std::process::id()));

    // Write as UTF-16LE with BOM (Windows .reg format).
    {
        let mut f = fs::File::create(&reg_file)
            .map_err(|e| InstallError::new(format!("Failed to create temp reg file: {}", e)))?;
        let utf16: Vec<u16> = content.encode_utf16().collect();
        // BOM
        f.write_all(&[0xFF, 0xFE])
            .map_err(|e| InstallError::new(format!("Failed to write BOM: {}", e)))?;
        for word in &utf16 {
            f.write_all(&word.to_le_bytes())
                .map_err(|e| InstallError::new(format!("Failed to write reg data: {}", e)))?;
        }
    }

    let reg_filename = reg_file.to_string_lossy().to_string();
    let result = process_utils::run_process("reg.exe", &["import", &reg_filename]);

    let _ = fs::remove_file(&reg_file);

    if !result.is_successful() {
        return Err(InstallError::new(format!(
            "Failed to apply changes to registry: error code {}\n\n{}",
            result.exit_code, result.output
        )));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn apply_registry(_entries: &[&str]) -> Result<(), InstallError> {
    Ok(())
}