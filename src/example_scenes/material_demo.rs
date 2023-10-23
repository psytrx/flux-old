use std::rc::Rc;

use glam::{uvec2, vec3, Vec3};

use crate::flux::{Camera, MatteMaterial, Primitive, Scene, Sphere};

pub fn material_demo() -> Scene {
    let resolution = uvec2(800, 450);
    let camera = Camera::new(resolution);

    let default_mat = Rc::new(MatteMaterial::new(Vec3::splat(0.5)));

    let floor_sphere = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, default_mat.clone())
    };

    let center_sphere = {
        let shape = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, default_mat.clone())
    };

    let small_sphere = {
        let shape = Sphere::new(vec3(1.0, 0.25, -1.0), 0.25);
        Primitive::new(shape, default_mat.clone())
    };

    let aggregate = vec![floor_sphere, center_sphere, small_sphere];

    Scene::new(camera, aggregate)
}
