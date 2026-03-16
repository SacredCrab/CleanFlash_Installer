use ab_glyph::{point, Font, FontRef, ScaleFont};
use crate::renderer::Renderer;

/// Manages loaded fonts and provides text measurement / rendering.
pub struct FontManager {
    regular: FontRef<'static>,
}

impl FontManager {
    /// Create a `FontManager` using the bundled Liberation Sans font.
    pub fn new() -> Self {
        let regular = FontRef::try_from_slice(include_bytes!(
            "../../../resources/liberation-sans.regular.ttf"
        ))
        .expect("Failed to parse bundled Liberation Sans font");
        Self { regular }
    }

    /// Measure the width (in pixels) of `text` at the given `size` (in px).
    pub fn measure_text(&self, text: &str, size: f32) -> (f32, f32) {
        let scaled = self.regular.as_scaled(size);
        let mut width: f32 = 0.0;
        let height = scaled.height();

        let mut last_glyph_id = None;

        for ch in text.chars() {
            let glyph_id = scaled.glyph_id(ch);
            if let Some(prev) = last_glyph_id {
                width += scaled.kern(prev, glyph_id);
            }
            width += scaled.h_advance(glyph_id);
            last_glyph_id = Some(glyph_id);
        }

        (width, height)
    }

    /// Draw `text` onto the renderer at (x, y) with the given pixel size and colour.
    pub fn draw_text(
        &self,
        renderer: &mut Renderer,
        x: i32,
        y: i32,
        text: &str,
        size: f32,
        color: u32,
    ) {
        let scaled = self.regular.as_scaled(size);
        let ascent = scaled.ascent();
        let mut cursor_x: f32 = 0.0;
        let mut last_glyph_id = None;

        for ch in text.chars() {
            let glyph_id = scaled.glyph_id(ch);
            if let Some(prev) = last_glyph_id {
                cursor_x += scaled.kern(prev, glyph_id);
            }

            let glyph = glyph_id.with_scale_and_position(
                size,
                point(x as f32 + cursor_x, y as f32 + ascent),
            );

            if let Some(outlined) = self.regular.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|gx, gy, coverage| {
                    let px = bounds.min.x as i32 + gx as i32;
                    let py = bounds.min.y as i32 + gy as i32;
                    let alpha = (coverage * 255.0) as u8;
                    if alpha > 0 {
                        renderer.blend_pixel(px, py, color, alpha);
                    }
                });
            }

            cursor_x += scaled.h_advance(glyph_id);
            last_glyph_id = Some(glyph_id);
        }
    }

    /// Draw multiline text, splitting on '\n'. Returns total height drawn.
    pub fn draw_text_multiline(
        &self,
        renderer: &mut Renderer,
        x: i32,
        y: i32,
        text: &str,
        size: f32,
        color: u32,
        line_spacing: f32,
    ) -> f32 {
        let scaled = self.regular.as_scaled(size);
        let line_height = scaled.height() + line_spacing;
        let mut cy = y as f32;

        for line in text.split('\n') {
            self.draw_text(renderer, x, cy as i32, line, size, color);
            cy += line_height;
        }

        cy - y as f32
    }
}


