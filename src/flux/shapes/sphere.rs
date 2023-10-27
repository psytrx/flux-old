use std::f32::consts::PI;

use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::{vec2, vec3, Vec2, Vec3};
use rand::{rngs::StdRng, Rng};

use crate::flux::uniform_sample_sphere;

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

    fn area(&self) -> f32 {
        todo!()
    }

    fn sample_point(&self, _origin: Vec3, rng: &mut StdRng) -> Vec3 {
        self.center + self.radius * uniform_sample_sphere(rng.gen())
    }

    fn pdf_value(&self, _origin: Vec3, _direction: Vec3) -> f32 {
        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - _origin).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
}

fn random_to_sphere(radius: f32, distance_squared: f32, rng: &mut StdRng) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    vec3(x, y, z)
}
