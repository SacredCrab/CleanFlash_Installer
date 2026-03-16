/// Bitflag-based install options, mirroring the C# InstallFlags class.

pub const NONE: u32 = 0;
pub const PEPPER: u32 = 1 << 0;
pub const NETSCAPE: u32 = 1 << 1;
pub const ACTIVEX: u32 = 1 << 2;
pub const PLAYER: u32 = 1 << 3;
pub const PLAYER_START_MENU: u32 = 1 << 4;
pub const PLAYER_DESKTOP: u32 = 1 << 5;
pub const X64: u32 = 1 << 6;
pub const DEBUG: u32 = 1 << 7;

const UNINSTALL_TICKS: u32 = 9;
const INSTALL_GENERAL_TICKS: u32 = 5;

#[derive(Clone, Copy)]
pub struct InstallFlags {
    value: u32,
}

impl InstallFlags {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn from(value: u32) -> Self {
        Self { value }
    }

    pub fn get_value(self) -> u32 {
        self.value
    }

    pub fn is_set(self, flag: u32) -> bool {
        (self.value & flag) == flag
    }

    pub fn is_none_set(self) -> bool {
        self.value == 0
    }

    pub fn set_flag(&mut self, flag: u32) {
        self.value |= flag;
    }

    pub fn set_conditionally(&mut self, condition: bool, flag: u32) {
        if condition {
            self.set_flag(flag);
        }
    }

    pub fn get_ticks(self) -> u32 {
        let is_64bit = cfg!(target_pointer_width = "64")
            || std::env::var("PROCESSOR_ARCHITEW6432").is_ok();

        let mut ticks = (if self.is_set(PEPPER) { 1 } else { 0 })
            + (if self.is_set(NETSCAPE) { 1 } else { 0 })
            + (if self.is_set(ACTIVEX) { 2 } else { 0 });

        if is_64bit {
            ticks *= 2;
        }

        if self.is_set(PLAYER) {
            ticks += 1;
        }

        ticks += UNINSTALL_TICKS;
        ticks += INSTALL_GENERAL_TICKS;
        ticks
    }
}
