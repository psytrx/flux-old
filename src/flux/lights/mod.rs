mod infinite;
mod sky;

pub use infinite::InfiniteAreaLight;
pub use sky::SkyLight;

use glam::Vec3;

use super::ray::Ray;

pub trait Light {
    fn le(&self, ray: &Ray) -> Vec3;
}
