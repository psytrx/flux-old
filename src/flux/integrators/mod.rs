mod path;

use glam::Vec3;
pub use path::PathTracingIntegrator;
use rand::rngs::StdRng;

use super::{ray::Ray, Scene};

pub struct LiResult {
    pub li: Vec3,
    pub rays: usize,
}

pub trait Integrator: Sync {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng) -> LiResult;
}
