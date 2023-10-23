mod camera;
mod interaction;
mod list;
mod ray;
mod scene;
mod sphere;

use glam::{vec2, vec3, Vec2, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{rngs::StdRng, Rng, SeedableRng};

use self::ray::Ray;

pub use camera::*;
pub use list::*;
pub use scene::*;
pub use sphere::*;

pub struct CameraSample {
    p_film: Vec2,
}

pub fn render_image(scene: &Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut rng = StdRng::seed_from_u64(0);
    let samples_per_pixel = 64;

    RgbImage::from_fn(
        scene.camera.resolution.x,
        scene.camera.resolution.y,
        |x, y| {
            let p_raster = vec2(x as f32, y as f32);

            let mut color_sum = Vec3::ZERO;
            let mut weight_sum = 0.0;

            for _ in 0..samples_per_pixel {
                let p_film = p_raster + rng.gen::<Vec2>();

                let camera_sample = CameraSample { p_film };
                let ray = scene.camera.ray(&camera_sample);

                if let Some(color) = pixel_color(scene, &ray) {
                    color_sum += color;
                    weight_sum += 1.0;
                }
            }

            let color = color_sum / weight_sum;
            color_to_rgb(color)
        },
    )
}

fn pixel_color(scene: &Scene, ray: &Ray) -> Option<Vec3> {
    match scene.aggregate.intersect(ray, f32::EPSILON..f32::INFINITY) {
        None => {
            let unit_direction = ray.direction.normalize();
            let a = (unit_direction.y + 1.0) / 2.0;
            let horizon_color = vec3(0.5, 0.7, 1.0);
            let zenith_color = vec3(1.0, 1.0, 1.0);
            Some((1.0 - a) * zenith_color + a * horizon_color)
        }
        Some(int) => {
            if int.front_face {
                Some((int.n + 1.0) / 2.0)
            } else {
                None
            }
        }
    }
}

fn color_to_rgb(color: Vec3) -> Rgb<u8> {
    let ir = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
    let ig = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
    let ib = (255.0 * color.z.clamp(0.0, 1.0)) as u8;

    Rgb([ir, ig, ib])
}
