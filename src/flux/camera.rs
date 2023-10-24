use glam::{vec3, Mat4, UVec2, Vec3};

use super::{ray::Ray, uniform_sample_disk, CameraSample};

pub struct Camera {
    pub resolution: UVec2,
    theta_x: f32,
    theta_y: f32,
    view_matrix: Mat4,
    lens_radius: f32,
    focus_dist: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(
        resolution: UVec2,
        position: Vec3,
        look_at: Vec3,
        fov: f32,
        lens_radius: f32,
        focus_dist: f32,
    ) -> Self {
        let view_matrix = Mat4::look_at_lh(position, look_at, Vec3::Y).inverse();

        let aspect_ratio = resolution.x as f32 / resolution.y as f32;

        let theta_x = (fov / 2.0).to_radians().tan();
        let theta_y = theta_x / aspect_ratio;

        Self {
            resolution,
            theta_x,
            theta_y,
            view_matrix,
            lens_radius,
            focus_dist,
        }
    }

    pub fn ray(&self, sample: &CameraSample) -> Ray {
        let uv = sample.p_film / self.resolution.as_vec2();

        let p_lens = self.lens_radius * uniform_sample_disk(sample.p_lens);
        let origin = p_lens.extend(0.0);

        // point on the near plane, offset by uv coordinates
        let near_plane_target = vec3(
            -self.theta_x + 2.0 * self.theta_x * uv.x,
            self.theta_y - 2.0 * self.theta_y * uv.y,
            1.0,
        );

        let ft = self.focus_dist / near_plane_target.z;
        let p_focus = near_plane_target * ft;

        let direction = p_focus - origin;
        let direction = self.view_matrix.transform_vector3(direction).normalize();

        let origin = self.view_matrix.transform_point3(origin);

        Ray::new(origin, direction, sample.time)
    }
}
