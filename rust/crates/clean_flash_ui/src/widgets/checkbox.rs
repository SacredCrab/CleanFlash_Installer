use super::Rect;
use crate::renderer::Renderer;

/// A software-rendered checkbox with a gray gradient, black outline, and white checkmark.
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

    pub fn draw(&self, renderer: &mut Renderer) {
        if !self.visible {
            return;
        }

        let r = self.rect;

        // Gray gradient background (matching button style).
        let color1 = Renderer::rgb(118, 118, 118);
        let color2 = Renderer::rgb(81, 81, 81);
        renderer.fill_gradient_v(r.x, r.y, r.w, r.h, color1, color2);

        // Black 1px outline.
        renderer.draw_rect(r.x, r.y, r.w, r.h, Renderer::rgb(0, 0, 0));

        // White checkmark when checked.
        if self.checked {
            let white = Renderer::rgb(255, 255, 255);
            Self::draw_checkmark(renderer, r.x, r.y, r.w, r.h, white);
        }

        if !self.enabled {
            // Dim overlay.
            for dy in 0..r.h {
                for dx in 0..r.w {
                    renderer.blend_pixel(
                        r.x + dx,
                        r.y + dy,
                        Renderer::rgb(50, 51, 51),
                        100,
                    );
                }
            }
        }
    }

    /// Draw a checkmark scaled to fit within the given box.
    fn draw_checkmark(renderer: &mut Renderer, bx: i32, by: i32, bw: i32, bh: i32, color: u32) {
        // Checkmark geometry defined in a normalised coordinate space.
        // The check goes from bottom-left, down to a valley, then up to top-right.
        // Key points (in fractions of size):
        //   start:  (0.20, 0.50)
        //   valley: (0.40, 0.72)
        //   end:    (0.80, 0.25)
        // We draw two thick line segments between these points.

        let w = bw as f32;
        let h = bh as f32;

        // Absolute coordinates of the three key points.
        let x0 = bx as f32 + w * 0.20;
        let y0 = by as f32 + h * 0.48;
        let x1 = bx as f32 + w * 0.40;
        let y1 = by as f32 + h * 0.72;
        let x2 = bx as f32 + w * 0.80;
        let y2 = by as f32 + h * 0.25;

        // Line thickness scales with checkbox size.
        let thickness = (w * 0.14).max(1.5);

        draw_thick_line(renderer, x0, y0, x1, y1, thickness, color);
        draw_thick_line(renderer, x1, y1, x2, y2, thickness, color);
    }
}

/// Draw a thick line between two points using filled circles along the path (round caps).
fn draw_thick_line(
    renderer: &mut Renderer,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: f32,
    color: u32,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let dist = (dx * dx + dy * dy).sqrt();
    let steps = (dist * 2.0).ceil() as i32;
    let half = thickness / 2.0;

    for i in 0..=steps {
        let t = i as f32 / steps.max(1) as f32;
        let cx = x0 + dx * t;
        let cy = y0 + dy * t;

        // Fill a small circle at (cx, cy).
        let min_x = (cx - half).floor() as i32;
        let max_x = (cx + half).ceil() as i32;
        let min_y = (cy - half).floor() as i32;
        let max_y = (cy + half).ceil() as i32;
        let r_sq = half * half;

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fdx = px as f32 + 0.5 - cx;
                let fdy = py as f32 + 0.5 - cy;
                if fdx * fdx + fdy * fdy <= r_sq {
                    renderer.set_pixel(px, py, color);
                }
            }
        }
    }
}
