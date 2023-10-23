mod matte;
mod metal;

pub use matte::MatteMaterial;
pub use metal::MetalMaterial;

use glam::Vec3;
use rand::rngs::StdRng;

use super::{interaction::Interaction, ray::Ray};

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub scattered: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec>;
}

pub fn is_near_zero(v: Vec3) -> bool {
    let s = 8.0 * f32::EPSILON;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
