use glam::uvec2;
use image::{Rgb, RgbImage};

fn main() -> anyhow::Result<()> {
    let resolution = uvec2(800, 600);

    let img = RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let r = x as f32 / resolution.x as f32;
        let g = y as f32 / resolution.y as f32;
        let b = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        Rgb([ir, ig, ib])
    });

    img.save("./output/output.png")?;

    Ok(())
}
