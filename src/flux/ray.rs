use embree4_sys::RTCRay;
use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl From<&Ray> for RTCRay {
    fn from(ray: &Ray) -> Self {
        RTCRay {
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
        }
    }
}
