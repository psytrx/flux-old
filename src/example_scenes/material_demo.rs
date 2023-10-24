use std::rc::Rc;

use glam::{uvec2, vec3};

use crate::flux::{
    Camera, DielectricMaterial, MatteMaterial, MetalMaterial, Primitive, Scene, Sphere,
};

pub fn material_demo() -> Scene {
    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(0.0, 2.0, -8.0);
        let look_at = vec3(0.0, 0.5, 0.0);
        let fov = 45.0;

        Camera::new(
            resolution,
            look_from,
            look_at,
            fov,
            0.025,
            look_at.distance(look_from),
        )
    };

    let mat_floor = Rc::new(MatteMaterial::new(vec3(0.8, 0.8, 0.0)));
    let mat_left = Rc::new(DielectricMaterial::new(1.5));
    let mat_center = Rc::new(MatteMaterial::new(vec3(0.1, 0.2, 0.5)));
    let mat_right = Rc::new(MetalMaterial::new(vec3(0.8, 0.6, 0.2), 0.05));

    let floor_sphere = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, mat_floor.clone())
    };

    let left_sphere = {
        let shape = Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_left.clone())
    };

    let center_sphere = {
        let shape = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_center.clone())
    };

    let right_sphere = {
        let shape = Sphere::new(vec3(2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_right.clone())
    };

    let aggregate = vec![floor_sphere, left_sphere, center_sphere, right_sphere];

    Scene::new(camera, aggregate)
}
