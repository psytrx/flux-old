mod interaction;
mod list;
mod ray;
mod sphere;

use glam::{vec2, vec3, UVec2, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};

use self::{list::ShapeList, ray::Ray, sphere::Sphere};

pub fn render_image(resolution: UVec2) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let look_from = vec3(0.0, 1.0, -4.0);
    let viewport_width = 4.0;
    let viewport_height = viewport_width / resolution.x as f32 * resolution.y as f32;
    let top_left = vec3(-viewport_width / 2.0, viewport_height / 2.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, -viewport_height, 0.0);

    let list = ShapeList::new(vec![
        Sphere::new(vec3(0.0, 1.0, 0.0), 1.0),
        Sphere::new(vec3(1.0, 0.25, -1.0), 0.25),
        Sphere::new(vec3(0.0, -100.0, 0.0), 100.0),
    ]);

    RgbImage::from_fn(resolution.x, resolution.y, |x, y| {
        let uv = vec2(
            x as f32 / resolution.x as f32,
            y as f32 / resolution.y as f32,
        );

        let origin = look_from;
        let target = top_left + horizontal * uv.x + vertical * uv.y;
        let direction = target - origin;
        let ray = Ray::new(origin, direction);

        let color = pixel_color(&list, &ray);

        color_to_rgb(color)
    })
}

fn pixel_color(list: &ShapeList, ray: &Ray) -> Vec3 {
    match list.intersect(ray, f32::EPSILON..f32::INFINITY) {
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
