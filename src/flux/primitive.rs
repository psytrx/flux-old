use std::rc::Rc;

use embree4_sys::{RTCDevice, RTCGeometry};

use super::{materials::Material, shapes::Sphere};

pub struct Primitive {
    pub shape: Sphere,
    pub material: Rc<dyn Material>,
}

impl Primitive {
    pub fn new(shape: Sphere, material: Rc<dyn Material>) -> Self {
        Self { shape, material }
    }

    pub unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry {
        self.shape.build_geometry(device)
    }
}
