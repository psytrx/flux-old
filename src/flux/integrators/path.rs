use glam::Vec3;

use log::trace;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::spawn_ray, ray::Ray, BxdfType, Scene};

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
                let emitted = int.primitive.material.emitted(ray, &int);

                match int.primitive.material.scatter(ray, &int, rng) {
                    None => LiResult {
                        li: rr_factor * emitted,
                        rays: 1,
                    },
                    Some(srec) => {
                        // let lights = scene
                        //     .primitives
                        //     .iter()
                        //     .filter(|prim| prim.material.bxdf_type() == BxdfType::DiffuseLight)
                        //     .collect::<Vec<_>>();
                        // let _light = lights[rng.gen_range(0..lights.len())];
                        //
                        // let sampled_light = lights[rng.gen_range(0..lights.len())];
                        // let sampled_point = sampled_light.shape.sample_point(int.p, rng);
                        // // trace!("Sampled point: {}", sampled_point);
                        //
                        // let direction = sampled_point - int.p;
                        // let distance = direction.length();
                        // let distance_squared = direction.length_squared();
                        // let direction = direction.normalize();
                        // let shadow_ray = spawn_ray(int.p, 8.0 * direction, ray.time);
                        //
                        // // TODO: magic number. Find a better way to handle t_max
                        // let t_max = 0.99 * distance;
                        // if scene.occluded(&shadow_ray, t_max) {
                        //     return LiResult {
                        //         li: rr_factor * emitted,
                        //         rays: 1,
                        //     };
                        // }
                        //
                        // let cosine = (direction.dot(int.n) / direction.length()).abs();
                        // let pdf_val = distance_squared / (cosine * sampled_light.shape.area());
                        //
                        // let scattering_pdf = 2.0
                        //     * int
                        //         .primitive
                        //         .material
                        //         .scattering_pdf(ray, &int, &shadow_ray);
                        //
                        // let li = self.li_internal(scene, &shadow_ray, rng, depth + 1);
                        // let scattered = (srec.attenuation * scattering_pdf * li.li) / pdf_val;

                        let scattered = self.li_internal(scene, &srec.scattered, rng, depth + 1);
                        LiResult {
                            li: rr_factor * (emitted + srec.attenuation * scattered.li),
                            rays: 1 + scattered.rays,
                        }
                    }
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
