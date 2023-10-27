mod accel;
mod bounds;
pub mod cameras;
mod denoise;
mod film;
pub mod integrators;
mod interaction;
pub mod lights;
mod materials;
mod onb;
mod pdf;
mod primitive;
mod ray;
mod renderer;
mod sampler;
mod scene;
pub mod shapes;
pub mod textures;
mod updater;

pub use bounds::*;
pub use cameras::*;
pub use denoise::*;
pub use film::Film;
pub use materials::*;
pub use primitive::*;
use rand::{rngs::StdRng, Rng};
pub use renderer::*;
pub use sampler::*;
pub use scene::*;
pub use updater::*;

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

#[allow(dead_code)]
pub fn uniform_sample_hemisphere(u: Vec2) -> Vec3 {
    let z = u.x;
    let r = (1.0 - z.powi(2)).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;
    let x = r * phi.cos();
    let y = r * phi.sin();
    vec3(x, y, z)
}

#[allow(dead_code)]
pub fn random_in_hemisphere(n: Vec3, rng: &mut StdRng) -> Vec3 {
    let on_unit_sphere = random_unit_vector(rng);
    if on_unit_sphere.dot(n) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn random_unit_vector(rng: &mut StdRng) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

#[allow(dead_code)]
fn random_in_unit_sphere(rng: &mut StdRng) -> Vec3 {
    loop {
        let p = vec3(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
fn random_cosine_direction(rng: &mut StdRng) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    vec3(x, y, z)
}
