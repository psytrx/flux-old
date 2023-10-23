#![feature(float_next_up_down)]

mod flux;

use std::rc::Rc;

use anyhow::Result;
use flux::{Camera, MatteMaterial, Primitive, Scene, Sphere};
use glam::{uvec2, vec3, Vec3};
use log::debug;
use measure_time::trace_time;
use num_format::{Locale, ToFormattedString};

use crate::flux::{Denoiser, Renderer};

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
    std::fs::copy("./output/output.png", "./output/output-raw.png")?;

    unsafe {
        let denoiser = Denoiser::new(scene.camera.resolution);
        let denoised = denoiser.denoise(&result.film);
        denoised.to_srgb_image().save("./output/output.png")?;
    }

    Ok(())
}

fn load_scene() -> Scene {
    let resolution = uvec2(800, 450);
    let camera = Camera::new(resolution);

    let default_mat = Rc::new(MatteMaterial::new(Vec3::splat(0.5)));

    let floor_sphere = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, default_mat.clone())
    };

    let center_sphere = {
        let shape = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, default_mat.clone())
    };

    let small_sphere = {
        let shape = Sphere::new(vec3(1.0, 0.25, -1.0), 0.25);
        Primitive::new(shape, default_mat.clone())
    };

    let aggregate = vec![floor_sphere, center_sphere, small_sphere];

    Scene::new(camera, aggregate)
}
