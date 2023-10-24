use std::rc::Rc;

use glam::{uvec2, vec3};

use crate::flux::{Camera, MatteMaterial, Primitive, Scene, Sphere};

pub fn defocus_blur() -> Scene {
    let n = 32;

    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(-3.0, 3.0, -1.0);
        let look_at = vec3(0.0, 0.0, (n / 4) as f32);
        let fov = 45.0;

        Camera::new(resolution, look_from, look_at, fov, 0.2, (n / 2) as f32)
    };

    let mat = Rc::new(MatteMaterial::new(vec3(0.5, 0.5, 0.5)));
    let mat_primary = Rc::new(MatteMaterial::new(vec3(0.8, 0.2, 0.2)));

    let mut aggregate = Vec::with_capacity(n);
    for z in 0..n {
        let sphere = {
            let shape = Sphere::new(vec3(0.0, 0.0, z as f32), 1.0);
            let mat = if z == n / 2 {
                mat_primary.clone()
            } else {
                mat.clone()
            };
            Primitive::new(shape, mat)
        };
        aggregate.push(sphere);
    }

    Scene::new(camera, aggregate)
}
