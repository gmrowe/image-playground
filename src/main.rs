use std::path::Path;
use image::{Rgba, RgbaImage, ImageFormat};


struct Position {
    dx: i64,
    dy: i64,
}

impl Position {
    fn new(dx: i64, dy: i64) -> Self {
        Self {dx, dy}
    }
}

fn in_bounds(x: i64, y: i64, width: u32, height: u32) -> bool {
    x >= 0
        && y >= 0
        && (x as u32) < width
        && (y as u32) < height
}

fn composite(
    bg_image: &mut RgbaImage,
    fg_image: &RgbaImage,
    fg_opacity: f64,
    fg_pos: Position
) {
    for (x0, y0, fg_pixel) in fg_image.enumerate_pixels() {
        let x = x0 as i64 + fg_pos.dx;
        let y = y0 as i64 + fg_pos.dy;
        if in_bounds(x, y, bg_image.width(), bg_image.height()) {
            let bg_pixel = bg_image.get_pixel_mut(x as u32, y as u32);
            *bg_pixel = alpha_blend(bg_pixel, fg_pixel, fg_opacity);
        }
    }
}

fn alpha_blend(bg_pixel: &Rgba<u8>, fg_pixel: &Rgba<u8>, fg_opacity: f64) -> Rgba<u8> {
    let Rgba([r0, g0, b0, a0]) = *bg_pixel;
    let Rgba([r1, g1, b1, a1]) = *fg_pixel;

    let bg_alpha = a0 as f64 / 255.0;
    let fg_alpha = a1 as f64 / 255.0 * fg_opacity;
    let comp_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);
    
    let blend = |bg_comp, fg_comp| {
        let raw =
            (fg_comp as f64 * fg_alpha) + (bg_comp as f64 * bg_alpha) * (1.0 - fg_alpha);
        (raw / comp_alpha).floor() as u8
    };

    Rgba([blend(r0, r1),
          blend(g0, g1),
          blend(b0, b1),
          (comp_alpha * 255.0).floor() as u8
    ])
}

fn main() -> image::error::ImageResult<()> {
    const BG_SOURCE: &str = "./assets/background.png";
    const FG_SOURCE: &str = "./assets/teapot.png";
    const DEST: &str = "./assets/output.png";

    let bg_image = image::open(&Path::new(BG_SOURCE))?;
    let fg_image = image::open(&Path::new(FG_SOURCE))?;
    const FG_ALPHA: f64 = 0.4;

    let mut bg = bg_image.to_rgba8();
    let fg = fg_image.to_rgba8();

    composite(&mut bg, &fg, FG_ALPHA, Position::new(0, 0));
    bg.save_with_format(DEST, ImageFormat::Png)?;
    
    Ok(())

}
