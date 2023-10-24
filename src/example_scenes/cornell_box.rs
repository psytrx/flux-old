use glam::{uvec2, vec3, Vec3};

use crate::flux::{Camera, Scene};

use super::empty_cornell_box_primitives;

pub fn cornell_box() -> Scene {
    let box_size = 100.0;

    let camera = {
        let resolution = uvec2(1024, 1024);
        let look_from = vec3(0.0, 0.0, -1.7 * box_size);
        let look_at = Vec3::ZERO;
        let fov = 45.0;

        Camera::new(
            resolution,
            look_from,
            look_at,
            fov,
            0.3,
            look_at.distance(look_from),
        )
    };

    let aggregate = empty_cornell_box_primitives(box_size);

    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}
