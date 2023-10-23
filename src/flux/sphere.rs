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

    pub fn hit(&self, ray: &Ray) -> Option<Interaction> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt_d = discriminant.sqrt();
            let mut t = (-half_b - sqrt_d) / a;
            if t < ray.t.start || t > ray.t.end {
                t = (-half_b + sqrt_d) / a;
                if t < ray.t.start || t > ray.t.end {
                    return None;
                }
            }

            let p = ray.at(t);
            let n = (p - self.center) / self.radius;
            Some(Interaction { p, n })
        }
    }
}
