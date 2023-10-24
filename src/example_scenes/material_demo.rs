use glam::{uvec2, vec3};

use crate::flux::{Camera, Scene};

use super::material_demo_primitives;

pub fn material_demo() -> Scene {
    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(0.0, 2.0, -8.0);
        let look_at = vec3(0.0, 2.0, 0.0);
        let fov = 70.0;

        Camera::new(
            resolution,
            look_from,
            look_at,
            fov,
            0.025,
            look_at.distance(look_from),
        )
    };

    let aggregate = material_demo_primitives();

    Scene::new(camera, aggregate)
}
