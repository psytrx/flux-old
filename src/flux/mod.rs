mod camera;
mod interaction;
mod list;
mod ray;
mod render;
mod scene;
mod sphere;

pub use camera::Camera;
use glam::{Vec2, Vec3};
pub use list::ShapeList;
use rand::{rngs::StdRng, Rng};
pub use render::render_image;
pub use scene::Scene;
pub use sphere::Sphere;

pub struct CameraSample {
    p_film: Vec2,
}

pub fn random_unit_vector(rng: &mut StdRng) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

fn random_in_unit_sphere(rng: &mut StdRng) -> Vec3 {
    loop {
        let p = rng.gen::<Vec3>() * 2.0 - 1.0;
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
