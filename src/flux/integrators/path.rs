use glam::{vec3, Vec3};

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
                let emitted = int.primitive.material.emitted(ray, &int);

                match int.primitive.material.scatter(ray, &int, rng) {
                    None => LiResult {
                        li: rr_factor * emitted,
                        rays: 1,
                    },
                    Some(srec) => {
                        let on_light = vec3(
                            rng.gen_range(-10.0..10.0),
                            50.0 - 32.0 * f32::EPSILON,
                            rng.gen_range(-10.0..10.0),
                        );
                        let to_light = on_light - int.p;
                        let distance_squared = to_light.length();
                        let to_light = to_light.normalize();

                        if to_light.dot(int.n) < 0.0 {
                            return LiResult {
                                li: rr_factor * emitted,
                                rays: 1,
                            };
                        }

                        let light_area = 400.0;
                        let light_cosine = to_light.y.abs();
                        if light_cosine < 32.0 * f32::EPSILON {
                            return LiResult {
                                li: rr_factor * emitted,
                                rays: 1,
                            };
                        }

                        let pdf = distance_squared / (light_cosine * light_area);
                        let scattered = Ray::new(int.p, to_light, ray.time);
                        let scattering_pdf =
                            int.primitive.material.scattering_pdf(ray, &int, &scattered);

                        let li = self.li_internal(scene, &srec.scattered, rng, depth + 1);
                        let scattered = (srec.attenuation * scattering_pdf * li.li) / pdf;

                        LiResult {
                            li: rr_factor * (emitted + scattered),
                            rays: 1 + li.rays,
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
