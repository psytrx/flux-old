use std::sync::{Arc, Mutex};

use glam::{vec2, vec3, UVec2, Vec2, Vec3};

use log::trace;

use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::{film::Film, ray::Ray, uniform_sample_sphere, CameraSample, Scene};

pub struct Renderer {
    samples_per_pixel: u32,
    max_depth: u32,
    num_passes: u64,
}

impl Renderer {
    pub fn new(samples_per_pixel: u32, max_depth: u32, num_passes: u64) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
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

        let num_cpus = num_cpus::get();

        finished_passes.for_each(|result| {
            let mut merged_film = merged_film.lock().unwrap();
            merged_film.merge_tile(UVec2::ZERO, result.film);

            let mut passes_merged = passes_merged.lock().unwrap();
            *passes_merged += 1;

            if *passes_merged % num_cpus == 0 {
                let mut last_update = last_update.lock().unwrap();
                if last_update.elapsed() > std::time::Duration::from_secs(1) {
                    merged_film
                        .to_srgb_image()
                        .save("./output/output.png")
                        .unwrap();
                    *last_update = std::time::Instant::now();

                    let progress = 100.0 * (*passes_merged as f32 / self.num_passes as f32);
                    trace!(
                        "{} / {} ({:>6.3}%)",
                        passes_merged,
                        self.num_passes,
                        progress
                    );
                }
            }
        });

        let film = Arc::try_unwrap(merged_film).unwrap().into_inner().unwrap();

        RenderResult { film }
    }

    fn render_pass(&self, scene: &Scene, pass: u64) -> RenderResult {
        let mut film = Film::new(scene.camera.resolution);
        let mut rng = StdRng::seed_from_u64(pass);

        for y in 0..scene.camera.resolution.y {
            for x in 0..scene.camera.resolution.x {
                let p_raster = vec2(x as f32, y as f32);

                for _ in 0..self.samples_per_pixel {
                    let p_film = p_raster + rng.gen::<Vec2>();

                    let camera_sample = CameraSample {
                        p_film,
                        time: rng.gen(),
                    };
                    let ray = scene.camera.ray(&camera_sample);

                    if let Some(color) = self.pixel_color(scene, &ray, &mut rng, 0) {
                        film.add_sample(p_film, color, 1.0);
                    }
                }
            }
        }

        RenderResult { film }
    }

    fn pixel_color(&self, scene: &Scene, ray: &Ray, rng: &mut StdRng, depth: u32) -> Option<Vec3> {
        if depth > self.max_depth {
            return None;
        }

        match scene.intersect(ray) {
            None => {
                let unit_direction = ray.direction.normalize();
                let a = (unit_direction.y + 1.0) / 2.0;
                let horizon_color = vec3(0.5, 0.7, 1.0);
                let zenith_color = vec3(1.0, 1.0, 1.0);
                Some((1.0 - a) * zenith_color + a * horizon_color)
            }
            Some(int) => {
                if int.front_face {
                    // surface normal shading:
                    // Some((int.n + 1.0) / 2.0)

                    let mut scattered_dir = int.n + uniform_sample_sphere(rng.gen());
                    if is_near_zero(scattered_dir) {
                        scattered_dir = int.n;
                    }
                    let scattered_ray = int.spawn_ray(scattered_dir);

                    let attenuation = Vec3::splat(0.5);
                    match self.pixel_color(scene, &scattered_ray, rng, depth + 1) {
                        Some(li) => Some(attenuation * li),
                        None => Some(attenuation),
                    }
                } else {
                    None
                }
            }
        }
    }
}

pub struct RenderResult {
    pub film: Film,
}

fn is_near_zero(v: Vec3) -> bool {
    let s = f32::EPSILON;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
