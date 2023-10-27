mod perspective;

pub use perspective::*;

use glam::UVec2;

use super::{ray::Ray, CameraSample};

pub trait Camera {
    fn resolution(&self) -> UVec2;
    fn ray(&self, sample: &CameraSample) -> Ray;
}
