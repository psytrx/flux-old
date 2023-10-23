use glam::{vec2, vec3, UVec2, Vec2, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};

pub fn render_image(resolution: UVec2) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let uv = vec2(
            x as f32 / resolution.x as f32,
            y as f32 / resolution.y as f32,
        );

        let color = pixel_color(uv);

        color_to_rgb(color)
    })
}

fn pixel_color(uv: Vec2) -> Vec3 {
    let r = uv.x;
    let g = uv.y;
    let b = 0.0;
    vec3(r, g, b)
}

fn color_to_rgb(color: Vec3) -> Rgb<u8> {
    let ir = (255.999 * color.x) as u8;
    let ig = (255.999 * color.y) as u8;
    let ib = (255.999 * color.z) as u8;

    Rgb([ir, ig, ib])
}
