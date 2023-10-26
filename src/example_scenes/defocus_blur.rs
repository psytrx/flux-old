use glam::{uvec2, vec3};

use crate::flux::{Camera, Scene};

use super::{default_sky_light, util::material_demo_aggregate};

pub fn defocus_blur() -> Scene {
    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(-8.0, 4.0, -4.0);
        let look_at = vec3(0.0, 1.0, 0.0);

        Camera::new(
            resolution,
            look_from,
            look_at,
            35.0,
            0.3,
            look_at.distance(look_from),
        )
    };

    let aggregate = material_demo_aggregate();
    let lights = vec![default_sky_light()];

    Scene::new(camera, aggregate, lights)
}
