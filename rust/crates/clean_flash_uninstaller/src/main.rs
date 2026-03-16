#![windows_subsystem = "windows"]

mod uninstall_form;

use clean_flash_ui::renderer::Renderer;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use uninstall_form::{UninstallForm, HEIGHT, WIDTH};

fn main() {
    let title = format!(
        "Clean Flash Player {} Uninstaller",
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

    window.set_target_fps(60);

    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut form = UninstallForm::new();

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
