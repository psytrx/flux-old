use std::ops::Range;

use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t: Range<f32>,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            t: f32::EPSILON..f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
