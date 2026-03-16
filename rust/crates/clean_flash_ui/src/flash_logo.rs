use crate::renderer::Renderer;

/// Cached flash logo bitmap. Stores a pre-rendered pixel buffer so the
/// expensive MSAA polygon fill only runs when the target size changes.
pub struct FlashLogoCache {
    width: i32,
    height: i32,
    /// Pre-rendered pixels in 0x00RRGGBB format, row-major, size = width * height.
    pixels: Vec<u32>,
    /// Shadow offset derived from width.
    shadow_off: i32,
}

impl FlashLogoCache {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            pixels: Vec::new(),
            shadow_off: 0,
        }
    }

    /// Draw the flash logo at (x, y) with size (w, h).
    /// Re-renders into the cache only if the size changed.
    pub fn draw(&mut self, renderer: &mut Renderer, x: i32, y: i32, w: i32, h: i32) {
        if w <= 0 || h <= 0 {
            return;
        }

        if self.width != w || self.height != h {
            self.rebuild(w, h);
        }

        // Draw subtle drop shadow first.
        let shadow_color = Renderer::rgb(0, 0, 0);
        let shadow_alpha: u8 = 40;
        let so = self.shadow_off;
        for dy in 0..h {
            for dx in 0..w {
                renderer.blend_pixel(x + dx + so, y + dy + so, shadow_color, shadow_alpha);
            }
        }

        // Blit cached pixels onto the renderer.
        for dy in 0..h {
            for dx in 0..w {
                let px = self.pixels[dy as usize * w as usize + dx as usize];
                renderer.set_pixel(x + dx, y + dy, px);
            }
        }
    }

    /// Render the logo into the internal cache at the given size.
    fn rebuild(&mut self, w: i32, h: i32) {
        self.width = w;
        self.height = h;
        self.shadow_off = ((w as f32 * 0.02).round() as i32).max(1);

        // Render into a temporary offscreen renderer.
        let mut tmp = Renderer::new(w as usize, h as usize);

        let vw = 500.0_f32;
        let vh = 487.0_f32;
        let sx = w as f32 / vw;
        let sy = h as f32 / vh;

        let blue = Renderer::rgb(0x00, 0x3A, 0x74);
        tmp.fill_rect(0, 0, w, h, blue);

        let white = Renderer::rgb(255, 255, 255);
        let points = build_f_polygon();
        fill_polygon(&mut tmp, &points, 0.0, 0.0, sx, sy, white);

        self.pixels = tmp.buffer;
    }
}

/// Evaluate a cubic bezier curve and return `segments` points (excluding the start point).
fn cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    segments: usize,
) -> Vec<(f32, f32)> {
    let mut pts = Vec::with_capacity(segments);
    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let u = 1.0 - t;
        let x = u * u * u * p0.0
            + 3.0 * u * u * t * p1.0
            + 3.0 * u * t * t * p2.0
            + t * t * t * p3.0;
        let y = u * u * u * p0.1
            + 3.0 * u * u * t * p1.1
            + 3.0 * u * t * t * p2.1
            + t * t * t * p3.1;
        pts.push((x, y));
    }
    pts
}

/// Build the "f" shape polygon by tracing the SVG path data, flattening beziers
/// into line segments. Coordinates are in the SVG viewBox space (500×487).
fn build_f_polygon() -> Vec<(f32, f32)> {
    let mut pts = Vec::with_capacity(128);
    const N: usize = 16; // segments per bezier curve

    // M 269.2,138.7
    pts.push((269.2, 138.7));

    // c -22.5,27.5 -35.8,61.8 -48.7,95
    pts.extend(cubic_bezier(
        (269.2, 138.7),
        (246.7, 166.2),
        (233.4, 200.5),
        (220.5, 233.7),
        N,
    ));

    // c -26.1,67.3 -43.6,105.8 -99.7,105.8
    pts.extend(cubic_bezier(
        (220.5, 233.7),
        (194.4, 301.0),
        (176.9, 339.5),
        (120.8, 339.5),
        N,
    ));

    // V 402
    pts.push((120.8, 402.0));

    // c 46.4,0 84.1,-17.2 112,-51.2
    pts.extend(cubic_bezier(
        (120.8, 402.0),
        (167.2, 402.0),
        (204.9, 384.8),
        (232.8, 350.8),
        N,
    ));

    // c 17.9,-21.9 30.3,-49.3 40.9,-75.8
    pts.extend(cubic_bezier(
        (232.8, 350.8),
        (250.7, 328.9),
        (263.1, 301.5),
        (273.7, 275.0),
        N,
    ));

    // h 74.1
    pts.push((347.8, 275.0));

    // v -62.5
    pts.push((347.8, 212.5));

    // h -48.8
    pts.push((299.0, 212.5));

    // c 18.9,-40.6 39.2,-62.5 82.1,-62.5
    pts.extend(cubic_bezier(
        (299.0, 212.5),
        (317.9, 171.9),
        (338.2, 150.0),
        (381.1, 150.0),
        N,
    ));

    // V 87.5
    pts.push((381.1, 87.5));

    // C 334.8,87.5 297.1,104.7 269.2,138.7  (absolute cubic, closes the shape)
    pts.extend(cubic_bezier(
        (381.1, 87.5),
        (334.8, 87.5),
        (297.1, 104.7),
        (269.2, 138.7),
        N,
    ));

    pts
}

/// Test whether a point is inside the polygon using the even-odd (parity) rule.
fn point_in_polygon(transformed: &[(f32, f32)], px: f32, py: f32) -> bool {
    let n = transformed.len();
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = transformed[i];
        let (xj, yj) = transformed[j];
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Scanline-fill a polygon with 4×4 MSAA antialiasing after transforming from
/// viewBox to screen coordinates: screen_x = ox + vx * sx, screen_y = oy + vy * sy.
fn fill_polygon(
    renderer: &mut Renderer,
    points: &[(f32, f32)],
    ox: f32,
    oy: f32,
    sx: f32,
    sy: f32,
    color: u32,
) {
    if points.len() < 3 {
        return;
    }

    // Transform to screen space.
    let transformed: Vec<(f32, f32)> = points
        .iter()
        .map(|&(px, py)| (ox + px * sx, oy + py * sy))
        .collect();

    // Bounding box.
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for &(x, y) in &transformed {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    let x_start = (min_x.floor() as i32).max(0);
    let x_end = (max_x.ceil() as i32).min(renderer.width as i32 - 1);
    let y_start = (min_y.floor() as i32).max(0);
    let y_end = (max_y.ceil() as i32).min(renderer.height as i32 - 1);

    // 4×4 sub-pixel grid offsets (16 samples per pixel).
    const GRID: usize = 4;
    const SAMPLES: usize = GRID * GRID;
    let step = 1.0 / GRID as f32;
    let half_step = step / 2.0;

    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let mut hits = 0u32;
            for sy_i in 0..GRID {
                let sample_y = y as f32 + half_step + sy_i as f32 * step;
                for sx_i in 0..GRID {
                    let sample_x = x as f32 + half_step + sx_i as f32 * step;
                    if point_in_polygon(&transformed, sample_x, sample_y) {
                        hits += 1;
                    }
                }
            }
            if hits == SAMPLES as u32 {
                renderer.set_pixel(x, y, color);
            } else if hits > 0 {
                let alpha = ((hits * 255 + SAMPLES as u32 / 2) / SAMPLES as u32) as u8;
                renderer.blend_pixel(x, y, color, alpha);
            }
        }
    }
}
