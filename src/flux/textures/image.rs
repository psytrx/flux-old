use glam::{vec3, Vec3};
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
        let x = int.uv.x * self.img.width() as f32;
        let y = (1.0 - int.uv.y) * self.img.height() as f32;

        match self.img.get_pixel(x as u32, y as u32) {
            Rgba([r, g, b, _a]) => vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0),
        }
    }
}
