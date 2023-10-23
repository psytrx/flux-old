use glam::{vec3, Mat4, UVec2, Vec3};

use super::{ray::Ray, CameraSample};

pub struct Camera {
    pub resolution: UVec2,
    theta_x: f32,
    theta_y: f32,
    view_matrix: Mat4,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(resolution: UVec2, position: Vec3, look_at: Vec3, fov: f32) -> Self {
        let view_matrix = Mat4::look_at_lh(position, look_at, Vec3::Y).inverse();

        let aspect_ratio = resolution.x as f32 / resolution.y as f32;

        let theta_x = (fov / 2.0).to_radians().tan();
        let theta_y = theta_x / aspect_ratio;

        Self {
            resolution,
            theta_x,
            theta_y,
            view_matrix,
        }
    }

    pub fn ray(&self, sample: &CameraSample) -> Ray {
        let uv = sample.p_film / self.resolution.as_vec2();

        // point on the near plane, offset by uv coordinates
        let near_plane_target = vec3(
            -self.theta_x + 2.0 * self.theta_x * uv.x,
            self.theta_y - 2.0 * self.theta_y * uv.y,
            1.0,
        );

        let direction = self
            .view_matrix
            .transform_vector3(near_plane_target)
            .normalize();

        let origin = self.view_matrix.transform_point3(Vec3::ZERO);

        Ray::new(origin, direction, sample.time)
    }
}
