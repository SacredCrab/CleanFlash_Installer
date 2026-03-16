use super::Rect;
use crate::font::FontManager;
use crate::renderer::Renderer;

/// Simple static label for drawing text with optional word-wrapping.
pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub color: u32,
    pub font_size: f32,
    pub line_spacing: f32,
    pub visible: bool,
    /// If > 0, text is word-wrapped to this pixel width.
    pub max_width: f32,
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
            max_width: 0.0,
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
            self.max_width,
        );
    }

    /// Check if a click at (mx, my) is within a rough bounding box of the label text.
    pub fn clicked(&self, mx: i32, my: i32, mouse_released: bool, fonts: &FontManager) -> bool {
        if !self.visible || !mouse_released {
            return false;
        }
        let (tw, th) = fonts.measure_text_multiline(
            &self.text,
            self.font_size,
            self.line_spacing,
            self.max_width,
        );
        let r = Rect::new(self.rect.x, self.rect.y, tw as i32 + 5, th as i32);
        r.contains(mx, my)
    }
}
