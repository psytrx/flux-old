use std::ptr::null_mut;

use embree4_sys::{rtcIntersect1, RTCRay, RTCRayHit, RTC_INVALID_GEOMETRY_ID};
use glam::vec3;

use super::{accel::EmbreeAccel, camera::Camera, interaction::Interaction, ray::Ray, Sphere};

pub struct Scene {
    pub camera: Camera,
    pub accel: EmbreeAccel,
}

impl Scene {
    pub fn new(camera: Camera, aggregate: Vec<Sphere>) -> Self {
        let accel = unsafe { EmbreeAccel::build(aggregate) };
        Self { camera, accel }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let mut ray_hit = RTCRayHit {
            ray: RTCRay {
                org_x: ray.origin.x,
                org_y: ray.origin.y,
                org_z: ray.origin.z,
                tnear: f32::EPSILON,
                dir_x: ray.direction.x,
                dir_y: ray.direction.y,
                dir_z: ray.direction.z,
                time: 0.0,
                tfar: f32::INFINITY,
                ..Default::default()
            },
            hit: Default::default(),
        };

        unsafe { rtcIntersect1(self.accel.scene, &mut ray_hit, null_mut()) };

        (ray_hit.hit.geomID != RTC_INVALID_GEOMETRY_ID).then(|| {
            let t = ray_hit.ray.tfar;
            let p = ray.at(t);

            let n = vec3(ray_hit.hit.Ng_x, ray_hit.hit.Ng_y, ray_hit.hit.Ng_z).normalize();
            let front_face = ray.direction.dot(n) < 0.0;

            Interaction {
                t,
                p,
                n,
                front_face,
                time: ray.time,
            }
        })
    }
}
