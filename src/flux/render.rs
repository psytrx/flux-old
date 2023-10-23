use std::sync::atomic::{AtomicUsize, Ordering};

use glam::{vec2, vec3, Vec2, Vec3};

use log::debug;
use num_format::{Locale, ToFormattedString};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{film::Film, ray::Ray, uniform_sample_sphere, CameraSample, Scene};

pub struct Renderer {
    samples_per_pixel: usize,
    max_depth: usize,
    rays: AtomicUsize,
}

impl Renderer {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self {
            samples_per_pixel,
            max_depth: 16,
            rays: AtomicUsize::new(0),
        }
    }

    pub fn render_film(&self, scene: &Scene) -> Film {
        let mut film = Film::new(scene.camera.resolution);
        let mut rng = StdRng::seed_from_u64(0);

        let t0 = std::time::Instant::now();
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
        let elapsed = t0.elapsed();

        let rays = self.rays.load(Ordering::Relaxed);
        debug!("rays:     {:>16}", rays.to_formatted_string(&Locale::en));

        let rays_per_sec = (rays as f32 / elapsed.as_secs_f32()) as usize;
        debug!(
            "rays/sec: {:>16}",
            rays_per_sec.to_formatted_string(&Locale::en)
        );

        film
    }

    fn pixel_color(
        &self,
        scene: &Scene,
        ray: &Ray,
        rng: &mut StdRng,
        depth: usize,
    ) -> Option<Vec3> {
        if depth > self.max_depth {
            return None;
        }

        self.rays.fetch_add(1, Ordering::Relaxed);
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

fn is_near_zero(v: Vec3) -> bool {
    let s = f32::EPSILON;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
