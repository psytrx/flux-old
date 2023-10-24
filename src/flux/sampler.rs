use glam::{vec2, Vec2};
use log::warn;
use rand::{rngs::StdRng, Rng};

use super::CameraSample;

pub struct StratifiedSampler {
    nx_ny: usize,
}

impl StratifiedSampler {
    pub fn new(samples_per_pixel: usize) -> Self {
        let nx_ny = ((samples_per_pixel as f32).sqrt().floor() as usize).max(1);

        if nx_ny * nx_ny != samples_per_pixel {
            let new_samples_per_pixel = nx_ny * nx_ny;
            warn!("StratifiedSampler exptects samples_per_pixel to be a perfect square number, but got {}. Falling back to {}",
                samples_per_pixel, new_samples_per_pixel)
        }

        Self { nx_ny }
    }

    pub fn camera_samples(&self, p_raster: Vec2, rng: &mut StdRng) -> Vec<CameraSample> {
        let mut samples = Vec::with_capacity(self.nx_ny.pow(2));

        for y in 0..self.nx_ny {
            for x in 0..self.nx_ny {
                let grid_offset =
                    (vec2(x as f32, y as f32) + rng.gen::<Vec2>()) / self.nx_ny as f32;
                let p_film = p_raster + grid_offset;

                let p_lens = rng.gen::<Vec2>();
                let time = rng.gen();

                let sample = CameraSample {
                    p_film,
                    p_lens,
                    time,
                };

                samples.push(sample)
            }
        }

        samples
    }
}
