use std::{f32::consts::PI, rc::Rc};

use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{
    interaction::{spawn_ray, Interaction},
    onb::Onb,
    ray::Ray,
    textures::Texture,
    uniform_sample_hemisphere,
};

use super::{BxdfType, Material, ScatterRec};

pub struct MatteMaterial {
    kd: Rc<dyn Texture<Vec3>>,
}

impl MatteMaterial {
    pub fn new(kd: Rc<dyn Texture<Vec3>>) -> Self {
        Self { kd }
    }
}

impl Material for MatteMaterial {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        let attenuation = self.kd.evaluate(int);

        let uvw = Onb::from_w(int.n);
        let direction = uvw.local(uniform_sample_hemisphere(rng.gen()));
        let scattered = spawn_ray(int.p, direction, ray.time);

        let pdf = uvw.w.dot(direction) / PI;

        Some(ScatterRec {
            attenuation,
            scattered,
            pdf,
        })
    }

    fn bxdf_type(&self) -> BxdfType {
        BxdfType::Diffuse
    }

    fn scattering_pdf(&self, _ray: &Ray, _int: &Interaction, _scattered: &Ray) -> f32 {
        1.0 / (2.0 * PI)
    }
}
