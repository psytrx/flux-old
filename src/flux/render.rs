use std::sync::{Arc, Mutex};

use glam::{vec2, vec3, UVec2, Vec3};
use log::debug;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::{film::Film, ray::Ray, sampler::StratifiedSampler, Scene};

pub struct Renderer {
    sampler: StratifiedSampler,
    min_depth: u32,
    max_depth: u32,
    rr_stop_prob: f32,
    num_passes: usize,
}

impl Renderer {
    pub fn new(
        sampler: StratifiedSampler,
        min_depth: u32,
        max_depth: u32,
        rr_stop_prob: f32,
        num_passes: usize,
    ) -> Self {
        Self {
            sampler,
            min_depth,
            max_depth,
            rr_stop_prob,
            num_passes,
        }
    }

    pub fn render_film(&self, scene: &Scene) -> RenderResult {
        let finished_passes = (0..self.num_passes)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|pass| self.render_pass(scene, pass));

        let merged_film = Arc::new(Mutex::new(Film::new(scene.camera.resolution)));
        let passes_merged = Arc::new(Mutex::new(0));
        let last_update = Arc::new(Mutex::new(std::time::Instant::now()));
        let rays = Arc::new(Mutex::new(0));

        let num_cpus = num_cpus::get();

        finished_passes.for_each(|result| {
            let mut merged_film = merged_film.lock().unwrap();
            merged_film.merge_tile(UVec2::ZERO, result.film);

            let mut passes_merged = passes_merged.lock().unwrap();
            *passes_merged += 1;

            let mut rays = rays.lock().unwrap();
            *rays += result.rays;

            if *passes_merged % num_cpus == 0 {
                let mut last_update = last_update.lock().unwrap();
                if last_update.elapsed() > std::time::Duration::from_secs(1) {
                    merged_film
                        .to_srgb_image()
                        .save("./output/output.png")
                        .unwrap();
                    *last_update = std::time::Instant::now();

                    let progress = 100.0 * (*passes_merged as f32 / self.num_passes as f32);
                    debug!(
                        "{} / {} ({:>6.3}%)",
                        passes_merged, self.num_passes, progress
                    );
                }
            }
        });

        let film = Arc::try_unwrap(merged_film).unwrap().into_inner().unwrap();
        let rays = Arc::try_unwrap(rays).unwrap().into_inner().unwrap();

        RenderResult { film, rays }
    }

    fn render_pass(&self, scene: &Scene, pass: usize) -> RenderResult {
        let mut film = Film::new(scene.camera.resolution);
        let mut rng = StdRng::seed_from_u64(pass.try_into().unwrap());
        let mut rays = 0;

        for y in 0..scene.camera.resolution.y {
            for x in 0..scene.camera.resolution.x {
                let p_raster = vec2(x as f32, y as f32);

                let camera_samples = self.sampler.camera_samples(p_raster, &mut rng);
                for sample in camera_samples {
                    let ray = scene.camera.ray(&sample);

                    let li = self.pixel_color(scene, &ray, &mut rng, 0);
                    film.add_sample(sample.p_film, li.color, 1.0);
                    rays += li.rays;
                }
            }
        }

        RenderResult { film, rays }
    }

    fn pixel_color(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng, depth: u32) -> ColorResult {
        if depth > self.max_depth {
            return ColorResult {
                color: Vec3::ZERO,
                rays: 0,
            };
        }

        let rr_factor = if depth >= self.min_depth {
            let q = 1.0 - self.rr_stop_prob;
            let s: f32 = rng.gen();
            if s < q {
                return ColorResult {
                    color: Vec3::ZERO,
                    rays: 0,
                };
            }
            1.0 / q
        } else {
            1.0
        };

        let rays = 1;
        match scene.intersect(ray) {
            None => {
                let unit_direction = ray.direction.normalize();
                let a = (unit_direction.y + 1.0) / 2.0;
                let horizon_color = vec3(0.5, 0.7, 1.0);
                let zenith_color = vec3(1.0, 1.0, 1.0);
                let color = (1.0 - a) * zenith_color + a * horizon_color;
                ColorResult {
                    color: rr_factor * color,
                    rays,
                }
            }
            Some(int) => match int.primitive.material.scatter(ray, &int, rng) {
                Some(srec) => {
                    let li = self.pixel_color(scene, &srec.scattered, rng, depth + 1);
                    ColorResult {
                        color: rr_factor * srec.attenuation * li.color,
                        rays: rays + li.rays,
                    }
                }
                None => ColorResult {
                    color: Vec3::ZERO,
                    rays,
                },
            },
        }
    }
}

struct ColorResult {
    color: Vec3,
    rays: usize,
}

pub struct RenderResult {
    pub film: Film,
    pub rays: usize,
}
