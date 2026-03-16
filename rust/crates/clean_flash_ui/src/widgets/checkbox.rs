use super::Rect;
use crate::renderer::{Renderer, RgbaImage};

/// An image-based checkbox matching the C# ImageCheckBox control.
pub struct ImageCheckBox {
    pub rect: Rect,
    pub checked: bool,
    pub enabled: bool,
    pub visible: bool,
}

impl ImageCheckBox {
    pub fn new(x: i32, y: i32) -> Self {
        Self::with_size(x, y, 21)
    }

    pub fn with_size(x: i32, y: i32, size: i32) -> Self {
        Self {
            rect: Rect::new(x, y, size, size),
            checked: true,
            enabled: true,
            visible: true,
        }
    }

    pub fn toggle_if_clicked(&mut self, mx: i32, my: i32, mouse_released: bool) -> bool {
        if self.visible && self.enabled && self.rect.contains(mx, my) && mouse_released {
            self.checked = !self.checked;
            true
        } else {
            false
        }
    }

    pub fn draw(
        &self,
        renderer: &mut Renderer,
        checked_img: &RgbaImage,
        unchecked_img: &RgbaImage,
    ) {
        if !self.visible {
            return;
        }
        let img = if self.checked {
            checked_img
        } else {
            unchecked_img
        };
        if img.width > 0 && img.height > 0 {
            renderer.draw_image_scaled(self.rect.x, self.rect.y, self.rect.w, self.rect.h, img);
        } else {
            // Fallback: draw a simple square.
            let bg = if self.checked {
                Renderer::rgb(97, 147, 232)
            } else {
                Renderer::rgb(80, 80, 80)
            };
            renderer.fill_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg);
            renderer.draw_rect(
                self.rect.x,
                self.rect.y,
                self.rect.w,
                self.rect.h,
                Renderer::rgb(160, 160, 160),
            );
            if self.checked {
                // Draw a simple checkmark.
                let cx = self.rect.x + 5;
                let cy = self.rect.y + 10;
                for i in 0..4 {
                    renderer.set_pixel(cx + i, cy + i, Renderer::rgb(255, 255, 255));
                    renderer.set_pixel(cx + i, cy + i + 1, Renderer::rgb(255, 255, 255));
                }
                for i in 0..8 {
                    renderer.set_pixel(cx + 3 + i, cy + 3 - i, Renderer::rgb(255, 255, 255));
                    renderer.set_pixel(cx + 3 + i, cy + 4 - i, Renderer::rgb(255, 255, 255));
                }
            }
        }

        if !self.enabled {
            // Dim overlay.
            for dy in 0..self.rect.h {
                for dx in 0..self.rect.w {
                    renderer.blend_pixel(
                        self.rect.x + dx,
                        self.rect.y + dy,
                        Renderer::rgb(50, 51, 51),
                        100,
                    );
                }
            }
        }
    }
}
