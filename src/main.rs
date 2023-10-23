#![feature(float_next_up_down)]

mod flux;

use anyhow::Result;
use flux::{Camera, Scene, Sphere};
use glam::{uvec2, vec3};
use log::debug;
use measure_time::trace_time;
use num_format::{Locale, ToFormattedString};

use crate::flux::Renderer;

fn main() -> Result<()> {
    env_logger::init();
    trace_time!("main");

    let sweeps = 1;
    let num_cpus = num_cpus::get();
    let num_passes = sweeps * num_cpus;

    let scene = load_scene();
    let renderer = Renderer::new(1, 16, num_passes);

    let t0 = std::time::Instant::now();
    let result = renderer.render_film(&scene);
    let elapsed = t0.elapsed();

    debug!(
        "rays:     {:>16}",
        result.rays.to_formatted_string(&Locale::en)
    );
    let rays_per_sec = (result.rays as f64 / elapsed.as_secs_f64()) as usize;
    debug!(
        "rays/sec: {:>16}",
        rays_per_sec.to_formatted_string(&Locale::en)
    );

    result.film.to_srgb_image().save("./output/output.png")?;

    Ok(())
}

fn load_scene() -> Scene {
    let resolution = uvec2(800, 450);
    let camera = Camera::new(resolution);

    let aggregate = vec![
        Sphere::new(vec3(0.0, 1.0, 0.0), 1.0),
        Sphere::new(vec3(1.0, 0.25, -1.0), 0.25),
        Sphere::new(vec3(0.0, -100.0, 0.0), 100.0),
    ];

    Scene::new(camera, aggregate)
}
