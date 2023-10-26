use glam::{vec2, vec3, Vec3};
use image::{DynamicImage, GenericImageView, Rgba};

use crate::flux::interaction::Interaction;

use super::Texture;

pub struct ImageTexture {
    width: u32,
    height: u32,
    cache: Vec<Vec3>,
}

impl ImageTexture {
    pub fn new(img: DynamicImage) -> Self {
        // build a directly accessible color cache
        let (width, height) = img.dimensions();
        let mut cache = vec![Vec3::ZERO; (width * height) as usize];
        for (x, y, pixel) in img.pixels() {
            let index = (y * width + x) as usize;
            let color = match pixel {
                Rgba([r, g, b, _]) => {
                    vec3((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
                }
            };
            cache[index] = color;
        }
        Self {
            width,
            height,
            cache,
        }
    }
}

impl Texture<Vec3> for ImageTexture {
    fn evaluate(&self, int: &Interaction) -> Vec3 {
        let uv = {
            let uv = int.primitive.shape.uv(int.p);
            // flip y coordinate to image coordinate space
            vec2(uv.x, 1.0 - uv.y)
        };

        let x = (uv.x * (self.width - 1) as f32) as u32;
        let y = (uv.y * (self.height - 1) as f32) as u32;
        let index = (y * self.width + x) as usize;

        self.cache[index]
    }
}
