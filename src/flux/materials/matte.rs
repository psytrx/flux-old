use std::rc::Rc;

use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, textures::Texture, uniform_sample_sphere};

use super::{is_near_zero, Material, ScatterRec};

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
        if int.front_face {
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
        } else {
            None
        }
    }
}
