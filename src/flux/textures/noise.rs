use glam::Vec3;
use noise::{
    core::perlin::perlin_2d,
    permutationtable::PermutationTable,
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
};

use crate::flux::interaction::Interaction;

use super::Texture;

pub struct NoiseTexture {
    noise: NoiseMap,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        let hasher = PermutationTable::new(0);
        let noise = PlaneMapBuilder::new_fn(|point, _| perlin_2d(point, &hasher), &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(0.0, (1.0 / scale) as f64)
            .set_y_bounds(0.0, (1.0 / scale) as f64)
            .build();
        Self { noise }
    }
}

impl Texture<Vec3> for NoiseTexture {
    fn evaluate(&self, int: &Interaction) -> Vec3 {
        let (width, height) = self.noise.size();
        let x = (int.uv.x * width as f32) as usize;
        let y = (int.uv.y * height as f32) as usize;

        let v = self.noise.get_value(x, y) as f32;
        // Map [-1, 1] to [0, 1]
        let v = (v + 1.0) / 2.0;
        Vec3::splat(v)
    }
}
