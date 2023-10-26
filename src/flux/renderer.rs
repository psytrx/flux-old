use std::sync::{Arc, Mutex};

use glam::{vec2, UVec2};
use log::debug;
use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::{film::Film, integrators::Integrator, sampler::StratifiedSampler, Scene};

pub struct Renderer {
    integrator: Box<dyn Integrator>,
    sampler: StratifiedSampler,
    num_passes: usize,
}

impl Renderer {
    pub fn new(
        integrator: Box<dyn Integrator>,
        sampler: StratifiedSampler,
        num_passes: usize,
    ) -> Self {
        Self {
            integrator,
            sampler,
            num_passes,
        }
    }

    pub fn render_film(&self, scene: &Scene) -> RenderResult {
        let finished_passes = (0..self.num_passes)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|pass| self.render_pass(scene, pass));

        let resolution = scene.camera.resolution();

        let merged_film = Arc::new(Mutex::new(Film::new(resolution)));
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

            if *passes_merged % num_cpus == 0 && *passes_merged < self.num_passes {
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
        let resolution = scene.camera.resolution();

        let mut film = Film::new(resolution);
        let mut rng = StdRng::seed_from_u64(pass.try_into().unwrap());
        let mut rays = 0;

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let p_raster = vec2(x as f32, y as f32);

                let camera_samples = self.sampler.camera_samples(p_raster, &mut rng);
                for sample in camera_samples {
                    let ray = scene.camera.ray(&sample);

                    let result = self.integrator.li(scene, &ray, &mut rng);
                    film.add_sample(sample.p_film, result.li, 1.0);
                    rays += result.rays;
                }
            }
        }

        RenderResult { film, rays }
    }
}

pub struct RenderResult {
    pub film: Film,
    pub rays: usize,
}
