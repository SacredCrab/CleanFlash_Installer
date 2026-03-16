use super::Rect;
use crate::font::FontManager;
use crate::renderer::Renderer;

/// A gradient button matching the C# GradientButton control.
pub struct GradientButton {
    pub rect: Rect,
    pub text: String,
    pub font_size: f32,
    pub color1: u32,
    pub color2: u32,
    pub back_color: u32,
    pub fore_color: u32,
    pub hover_alpha: f64,
    pub disable_alpha: f64,
    pub enabled: bool,
    pub visible: bool,
    pub hovered: bool,
    pub pressed: bool,
}

impl GradientButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, text: &str) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            text: text.to_string(),
            font_size: 13.0,
            color1: Renderer::rgb(118, 118, 118),
            color2: Renderer::rgb(81, 81, 81),
            back_color: Renderer::rgb(0, 0, 0),
            fore_color: Renderer::rgb(227, 227, 227),
            hover_alpha: 0.875,
            disable_alpha: 0.644,
            enabled: true,
            visible: true,
            hovered: false,
            pressed: false,
        }
    }

    /// Update hover / pressed state from mouse position and button state.
    pub fn update(&mut self, mx: i32, my: i32, mouse_down: bool) {
        if !self.visible || !self.enabled {
            self.hovered = false;
            self.pressed = false;
            return;
        }
        self.hovered = self.rect.contains(mx, my);
        self.pressed = self.hovered && mouse_down;
    }

    /// Returns true if mouse was just released inside this button.
    pub fn clicked(&self, mx: i32, my: i32, mouse_released: bool) -> bool {
        self.visible && self.enabled && self.rect.contains(mx, my) && mouse_released
    }

    pub fn draw(&self, renderer: &mut Renderer, fonts: &FontManager) {
        if !self.visible {
            return;
        }

        let (mut c1, mut c2, mut bg, mut fg) = (self.color1, self.color2, self.back_color, self.fore_color);

        if !self.enabled {
            c1 = dim_color(c1, self.disable_alpha);
            c2 = dim_color(c2, self.disable_alpha);
            bg = dim_color(bg, self.disable_alpha);
            fg = dim_color(fg, self.disable_alpha);
        } else if self.pressed {
            c1 = dim_color(c1, self.hover_alpha);
            c2 = dim_color(c2, self.hover_alpha);
        } else if !self.hovered {
            c1 = dim_color(c1, self.hover_alpha);
            c2 = dim_color(c2, self.hover_alpha);
        }

        let r = self.rect;
        renderer.fill_gradient_v(r.x, r.y, r.w, r.h, c1, c2);
        renderer.draw_rect(r.x, r.y, r.w, r.h, bg);

        // Measure text to centre it.
        let font_size = self.font_size;
        let (tw, th) = fonts.measure_text(&self.text, font_size);
        let tx = r.x + ((r.w as f32 - tw) / 2.0) as i32;
        let ty = r.y + ((r.h as f32 - th) / 2.0) as i32;

        // Shadow.
        fonts.draw_text(renderer, tx + 1, ty + 1, &self.text, font_size, bg);
        // Foreground.
        fonts.draw_text(renderer, tx, ty, &self.text, font_size, fg);
    }
}

fn dim_color(c: u32, alpha: f64) -> u32 {
    let r = (((c >> 16) & 0xFF) as f64 * alpha) as u8;
    let g = (((c >> 8) & 0xFF) as f64 * alpha) as u8;
    let b = ((c & 0xFF) as f64 * alpha) as u8;
    Renderer::rgb(r, g, b)
}
