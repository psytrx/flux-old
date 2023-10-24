use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, uniform_sample_sphere};

use super::{is_near_zero, Material, ScatterRec};

pub struct MatteMaterial {
    albedo: Vec3,
}

impl MatteMaterial {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for MatteMaterial {
    fn scatter(&self, _ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        if int.front_face {
            let attenuation = self.albedo;

            let direction = int.n + uniform_sample_sphere(rng.gen());
            let direction = if is_near_zero(direction) {
                int.n
            } else {
                direction
            };
            let scattered = int.spawn_ray(direction);

            Some(ScatterRec {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
