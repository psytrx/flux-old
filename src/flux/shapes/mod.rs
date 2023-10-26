mod floor;
mod quad;
mod quadbox;
mod sphere;
mod subdivision;
mod transform;
mod trimesh;

pub use floor::Floor;
pub use quad::Quad;
pub use quadbox::QuadBox;
pub use sphere::Sphere;
pub use subdivision::SubdivisionMesh;
pub use transform::Transform;
pub use trimesh::TriangleMesh;

use embree4_sys::{RTCDevice, RTCGeometry};
use glam::{Vec2, Vec3};

use super::interaction::Interaction;

pub trait Shape {
    unsafe fn build_geometry(&self, id: u32, device: RTCDevice) -> RTCGeometry;

    fn uv(&self, p: Vec3) -> Vec2;

    fn adjust_interaction(&self, _int: &mut Interaction) {}
}
