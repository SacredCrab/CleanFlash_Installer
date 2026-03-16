pub mod file_util;
pub mod process_utils;
pub mod redirection;
pub mod registry;
pub mod resources;
pub mod system_info;
pub mod uninstaller;
pub mod update_checker;
pub mod winapi_helpers;

use std::fmt;

/// Progress callback trait, analogous to C# IProgressForm.
pub trait ProgressCallback: Send {
    fn update_progress_label(&self, text: &str, tick: bool);
    fn tick_progress(&self);
}

/// Install/uninstall error type.
#[derive(Debug)]
pub struct InstallError {
    pub message: String,
}

impl InstallError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InstallError {}

/// Result of running an external process.
pub struct ExitedProcess {
    pub exit_code: i32,
    pub output: String,
}

impl ExitedProcess {
    pub fn is_successful(&self) -> bool {
        self.exit_code == 0
    }
}
