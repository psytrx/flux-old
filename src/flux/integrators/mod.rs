mod albedo;
mod normal;
mod path;

pub use albedo::*;
pub use normal::*;
pub use path::*;

use glam::Vec3;
use rand::rngs::StdRng;

use super::{ray::Ray, Scene};

pub struct LiResult {
    pub li: Vec3,
    pub rays: usize,
}

pub trait Integrator: Sync {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng) -> LiResult;
}
