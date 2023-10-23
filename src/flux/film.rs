use glam::{vec3, UVec2, Vec2, Vec3};
use image::{Rgb, RgbImage};

#[derive(Debug)]
pub struct Film {
    pub resolution: UVec2,
    pixels: Vec<Pixel>,
}

impl Film {
    pub fn new(resolution: UVec2) -> Self {
        let buffer_size = (resolution.x * resolution.y) as usize;
        let pixels = vec![Pixel::ZERO; buffer_size];
        Self::from_pixels(resolution, pixels)
    }

    pub fn from_rgb_f32_slice(resolution: UVec2, data: &mut [f32]) -> Self {
        let pixels = data
            .chunks_exact(3)
            .map(|chunk| Pixel {
                color_sum: match chunk {
                    [r, g, b] => vec3(*r, *g, *b),
                    _ => panic!("Invalid chunk size"),
                },
                weight_sum: 1.0,
            })
            .collect();

        Self::from_pixels(resolution, pixels)
    }

    fn from_pixels(resolution: UVec2, pixels: Vec<Pixel>) -> Self {
        Self { resolution, pixels }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        assert!(x <= self.resolution.x && y <= self.resolution.y);

        let x = x.min(self.resolution.x - 1);
        let y = y.min(self.resolution.y - 1);

        (y * self.resolution.x + x) as usize
    }

    pub fn pixel(&self, x: u32, y: u32) -> &Pixel {
        let index = self.index(x, y);
        &self.pixels[index]
    }

    fn pixel_mut(&mut self, x: u32, y: u32) -> &mut Pixel {
        let index = self.index(x, y);
        &mut self.pixels[index]
    }

    pub fn add_sample(&mut self, p_film: Vec2, color: Vec3, sample_weight: f32) {
        let pixel = self.pixel_mut(p_film.x as u32, p_film.y as u32);
        pixel.color_sum += color;
        pixel.weight_sum += sample_weight;
    }

    pub fn merge_tile(&mut self, p0: UVec2, tile: Film) {
        for y in 0..tile.resolution.y {
            for x in 0..tile.resolution.x {
                let tile_pixel = tile.pixel(x, y);
                let local_pixel = self.pixel_mut(p0.x + x, p0.y + y);
                local_pixel.color_sum += tile_pixel.color_sum;
                local_pixel.weight_sum += tile_pixel.weight_sum;
            }
        }
    }

    pub fn to_srgb_image(&self) -> RgbImage {
        RgbImage::from_fn(self.resolution.x, self.resolution.y, |x, y| {
            let pixel = self.pixel(x, y);
            let color = pixel.color();
            color_to_srgb(color)
        })
    }
}

fn color_to_srgb(color: Vec3) -> Rgb<u8> {
    // "gamma 2" correction
    let color = vec3(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

    let ir = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
    let ig = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
    let ib = (255.0 * color.z.clamp(0.0, 1.0)) as u8;

    Rgb([ir, ig, ib])
}

#[derive(Clone, Debug)]
pub struct Pixel {
    color_sum: Vec3,
    weight_sum: f32,
}

impl Pixel {
    pub const ZERO: Self = Self {
        color_sum: Vec3::ZERO,
        weight_sum: 0.0,
    };

    pub fn color(&self) -> Vec3 {
        if self.weight_sum == 0.0 {
            self.color_sum
        } else {
            self.color_sum / self.weight_sum
        }
    }
}
