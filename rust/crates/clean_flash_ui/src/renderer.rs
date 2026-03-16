/// Software renderer operating on a `Vec<u32>` pixel buffer (0xAA_RR_GG_BB).
/// All drawing is done in-memory; the buffer is presented via minifb.
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    /// Pack r, g, b into the minifb pixel format (0x00RRGGBB).
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Clear the entire buffer to a single colour.
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    /// Set a single pixel (bounds-checked).
    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = color;
        }
    }

    /// Alpha-blend a single pixel. `alpha` is 0..=255.
    #[inline]
    pub fn blend_pixel(&mut self, x: i32, y: i32, color: u32, alpha: u8) {
        if x < 0 || y < 0 || (x as usize) >= self.width || (y as usize) >= self.height {
            return;
        }
        let idx = y as usize * self.width + x as usize;
        let dst = self.buffer[idx];
        self.buffer[idx] = alpha_blend(dst, color, alpha);
    }

    /// Fill a solid rectangle.
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let x0 = x.max(0) as usize;
        let y0 = y.max(0) as usize;
        let x1 = ((x + w) as usize).min(self.width);
        let y1 = ((y + h) as usize).min(self.height);

        for row in y0..y1 {
            let start = row * self.width + x0;
            let end = row * self.width + x1;
            self.buffer[start..end].fill(color);
        }
    }

    /// Draw a 1px rectangle outline.
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        // Top / bottom
        for dx in 0..w {
            self.set_pixel(x + dx, y, color);
            self.set_pixel(x + dx, y + h - 1, color);
        }
        // Left / right
        for dy in 0..h {
            self.set_pixel(x, y + dy, color);
            self.set_pixel(x + w - 1, y + dy, color);
        }
    }

    /// Fill a rectangle with a vertical linear gradient from `color1` (top) to `color2` (bottom).
    pub fn fill_gradient_v(&mut self, x: i32, y: i32, w: i32, h: i32, color1: u32, color2: u32) {
        if h <= 0 {
            return;
        }
        let (r1, g1, b1) = unpack(color1);
        let (r2, g2, b2) = unpack(color2);

        for dy in 0..h {
            let t = dy as f32 / (h - 1).max(1) as f32;
            let r = lerp_u8(r1, r2, t);
            let g = lerp_u8(g1, g2, t);
            let b = lerp_u8(b1, b2, t);
            let c = Self::rgb(r, g, b);
            self.fill_rect(x, y + dy, w, 1, c);
        }
    }

    /// Fill a rectangle with a horizontal linear gradient.
    pub fn fill_gradient_h(&mut self, x: i32, y: i32, w: i32, h: i32, color1: u32, color2: u32) {
        if w <= 0 {
            return;
        }
        let (r1, g1, b1) = unpack(color1);
        let (r2, g2, b2) = unpack(color2);

        for dx in 0..w {
            let t = dx as f32 / (w - 1).max(1) as f32;
            let r = lerp_u8(r1, r2, t);
            let g = lerp_u8(g1, g2, t);
            let b = lerp_u8(b1, b2, t);
            let c = Self::rgb(r, g, b);
            self.fill_rect(x + dx, y, 1, h, c);
        }
    }

    /// Draw an RGBA image scaled to (w, h) at (x, y) using nearest-neighbor interpolation.
    pub fn draw_image_scaled(&mut self, x: i32, y: i32, w: i32, h: i32, img: &RgbaImage) {
        if w <= 0 || h <= 0 || img.width == 0 || img.height == 0 {
            return;
        }
        for dy in 0..h {
            for dx in 0..w {
                let src_x = (dx as f32 * img.width as f32 / w as f32) as usize;
                let src_y = (dy as f32 * img.height as f32 / h as f32) as usize;
                let idx = (src_y * img.width + src_x) * 4;
                let r = img.data[idx];
                let g = img.data[idx + 1];
                let b = img.data[idx + 2];
                let a = img.data[idx + 3];
                if a == 255 {
                    self.set_pixel(x + dx, y + dy, Self::rgb(r, g, b));
                } else if a > 0 {
                    self.blend_pixel(x + dx, y + dy, Self::rgb(r, g, b), a);
                }
            }
        }
    }

    /// Draw an RGBA image onto the framebuffer at (x, y).
    pub fn draw_image(&mut self, x: i32, y: i32, img: &RgbaImage) {
        for iy in 0..img.height as i32 {
            for ix in 0..img.width as i32 {
                let idx = (iy as usize * img.width + ix as usize) * 4;
                let r = img.data[idx];
                let g = img.data[idx + 1];
                let b = img.data[idx + 2];
                let a = img.data[idx + 3];
                if a == 255 {
                    self.set_pixel(x + ix, y + iy, Self::rgb(r, g, b));
                } else if a > 0 {
                    self.blend_pixel(x + ix, y + iy, Self::rgb(r, g, b), a);
                }
            }
        }
    }
}

/// Simple RGBA image stored as raw bytes.
pub struct RgbaImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // RGBA, row-major
}

impl RgbaImage {
    /// Load a PNG from embedded bytes.
    pub fn from_png_bytes(bytes: &[u8]) -> Self {
        let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
            .expect("Failed to decode PNG")
            .to_rgba8();
        Self {
            width: img.width() as usize,
            height: img.height() as usize,
            data: img.into_raw(),
        }
    }

    /// Create an empty (transparent) image.
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}

// ---- helpers ----

#[inline]
fn unpack(c: u32) -> (u8, u8, u8) {
    (((c >> 16) & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, (c & 0xFF) as u8)
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

#[inline]
fn alpha_blend(dst: u32, src: u32, alpha: u8) -> u32 {
    let (sr, sg, sb) = unpack(src);
    let (dr, dg, db) = unpack(dst);
    let a = alpha as u16;
    let inv = 255 - a;
    let r = ((sr as u16 * a + dr as u16 * inv) / 255) as u8;
    let g = ((sg as u16 * a + dg as u16 * inv) / 255) as u8;
    let b = ((sb as u16 * a + db as u16 * inv) / 255) as u8;
    Renderer::rgb(r, g, b)
}
