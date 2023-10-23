use std::rc::Rc;

use glam::{uvec2, vec3, Vec3};

use crate::flux::{Camera, MatteMaterial, Primitive, Scene, Sphere};

pub fn material_demo() -> Scene {
    let resolution = uvec2(800, 450);
    let camera = Camera::new(resolution);

    let white = Rc::new(MatteMaterial::new(Vec3::ONE));
    let red_mat = Rc::new(MatteMaterial::new(Vec3::new(0.5, 0.0, 0.0)));
    let blue_mat = Rc::new(MatteMaterial::new(Vec3::new(0.0, 0.0, 0.5)));

    let floor_sphere = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, white.clone())
    };

    let left_sphere = {
        let shape = Sphere::new(vec3(-1.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, red_mat.clone())
    };

    let right_sphere = {
        let shape = Sphere::new(vec3(1.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, blue_mat.clone())
    };

    let aggregate = vec![floor_sphere, left_sphere, right_sphere];

    Scene::new(camera, aggregate)
}
