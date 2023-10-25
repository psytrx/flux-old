use glam::Vec3;
use rand::rngs::StdRng;

use crate::flux::{ray::Ray, Scene};

use super::{Integrator, LiResult};

pub struct NormalIntegrator;

impl NormalIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for NormalIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, _rng: &mut StdRng) -> LiResult {
        match scene.intersect(ray) {
            Some(int) => LiResult {
                li: (int.n + 1.0) / 2.0,
                rays: 1,
            },
            None => LiResult {
                li: Vec3::ZERO,
                rays: 1,
            },
        }
    }
}
