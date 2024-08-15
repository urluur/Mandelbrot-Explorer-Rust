// const MAX_ITER: u32 = 1000;

pub fn color_map(iter: u32, max_iter: u32) -> u32 {
    let hue = (iter as f64 / max_iter as f64) * 360.0;
    let saturation = 1.0;
    let lightness = if iter == max_iter { 0.0 } else { 0.5 };
    let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = lightness - c / 2.0;
    let (r, g, b) = match hue {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        (((r + m) * 255.0) as u8),
        (((g + m) * 255.0) as u8),
        (((b + m) * 255.0) as u8),
    )
}
