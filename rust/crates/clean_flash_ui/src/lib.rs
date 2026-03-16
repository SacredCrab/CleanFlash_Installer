pub mod flash_logo;
pub mod font;
pub mod renderer;
pub mod widgets;

pub use font::FontManager;
pub use renderer::Renderer;

/// Query the system DPI scale factor (1.0 = 100%, 1.5 = 150%, 2.0 = 200%).
/// The process must already be DPI-aware (declared in the manifest) for this
/// to return the true hardware DPI rather than the virtualised 96.
pub fn get_dpi_scale() -> f32 {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::UI::HiDpi::GetDpiForSystem;
        let dpi = GetDpiForSystem();
        if dpi == 0 { 1.0 } else { dpi as f32 / 96.0 }
    }
    #[cfg(not(target_os = "windows"))]
    { 1.0 }
}

/// Set the window icon from the icon resource already embedded in the binary
/// by `winresource` (resource ID 1). No-op on non-Windows platforms.
pub fn set_window_icon(window: &minifb::Window) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            LoadImageW, SendMessageW, ICON_BIG, ICON_SMALL, IMAGE_ICON, LR_DEFAULTSIZE,
            WM_SETICON,
        };

        let hwnd = window.get_window_handle(); // *mut c_void
        let hmodule = GetModuleHandleW(std::ptr::null());
        // MAKEINTRESOURCEW(1): load the icon embedded by winresource as resource ID 1.
        let icon = LoadImageW(hmodule, 1 as *const u16, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE);
        if !icon.is_null() {
            SendMessageW(hwnd, WM_SETICON, ICON_BIG as usize, icon as isize);
            SendMessageW(hwnd, WM_SETICON, ICON_SMALL as usize, icon as isize);
        }
    }
}
