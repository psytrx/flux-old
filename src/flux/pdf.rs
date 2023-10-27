use std::f32::consts::PI;

use glam::Vec3;
use rand::rngs::StdRng;

use super::{onb::Onb, random_cosine_direction, random_unit_vector};

pub trait Pdf {
    fn value(&self, _direction: Vec3) -> f32 {
        0.0
    }

    fn generate(&self, rng: &mut StdRng) -> Vec3;
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f32 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self, rng: &mut StdRng) -> Vec3 {
        random_unit_vector(rng)
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: Onb::from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f32 {
        // INFO: We assume the ray direction is normalized
        let cosine_theta = direction.dot(self.uvw.w);
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self, rng: &mut StdRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}
