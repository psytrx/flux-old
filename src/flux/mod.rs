pub fn render_image(resolution: glam::UVec2) -> image::RgbImage {
    image::RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let uv = glam::vec2(
            x as f32 / resolution.x as f32,
            y as f32 / resolution.y as f32,
        );

        let color = pixel_color(uv);

        color_to_rgb(color)
    })
}

fn pixel_color(uv: glam::Vec2) -> glam::Vec3 {
    let r = uv.x;
    let g = uv.y;
    let b = 0.0;
    glam::vec3(r, g, b)
}

fn color_to_rgb(color: glam::Vec3) -> image::Rgb<u8> {
    let ir = (255.999 * color.x) as u8;
    let ig = (255.999 * color.y) as u8;
    let ib = (255.999 * color.z) as u8;

    image::Rgb([ir, ig, ib])
}
