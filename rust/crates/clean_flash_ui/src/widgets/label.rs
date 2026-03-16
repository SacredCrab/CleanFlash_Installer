use super::Rect;
use crate::font::FontManager;
use crate::renderer::Renderer;

/// Simple static label for drawing text.
pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub color: u32,
    pub font_size: f32,
    pub line_spacing: f32,
    pub visible: bool,
}

impl Label {
    pub fn new(x: i32, y: i32, text: &str, font_size: f32) -> Self {
        Self {
            rect: Rect::new(x, y, 0, 0),
            text: text.to_string(),
            color: Renderer::rgb(245, 245, 245),
            font_size,
            line_spacing: 2.0,
            visible: true,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, fonts: &FontManager) {
        if !self.visible || self.text.is_empty() {
            return;
        }
        fonts.draw_text_multiline(
            renderer,
            self.rect.x,
            self.rect.y,
            &self.text,
            self.font_size,
            self.color,
            self.line_spacing,
        );
    }

    /// Check if a click at (mx, my) is within a rough bounding box of the label text.
    pub fn clicked(&self, mx: i32, my: i32, mouse_released: bool, fonts: &FontManager) -> bool {
        if !self.visible || !mouse_released {
            return false;
        }
        let (tw, _th) = fonts.measure_text(&self.text, self.font_size);
        let lines = self.text.lines().count().max(1);
        let approx_h = (self.font_size * lines as f32 + 2.0 * lines as f32) as i32;
        let r = Rect::new(self.rect.x, self.rect.y, tw as i32 + 5, approx_h);
        r.contains(mx, my)
    }
}
