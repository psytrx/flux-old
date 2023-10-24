use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, uniform_sample_sphere};

use super::{reflect, Material, ScatterRec};

pub struct MetalMaterial {
    albedo: Vec3,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        int.front_face
            .then(|| {
                let attenuation = self.albedo;

                let reflected = reflect(ray.direction.normalize(), int.n);
                let direction = reflected + self.fuzz * uniform_sample_sphere(rng.gen());

                // gazing rays, scattering to below the originating surface
                let gazing = direction.dot(int.n) > 0.0;

                gazing.then(|| ScatterRec {
                    attenuation,
                    scattered: int.spawn_ray(direction),
                })
            })
            .unwrap_or(None)
    }
}
