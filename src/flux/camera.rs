use glam::{vec3, UVec2};

use super::{ray::Ray, CameraSample};

pub struct Camera {
    pub resolution: UVec2,
}

impl Camera {
    pub fn new(resolution: UVec2) -> Self {
        Self { resolution }
    }

    pub fn ray(&self, sample: &CameraSample) -> Ray {
        let look_from = vec3(0.0, 1.5, -4.0);
        let viewport_width = 6.0;
        let viewport_height = viewport_width / self.resolution.x as f32 * self.resolution.y as f32;
        let top_left = vec3(-viewport_width / 2.0, viewport_height / 2.0 + 0.5, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, -viewport_height, 0.0);

        let origin = look_from;

        let uv = sample.p_film / self.resolution.as_vec2();
        let target = top_left + horizontal * uv.x + vertical * uv.y;
        let direction = target - origin;

        Ray::new(origin, direction, sample.time)
    }
}
