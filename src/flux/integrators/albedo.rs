use glam::Vec3;
use rand::rngs::StdRng;

use crate::flux::{ray::Ray, BxdfType, Scene};

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
                let le = int.primitive.material.emitted(ray, &int);

                match int.primitive.material.scatter(ray, &int, rng) {
                    Some(srec) => {
                        // only return the first specular or diffuse hit
                        let t = int.primitive.material.bxdf_type();
                        if t == BxdfType::Diffuse || t == BxdfType::Specular {
                            LiResult {
                                li: le + srec.attenuation,
                                rays: 1,
                            }
                        } else {
                            let result = self.li(scene, &srec.scattered, rng);
                            LiResult {
                                li: le + srec.attenuation * result.li,
                                rays: 1 + result.rays,
                            }
                        }
                    }
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
