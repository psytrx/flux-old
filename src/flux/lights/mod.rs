mod infinite;
mod sky;

pub use infinite::*;
pub use sky::*;

use glam::Vec3;

use super::ray::Ray;

pub trait Light {
    fn le(&self, ray: &Ray) -> Vec3;
}
