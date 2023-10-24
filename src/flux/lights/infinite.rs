use glam::Vec3;

use crate::flux::ray::Ray;

use super::Light;

pub struct InfiniteAreaLight {
    emit: Vec3,
}

#[allow(dead_code)]
impl InfiniteAreaLight {
    pub fn new(emit: Vec3) -> Self {
        Self { emit }
    }
}

impl Light for InfiniteAreaLight {
    fn le(&self, _ray: &Ray) -> Vec3 {
        self.emit
    }
}
