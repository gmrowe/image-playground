//! An example of opening an image.
use std::path::Path;

const BG_SOURCE: &str = "./assets/background.png";
const FG_SOURCE: &str = "./assets/teapot.png";
const DEST: &str = "./assets/output.png";

fn alpha_blend(bg: &image::Rgba<u8>, fg: &image::Rgba<u8>) -> image::Rgba<u8> {
    let image::Rgba([r0, g0, b0, a0]) = *bg;
    let image::Rgba([r1, g1, b1, a1]) = *fg;

    let a0_f = a0 as f64 / 255.0;
    let a1_f = a1 as f64 / 255.0;
    let a_comp_f = a1_f + a0_f * (1.0 - a1_f);

    let cr = ((r1 as f64 * a1_f) + (r0 as f64 * a0_f) * (1.0 - a1_f)) / a_comp_f;
    let cg = ((g1 as f64 * a1_f) + (g0 as f64 * a0_f) * (1.0 - a1_f)) / a_comp_f;
    let cb = ((b1 as f64 * a1_f) + (b0 as f64 * a0_f) * (1.0 - a1_f)) / a_comp_f;
      
    image::Rgba([cr.floor() as u8,
                 cg.floor() as u8,
                 cb.floor() as u8,
                 (a_comp_f * 255.0).floor() as u8
    ])
}

fn main() -> image::error::ImageResult<()> {
    let bg_image = image::open(&Path::new(BG_SOURCE))?;
    let fg_image = image::open(&Path::new(FG_SOURCE))?;

    let mut bg = bg_image.to_rgba8();
    let fg = fg_image.to_rgba8();

    for (bg_pix, fg_pix) in bg.pixels_mut().zip(fg.pixels()){
        *bg_pix = alpha_blend(bg_pix, fg_pix);
    }

    bg.save_with_format(DEST, image::ImageFormat::Png)?;
    
    Ok(())

}
