#![feature(float_next_up_down)]

mod example_scenes;
mod flux;

use anyhow::Result;
use flux::{integrators::Integrator, RenderResult, Scene};
use log::{debug, info};
use measure_time::{debug_time, trace_time};
use num_format::{Locale, ToFormattedString};

use crate::{
    example_scenes::{load_example_scene, ExampleScene},
    flux::{
        integrators::{AlbedoIntegrator, NormalIntegrator, PathTracingIntegrator},
        Denoiser, Renderer, StratifiedSampler,
    },
};

fn main() -> Result<()> {
    env_logger::init();

    let t0_main = std::time::Instant::now();

    let args = std::env::args().collect::<Vec<_>>();
    let debug_mode = args.contains(&String::from("--dev"));

    let sweeps = if debug_mode { 1 } else { 4 };
    let num_cpus = num_cpus::get();
    let num_passes = sweeps * num_cpus;

    let scene = {
        info!("loading scene...");
        load_example_scene(ExampleScene::CornellBox)
    };

    let renderer = {
        let integrator = Box::new(PathTracingIntegrator::new(8, 32, 0.1));

        let samples_per_pixel = if debug_mode { 1 } else { 4 };
        let sampler = StratifiedSampler::new(samples_per_pixel);

        Renderer::new(integrator, sampler, num_passes)
    };

    let t0_render = std::time::Instant::now();
    let result = {
        info!("rendering...");
        renderer.render_film(&scene)
    };
    let elapsed = t0_render.elapsed();

    info!("render finished in {:.3?}", elapsed);

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

    let denoised = {
        info!("denoising...");
        debug_time!("denoising");

        let albedo = {
            trace_time!("rendering albedo channel");

            let result = render_aux(&scene, Box::new(AlbedoIntegrator::new()));
            result
                .film
                .to_srgb_image()
                .save("./output/output-albedo.png")?;
            result.film
        };

        let normal = {
            trace_time!("rendering normal channel");

            let result = render_aux(&scene, Box::new(NormalIntegrator::new()));
            result
                .film
                .to_srgb_image()
                .save("./output/output-normal.png")?;

            // OIDN expects normals to be in range [-1, 1], but the integrator generates colors in
            // range [0, 1], so we scale the normals here.
            result.film.mapped(|s| s * 2.0 - 1.0)
        };

        unsafe {
            let denoiser = Denoiser::new(scene.camera.resolution, &albedo, &normal);
            denoiser.denoise(&result.film)
        }
    };

    denoised.to_srgb_image().save("./output/output.png")?;

    let elapsed = t0_main.elapsed();
    info!("finished in {:.3?}", elapsed);

    Ok(())
}

fn render_aux(scene: &Scene, integrator: Box<dyn Integrator>) -> RenderResult {
    let sampler = StratifiedSampler::new(4);

    let passes = num_cpus::get();
    let renderer = Renderer::new(integrator, sampler, passes);

    renderer.render_film(scene)
}
