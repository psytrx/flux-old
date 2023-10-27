#![feature(float_next_up_down)]

mod example_scenes;
mod flux;

use std::str::FromStr;

use crate::{
    example_scenes::{load_example_scene, ExampleScene},
    flux::{
        integrators::Integrator,
        integrators::{AlbedoIntegrator, NormalIntegrator, PathTracingIntegrator},
        Denoiser, RenderResult, Renderer, Scene, StratifiedSampler,
    },
};

use anyhow::Result;
use clap::Parser;
use log::{debug, info};
use measure_time::{debug_time, trace_time};
use num_format::{Locale, ToFormattedString};
use strum::ParseError;

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::init();

    let scene = load_scene(&args)?;

    let renderer = {
        let integrator = Box::new(PathTracingIntegrator::new(8, 32, 0.1));

        let samples_per_pixel = if args.dev { 1 } else { 4 };
        let sampler = StratifiedSampler::new(samples_per_pixel);

        let sweeps = if args.dev { 1 } else { 4 };
        let num_cpus = num_cpus::get();
        let num_passes = sweeps * num_cpus;

        Renderer::new(
            integrator,
            sampler,
            num_passes,
            Some(|evt| {
                debug!("pass {}\t({:>6.3}%)", evt.passes, evt.progress);
                evt.film
                    .to_srgb_image()
                    .save("./output/output.png")
                    .unwrap();
            }),
        )
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

            let result = render_aux_channel(&scene, Box::new(AlbedoIntegrator::new()), args.dev);
            result
                .film
                .to_srgb_image()
                .save("./output/output-albedo.png")?;
            result.film
        };

        let normal = {
            trace_time!("rendering normal channel");

            let result = render_aux_channel(&scene, Box::new(NormalIntegrator::new()), args.dev);
            result
                .film
                .to_srgb_image()
                .save("./output/output-normal.png")?;

            // OIDN expects normals to be in range [-1, 1], but the integrator generates colors in
            // range [0, 1], so we scale the normals here.
            result.film.mapped(|s| s * 2.0 - 1.0)
        };

        unsafe {
            let denoiser = Denoiser::new(scene.camera.resolution(), &albedo, &normal);
            trace_time!("denoise filter");
            denoiser.denoise(&result.film)
        }
    };

    denoised.to_srgb_image().save("./output/output.png")?;

    Ok(())
}

fn load_scene(args: &Args) -> Result<Scene> {
    info!("loading scene...");
    debug_time!("loading scene");

    let example_scene = ExampleScene::from_str(&args.scene)
        .map_err(|parse_err| FluxError::Scene(args.scene.clone(), parse_err))?;
    Ok(load_example_scene(example_scene))
}

fn render_aux_channel(
    scene: &Scene,
    integrator: Box<dyn Integrator>,
    dev_mode: bool,
) -> RenderResult {
    let samples_per_pixel = if dev_mode { 1 } else { 16 };
    let sampler = StratifiedSampler::new(samples_per_pixel);

    let passes = num_cpus::get();
    let renderer = Renderer::new(integrator, sampler, passes, None);

    renderer.render_film(scene)
}

#[derive(Debug, thiserror::Error)]
enum FluxError {
    #[error("Failed to parse scene '{0}': {1}")]
    Scene(String, ParseError),
}

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// The example scene to render
    #[arg(long, default_value = "cornellbox")]
    pub scene: String,

    /// Samples per pixel per pass
    #[arg(long = "spp", default_value = "16")]
    pub samples_per_pixel: usize,

    /// Runs quick/noisy renders for iterating quickly
    #[arg(long)]
    pub dev: bool,
}
