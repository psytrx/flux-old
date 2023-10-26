use glam::{vec2, vec3, Vec3};
use image::{DynamicImage, GenericImageView, Rgba};


use crate::flux::interaction::Interaction;

use super::Texture;

pub struct ImageTexture {
    img: DynamicImage,
}

impl ImageTexture {
    pub fn new(img: DynamicImage) -> Self {
        Self { img }
    }
}

impl Texture<Vec3> for ImageTexture {
    fn evaluate(&self, int: &Interaction) -> Vec3 {
        let uv = {
            let uv = int.primitive.shape.uv(int.p);
            vec2(uv.x, 1.0 - uv.y)
        };

        let x = ((uv.x * self.img.width() as f32) as u32).clamp(0, self.img.width() - 1);
        let y = ((uv.y * self.img.height() as f32) as u32).clamp(0, self.img.height() - 1);

        let p = self.img.get_pixel(x, y);
        match p {
            Rgba([r, g, b, _]) => vec3(r as f32 * 255.0, g as f32 * 255.0, b as f32 * 255.0),
        }
    }
}
