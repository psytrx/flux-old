use glam::{vec3, Vec3};

use crate::flux::interaction::Interaction;

use super::Texture;

pub struct UvTexture {}

impl UvTexture {
    pub fn new() -> Self {
        Self {}
    }
}

impl Texture<Vec3> for UvTexture {
    fn evaluate(&self, int: &Interaction) -> Vec3 {
        vec3(int.uv.x, 0.0, int.uv.y)
    }
}
