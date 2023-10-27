use std::{f32::consts::PI, rc::Rc};

use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, textures::Texture, uniform_sample_sphere};

use super::{is_near_zero, BxdfType, Material, ScatterRec};

pub struct MatteMaterial {
    kd: Rc<dyn Texture<Vec3>>,
}

impl MatteMaterial {
    pub fn new(kd: Rc<dyn Texture<Vec3>>) -> Self {
        Self { kd }
    }
}

impl Material for MatteMaterial {
    fn scatter(&self, _ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        let attenuation = self.kd.evaluate(int);

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
    }

    fn bxdf_type(&self) -> BxdfType {
        BxdfType::Diffuse
    }

    fn scattering_pdf(&self, _ray: &Ray, int: &Interaction, scattered: &Ray) -> f32 {
        // INFO: We assume the ray direction is normalized
        let cos_theta = int.n.dot(scattered.direction);
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
