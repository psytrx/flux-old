mod camera;
mod interaction;
mod list;
mod ray;
mod scene;
mod sphere;

use glam::{vec2, vec3, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};

use self::ray::Ray;

pub use camera::*;
pub use list::*;
pub use scene::*;
pub use sphere::*;

pub fn render_image(scene: &Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    RgbImage::from_fn(
        scene.camera.resolution.x,
        scene.camera.resolution.y,
        |x, y| {
            let uv = vec2(
                x as f32 / scene.camera.resolution.x as f32,
                y as f32 / scene.camera.resolution.y as f32,
            );

            let ray = scene.camera.ray(uv);
            let color = pixel_color(scene, &ray);

            color_to_rgb(color)
        },
    )
}

fn pixel_color(scene: &Scene, ray: &Ray) -> Vec3 {
    match scene.aggregate.intersect(ray, f32::EPSILON..f32::INFINITY) {
        None => {
            let unit_direction = ray.direction.normalize();
            let a = (unit_direction.y + 1.0) / 2.0;
            let horizon_color = vec3(0.5, 0.7, 1.0);
            let zenith_color = vec3(1.0, 1.0, 1.0);
            (1.0 - a) * zenith_color + a * horizon_color
        }
        Some(int) => {
            if int.front_face {
                (int.n + 1.0) / 2.0
            } else {
                Vec3::ZERO
            }
        }
    }
}

fn color_to_rgb(color: Vec3) -> Rgb<u8> {
    let ir = (255.999 * color.x) as u8;
    let ig = (255.999 * color.y) as u8;
    let ib = (255.999 * color.z) as u8;

    Rgb([ir, ig, ib])
}
