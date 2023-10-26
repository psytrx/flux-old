use glam::{uvec2, vec3};

use super::default_sky_light;
use super::util::material_demo_aggregate;

use crate::flux::Camera;
use crate::flux::Scene;

pub fn material_demo() -> Scene {
    let camera = {
        let resolution = uvec2(1024, 1024);
        let look_from = vec3(0.0, 4.0, -8.0);
        let look_at = vec3(0.0, 2.0, 0.0);

        Camera::new(
            resolution,
            look_from,
            look_at,
            50.0,
            0.025,
            look_at.distance(look_from),
        )
    };

    let aggregate = material_demo_aggregate();
    let lights = vec![default_sky_light()];

    Scene::new(camera, aggregate, lights)
}
