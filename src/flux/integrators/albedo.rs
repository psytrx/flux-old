use glam::Vec3;
use rand::rngs::StdRng;

use crate::flux::{ray::Ray, Scene};

use super::{Integrator, LiResult};

pub struct AlbedoIntegrator;

impl AlbedoIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for AlbedoIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng) -> LiResult {
        match scene.intersect(ray) {
            Some(int) => {
                let le = int.primitive.material.emitted(&int);
                match int.primitive.material.scatter(ray, &int, rng) {
                    Some(srec) => LiResult {
                        li: le + srec.attenuation,
                        rays: 1,
                    },
                    None => LiResult { li: le, rays: 1 },
                }
            }
            None => {
                let background_radiance =
                    scene.lights.iter().map(|light| light.le(ray)).sum::<Vec3>();
                LiResult {
                    li: background_radiance,
                    rays: 1,
                }
            }
        }
    }
}
