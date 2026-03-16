#![windows_subsystem = "windows"]

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

    let mut window = Window::new(
        &title,
        WIDTH,
        HEIGHT,
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

    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut form = InstallForm::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (mx, my) = window
            .get_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let mouse_down = window.get_mouse_down(MouseButton::Left);

        form.update_and_draw(&mut renderer, mx as i32, my as i32, mouse_down);

        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .expect("Failed to update window buffer");
    }
}
