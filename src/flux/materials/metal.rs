use std::rc::Rc;

use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, textures::Texture, uniform_sample_sphere};

use super::{reflect, Material, ScatterRec};

pub struct MetalMaterial {
    kd: Rc<dyn Texture<Vec3>>,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(kd: Rc<dyn Texture<Vec3>>, fuzz: f32) -> Self {
        Self { kd, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        let attenuation = self.kd.evaluate(int);

        let reflected = reflect(ray.direction.normalize(), int.n);
        let direction = reflected + self.fuzz * uniform_sample_sphere(rng.gen());

        // Rays scattering to below the originating surface will be cancelled
        let above_surface = direction.dot(int.n) > 0.0;
        if above_surface {
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
