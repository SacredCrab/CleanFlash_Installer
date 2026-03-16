pub mod font;
pub mod renderer;
pub mod widgets;

pub use font::FontManager;
pub use renderer::Renderer;

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
