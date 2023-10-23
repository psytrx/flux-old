use std::ops::Range;

use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::Vec3;

use super::{interaction::Interaction, ray::Ray};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersect(&self, ray: &Ray, t: Range<f32>) -> Option<Interaction> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt_d = discriminant.sqrt();
            let mut root = (-half_b - sqrt_d) / a;
            if !t.contains(&root) {
                root = (-half_b + sqrt_d) / a;
                if !t.contains(&root) {
                    return None;
                }
            }

            let t = root;
            let p = ray.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = ray.direction.dot(outward_normal) < 0.0;
            let n = if front_face {
                outward_normal
            } else {
                -outward_normal
            };

            Some(Interaction {
                t,
                p,
                n,
                front_face,
            })
        }
    }

    pub unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry {
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
}
