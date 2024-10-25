use std::sync::{Arc, Mutex};

use glam::{vec2, UVec2};
use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::{
    film::Film, integrators::Integrator, sampler::StratifiedSampler, updater::RenderUpdateEvent,
    RenderUpdater, Scene,
};

pub struct Renderer {
    integrator: Box<dyn Integrator>,
    sampler: StratifiedSampler,
    num_passes: usize,
    updater: Option<Box<dyn RenderUpdater>>,
}

impl Renderer {
    pub fn new(
        integrator: Box<dyn Integrator>,
        sampler: StratifiedSampler,
        num_passes: usize,
        updater: Option<Box<dyn RenderUpdater>>,
    ) -> Self {
        Self {
            integrator,
            sampler,
            num_passes,
            updater,
        }
    }

    pub fn render_film(&self, scene: &Scene) -> RenderResult {
        let render_passes = (0..self.num_passes)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|pass| self.render_pass(scene, pass));

        let resolution = scene.camera.resolution();

        let shared = Arc::new(Mutex::new(SharedState {
            merged_film: Film::new(resolution),
            passes_merged: 0,
            total_rays: 0,
        }));

        render_passes.for_each(|result| {
            let mut shared = shared.lock().unwrap();
            shared.merged_film.merge_tile(UVec2::ZERO, result.film);
            shared.passes_merged += 1;
            shared.total_rays += result.rays;

            if let Some(updater) = &self.updater {
                if updater.should_update(shared.passes_merged, self.num_passes) {
                    let progress_percent =
                        100.0 * (shared.passes_merged as f32 / self.num_passes as f32);
                    let event = RenderUpdateEvent {
                        current_pass: shared.passes_merged,
                        total_passes: self.num_passes,
                        progress_percent,
                        film: shared.merged_film.clone(),
                    };
                    updater.update(event);
                }
            }
        });

        let shared = Arc::try_unwrap(shared).unwrap().into_inner().unwrap();
        RenderResult {
            film: shared.merged_film,
            rays: shared.total_rays,
        }
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

#[derive(Debug)]
struct SharedState {
    merged_film: Film,
    passes_merged: usize,
    total_rays: usize,
}

pub struct RenderResult {
    pub film: Film,
    pub rays: usize,
}
