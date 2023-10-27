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

        match scene.intersect(ray) {
            None => {
                let background = scene.lights.iter().map(|light| light.le(ray)).sum::<Vec3>();
                LiResult {
                    li: rr_factor * background,
                    rays: 1,
                }
            }
            Some(int) => {
                let emitted = int.primitive.material.emitted(&int);

                match int.primitive.material.scatter(ray, &int, rng) {
                    Some(srec) => {
                        let scattering_pdf =
                            int.primitive
                                .material
                                .scattering_pdf(ray, &int, &srec.scattered);
                        let pdf = scattering_pdf;

                        let li = self.li_internal(scene, &srec.scattered, rng, depth + 1);
                        let scattered = (srec.attenuation * scattering_pdf * li.li) / pdf;
                        LiResult {
                            li: rr_factor * (emitted + scattered),
                            rays: 1 + li.rays,
                        }
                    }
                    None => LiResult {
                        li: rr_factor * emitted,
                        rays: 1,
                    },
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
