use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{ray::Ray, Scene};

use super::{Integrator, LiResult};

pub struct PathTracingIntegrator {
    min_depth: u32,
    max_depth: u32,
    rr_stop_prob: f32,
}

impl PathTracingIntegrator {
    pub fn new(min_depth: u32, max_depth: u32, rr_stop_prob: f32) -> Self {
        Self {
            min_depth,
            max_depth,
            rr_stop_prob,
        }
    }

    fn li_internal(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng, depth: u32) -> LiResult {
        if depth > self.max_depth {
            return LiResult {
                li: Vec3::ZERO,
                rays: 0,
            };
        }

        let rr_factor = if depth > self.min_depth {
            let q = 1.0 - self.rr_stop_prob;
            let s: f32 = rng.gen();
            if s < q {
                return LiResult {
                    li: Vec3::ZERO,
                    rays: 0,
                };
            }
            1.0 / q
        } else {
            1.0
        };

        let rays = 1;
        match scene.intersect(ray) {
            Some(int) => {
                let le = int.primitive.material.emitted(&int);

                match int.primitive.material.scatter(ray, &int, rng) {
                    Some(srec) => {
                        let result = self.li_internal(scene, &srec.scattered, rng, depth + 1);
                        LiResult {
                            li: rr_factor * (le + srec.attenuation * result.li),
                            rays: rays + result.rays,
                        }
                    }
                    None => LiResult {
                        li: rr_factor * le,
                        rays,
                    },
                }
            }
            None => {
                let background_radiance =
                    scene.lights.iter().map(|light| light.le(ray)).sum::<Vec3>();
                LiResult {
                    li: rr_factor * background_radiance,
                    rays,
                }
            }
        }
    }
}

impl Integrator for PathTracingIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng) -> LiResult {
        self.li_internal(scene, ray, rng, 0)
    }
}
