use super::Rect;
use crate::renderer::Renderer;

/// A smooth gradient progress bar matching the C# SmoothProgressBar control.
pub struct ProgressBar {
    pub rect: Rect,
    pub minimum: i32,
    pub maximum: i32,
    pub value: i32,
    pub color1: u32,
    pub color2: u32,
    pub visible: bool,
}

impl ProgressBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            minimum: 0,
            maximum: 100,
            value: 0,
            color1: Renderer::rgb(97, 147, 232),
            color2: Renderer::rgb(28, 99, 232),
            visible: true,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        if !self.visible {
            return;
        }
        let range = (self.maximum - self.minimum).max(1) as f32;
        let percent = (self.value - self.minimum) as f32 / range;
        let fill_w = (self.rect.w as f32 * percent) as i32;

        if fill_w > 0 {
            renderer.fill_gradient_h(
                self.rect.x,
                self.rect.y,
                fill_w,
                self.rect.h,
                self.color1,
                self.color2,
            );
        }

        // 3-D border.
        let r = self.rect;
        let dark = Renderer::rgb(105, 105, 105);
        let light = Renderer::rgb(255, 255, 255);
        // Top
        for dx in 0..r.w {
            renderer.set_pixel(r.x + dx, r.y, dark);
        }
        // Left
        for dy in 0..r.h {
            renderer.set_pixel(r.x, r.y + dy, dark);
        }
        // Bottom
        for dx in 0..r.w {
            renderer.set_pixel(r.x + dx, r.y + r.h - 1, light);
        }
        // Right
        for dy in 0..r.h {
            renderer.set_pixel(r.x + r.w - 1, r.y + dy, light);
        }
    }
}
