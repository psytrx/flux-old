use glam::Vec3;
use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray, uniform_sample_sphere};

use super::{Material, ScatterRec};

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
        int.front_face.then(|| {
            let attenuation = self.albedo;

            let mut scattered_dir = int.n + uniform_sample_sphere(rng.gen());
            if is_near_zero(scattered_dir) {
                scattered_dir = int.n;
            }
            let scattered = int.spawn_ray(scattered_dir);

            ScatterRec {
                attenuation,
                scattered,
            }
        })
    }
}

fn is_near_zero(v: Vec3) -> bool {
    let s = f32::EPSILON;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
