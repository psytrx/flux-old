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
        let uv = int.primitive.shape.uv(int.p);
        vec3(uv.x, 0.0, uv.y)
    }
}
