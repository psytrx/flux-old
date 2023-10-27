mod floor;
mod quad;
mod quadbox;
mod sphere;
mod subdivision;
mod transform;
mod trimesh;

pub use floor::*;
pub use quad::*;
pub use quadbox::*;
use rand::rngs::StdRng;
pub use sphere::*;
pub use subdivision::*;
pub use transform::*;
pub use trimesh::*;

use embree4_sys::{RTCDevice, RTCGeometry};
use glam::{Vec2, Vec3};

use super::interaction::Interaction;

pub trait Shape {
    unsafe fn build_geometry(&self, id: u32, device: RTCDevice) -> RTCGeometry;

    fn uv(&self, p: Vec3) -> Vec2;

    fn adjust_interaction(&self, _int: &mut Interaction) {}

    fn sample_point(&self, _origin: Vec3, _rng: &mut StdRng) -> Vec3 {
        todo!()
    }

    fn area(&self) -> f32 {
        todo!()
    }
}
