use std::ptr::null_mut;

use embree4_sys::{rtcIntersect1, RTCRay, RTCRayHit, RTC_INVALID_GEOMETRY_ID};
use glam::vec3;

use super::{
    accel::EmbreeAccel, camera::Camera, interaction::Interaction, lights::Light,
    primitive::Primitive, ray::Ray,
};

pub struct Scene {
    primitives: Vec<Primitive>,
    pub accel: EmbreeAccel,
    pub camera: Camera,
    pub lights: Vec<Box<dyn Light>>,
}

// TODO: This is currently required for the progressive renderer to share the scene between
// threads. However, we may be able to clone it. Take care of the shared resources between
// the Embree API, since it counts references and can't know we copied/cloned the reference.
unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(camera: Camera, primitives: Vec<Primitive>, lights: Vec<Box<dyn Light>>) -> Self {
        let accel = unsafe { EmbreeAccel::build(&primitives) };
        Self {
            primitives,
            accel,
            camera,
            lights,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let mut ray_hit = RTCRayHit {
            ray: RTCRay::from(ray),
            hit: Default::default(),
        };

        unsafe { rtcIntersect1(self.accel.scene, &mut ray_hit, null_mut()) };

        (ray_hit.hit.geomID != RTC_INVALID_GEOMETRY_ID).then(|| {
            let t = ray_hit.ray.tfar;
            let p = ray.at(t);

            let n = vec3(ray_hit.hit.Ng_x, ray_hit.hit.Ng_y, ray_hit.hit.Ng_z).normalize();
            let front_face = ray.direction.dot(n) < 0.0;
            let n = if front_face { n } else { -n };

            let prim_idx = ray_hit.hit.geomID as usize;
            let primitive = &self.primitives[prim_idx];

            let uv = primitive.shape.uv(p);

            let mut int = Interaction {
                t,
                p,
                n,
                front_face,
                time: ray.time,
                uv,
                primitive,
            };
            primitive.shape.adjust_interaction(&mut int);

            int
        })
    }
}
