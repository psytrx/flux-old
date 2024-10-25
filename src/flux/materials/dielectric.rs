use std::rc::Rc;

use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, textures::Texture};

use super::{reflect, refract, BxdfType, Material, ScatterRec};

pub struct DielectricMaterial {
    kd: Rc<dyn Texture<Vec3>>,
    ior: f32,
}

impl DielectricMaterial {
    pub fn new(kd: Rc<dyn Texture<Vec3>>, ior: f32) -> Self {
        Self { kd, ior }
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        let attenuation = self.kd.evaluate(int);

        // If we hit the back face, we flip the normal
        let front_face = ray.direction.dot(int.n) < 0.0;

        // TODO: avoid this allocation, maybe pass an extra paramter into
        // Interaction::spawn_ray to flip the direction?
        let mut int = Interaction {
            n: if front_face { int.n } else { -int.n },
            ..*int
        };

        let refraction_ratio = if front_face { 1.0 / self.ior } else { self.ior };

        // INFO: We expect ray directions to be normalized. See Scene::intersect, where we
        // normalize the normal vector by default.
        let unit_direction = ray.direction;

        let cos_theta = (-unit_direction).dot(int.n).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflecting = cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen();

        let scattered = if reflecting {
            let direction = reflect(unit_direction, int.n);
            int.spawn_ray(direction)
        } else {
            let direction = refract(unit_direction, int.n, refraction_ratio);
            // If we refract the ray, we need to flip the normal direction so we offset the spawned
            // ray in the correct direction.
            int.n = -int.n;
            int.spawn_ray(direction)
        };

        Some(ScatterRec {
            attenuation,
            scattered,
        })
    }

    fn bxdf_type(&self) -> BxdfType {
        BxdfType::Other
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
