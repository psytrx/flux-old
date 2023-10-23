mod camera;
mod interaction;
mod list;
mod ray;
mod scene;
mod sphere;

use glam::{vec2, vec3, UVec2, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};

use self::{camera::Camera, list::ShapeList, ray::Ray, scene::Scene, sphere::Sphere};

pub fn render_image(resolution: UVec2) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let camera = Camera::new(resolution);

    let aggregate = ShapeList::new(vec![
        Sphere::new(vec3(0.0, 1.0, 0.0), 1.0),
        Sphere::new(vec3(1.0, 0.25, -1.0), 0.25),
        Sphere::new(vec3(0.0, -100.0, 0.0), 100.0),
    ]);

    let scene = Scene { camera, aggregate };

    RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let uv = vec2(
            x as f32 / resolution.x as f32,
            y as f32 / resolution.y as f32,
        );

        let ray = scene.camera.ray(uv);
        let color = pixel_color(&scene, &ray);

        color_to_rgb(color)
    })
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
