mod matte;

pub use matte::MatteMaterial;

use glam::Vec3;
use rand::rngs::StdRng;

use super::{interaction::Interaction, ray::Ray};

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec>;
}
