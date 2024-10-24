use std::f32::consts::PI;

use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::{vec2, Vec2, Vec3};

use super::Shape;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    unsafe fn build_geometry(&self, _id: u32, device: RTCDevice) -> RTCGeometry {
        let geometry = rtcNewGeometry(device, RTCGeometryType::SPHERE_POINT);

        let buffer_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::VERTEX,
            0,
            RTCFormat::FLOAT4,
            4 * std::mem::size_of::<f32>(),
            1,
        );

        let buffer = std::slice::from_raw_parts_mut(buffer_ptr as *mut f32, 4);
        buffer.copy_from_slice(&[self.center.x, self.center.y, self.center.z, self.radius]);

        geometry
    }

    fn uv(&self, p: Vec3) -> Vec2 {
        let oc = p - self.center;

        let theta = (-oc.y / self.radius).acos();
        let phi = (-oc.z).atan2(oc.x) + PI;

        vec2(phi / (2.0 * PI), theta / PI)
    }
}
