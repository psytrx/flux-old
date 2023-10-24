mod floor;
mod quad;
mod sphere;

use embree4_sys::{RTCDevice, RTCGeometry};
pub use floor::Floor;
use glam::{Vec2, Vec3};
pub use quad::Quad;
pub use sphere::Sphere;

pub trait Shape {
    unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry;
    fn uv(&self, p: Vec3) -> Vec2;
}
