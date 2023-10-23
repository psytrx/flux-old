mod accel;
mod camera;
mod interaction;
mod ray;
mod render;
mod scene;
mod sphere;

pub use camera::Camera;
pub use render::render_image;
pub use scene::Scene;
pub use sphere::Sphere;

use std::f32::consts::PI;

use glam::{vec3, Vec2, Vec3};

pub struct CameraSample {
    p_film: Vec2,
}

pub fn uniform_sample_sphere(u: Vec2) -> Vec3 {
    let z = 1.0 - 2.0 * u.x;

    let r = (1.0 - z.powi(2)).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;
    let x = r * phi.cos();
    let y = r * phi.sin();

    vec3(x, y, z)
}
