mod floor;
mod sphere;

use embree4_sys::{RTCDevice, RTCGeometry};
pub use floor::Floor;
pub use sphere::Sphere;

pub trait Shape {
    unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry;
}
