pub fn render_image(resolution: glam::UVec2) -> image::RgbImage {
    image::RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let r = x as f32 / resolution.x as f32;
        let g = y as f32 / resolution.y as f32;
        let b = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        image::Rgb([ir, ig, ib])
    })
}
