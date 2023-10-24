use glam::Vec3;

use crate::flux::ray::Ray;

use super::Light;

pub struct SkyLight {
    horizon_color: Vec3,
    zenith_color: Vec3,
}

impl SkyLight {
    pub fn new(horizon_color: Vec3, zenith_color: Vec3) -> Self {
        Self {
            horizon_color,
            zenith_color,
        }
    }
}

impl Light for SkyLight {
    fn le(&self, ray: &Ray) -> Vec3 {
        // INFO: We assume ray directions are normalized
        let unit_direction = ray.direction;
        let a = unit_direction.y.abs();
        (1.0 - a) * self.zenith_color + a * self.horizon_color
    }
}
