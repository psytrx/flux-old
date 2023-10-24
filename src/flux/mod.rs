mod accel;
mod bounds;
mod camera;
mod denoise;
mod film;
mod interaction;
mod materials;
mod primitive;
mod ray;
mod render;
mod scene;
pub mod shapes;

pub use bounds::Bounds2;
pub use camera::Camera;
pub use denoise::Denoiser;
pub use materials::*;
pub use primitive::Primitive;
pub use render::Renderer;
pub use scene::Scene;

use std::f32::consts::PI;

use glam::{vec2, vec3, Vec2, Vec3};

pub struct CameraSample {
    p_film: Vec2,
    p_lens: Vec2,
    time: f32,
}

pub fn uniform_sample_sphere(u: Vec2) -> Vec3 {
    let z = 1.0 - 2.0 * u.x;

    let r = (1.0 - z.powi(2)).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;
    let x = r * phi.cos();
    let y = r * phi.sin();

    vec3(x, y, z)
}

pub fn uniform_sample_disk(u: Vec2) -> Vec2 {
    let u_offset = 2.0 * u - Vec2::ONE;

    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        Vec2::ZERO
    } else {
        let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
            (u_offset.x, PI / 4.0 * (u_offset.y / u_offset.x))
        } else {
            (u_offset.y, PI / 2.0 - PI / 4.0 * (u_offset.x / u_offset.y))
        };

        vec2(r * theta.cos(), r * theta.sin())
    }
}
