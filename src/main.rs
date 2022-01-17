use std::path::Path;

const BG_SOURCE: &str = "./assets/background.png";
const FG_SOURCE: &str = "./assets/teapot.png";
const DEST: &str = "./assets/output.png";

fn alpha_blend(bg: &image::Rgba<u8>, fg: &image::Rgba<u8>) -> image::Rgba<u8> {
    let image::Rgba([r0, g0, b0, a0]) = *bg;
    let image::Rgba([r1, g1, b1, a1]) = *fg;

    let bg_alpha = a0 as f64 / 255.0;
    let fg_alpha = a1 as f64 / 255.0 / 2.0;
    let comp_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);
    
    let a_blend = |bg_comp, fg_comp| {
        let raw =
            (fg_comp as f64 * fg_alpha) + (bg_comp as f64 * bg_alpha) * (1.0 - fg_alpha);
        (raw / comp_alpha).floor() as u8
    };

    image::Rgba([a_blend(r0, r1),
                 a_blend(g0, g1),
                 a_blend(b0, b1),
                 (comp_alpha * 255.0).floor() as u8
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
