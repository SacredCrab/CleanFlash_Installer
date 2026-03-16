//#![windows_subsystem = "windows"]

mod install_flags;
mod install_form;
mod installer;

use install_form::{InstallForm, HEIGHT, WIDTH};
use clean_flash_ui::renderer::Renderer;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

fn main() {
    let title = format!(
        "Clean Flash Player {} Installer",
        clean_flash_common::update_checker::FLASH_VERSION
    );

    // Query DPI before creating any window. The manifest already marks the process
    // as per-monitor v2 DPI aware, so GetDpiForSystem returns the real hardware DPI.
    let scale = clean_flash_ui::get_dpi_scale();
    let phys_w = (WIDTH as f32 * scale).round() as usize;
    let phys_h = (HEIGHT as f32 * scale).round() as usize;

    let mut window = Window::new(
        &title,
        phys_w,
        phys_h,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create window");

    // Set window icon from the resource embedded by build.rs.
    clean_flash_ui::set_window_icon(&window);

    // Cap at ~60 fps.
    window.set_target_fps(60);

    // Renderer operates at physical resolution; the form layout is scaled accordingly.
    let mut renderer = Renderer::new(phys_w, phys_h);
    let mut form = InstallForm::new(scale);

    while window.is_open() && !window.is_key_down(Key::Escape)
        && !(window.is_key_down(Key::F4)
            && (window.is_key_down(Key::LeftAlt) || window.is_key_down(Key::RightAlt)))
    {
        let (mx, my) = window
            .get_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let mouse_down = window.get_mouse_down(MouseButton::Left);

        form.update_and_draw(&mut renderer, mx as i32, my as i32, mouse_down);

        window
            .update_with_buffer(&renderer.buffer, phys_w, phys_h)
            .expect("Failed to update window buffer");
    }
}
