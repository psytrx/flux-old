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
        let uv = int.primitive.shape.uv(int.p);

        let u = uv.x * self.img.width() as f32;
        let v = (1.0 - uv.y) * self.img.height() as f32;

        let x0 = u.floor();
        let y0 = v.floor();

        let s = u - x0;
        let t = v - y0;

        let p = self.img.get_pixel(x0 as u32, y0 as u32);
        let p_r = self
            .img
            .get_pixel((x0 as u32 + 1) % self.img.width(), y0 as u32);
        let p_b = self
            .img
            .get_pixel(x0 as u32, (y0 as u32 + 1) % self.img.height());

        match p {
            Rgba([pr, pg, pb, _]) => match p_r {
                Rgba([pr_r, pr_g, pr_b, _]) => match p_b {
                    Rgba([pb_r, pb_g, pb_b, _]) => {
                        let c_p = vec3(pr as f32, pg as f32, pb as f32) / 255.0;
                        let c_pr = vec3(pr_r as f32, pr_g as f32, pr_b as f32) / 255.0;
                        let c_pb = vec3(pb_r as f32, pb_g as f32, pb_b as f32) / 255.0;

                        let c_x = (1.0 - s) * c_p + s * c_pr;
                        let c_y = (1.0 - t) * c_p + t * c_pb;
                        (c_x + c_y) / 2.0
                    }
                },
            },
        }
    }
}
