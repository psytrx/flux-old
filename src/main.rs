#![feature(float_next_up_down)]

mod example_scenes;
mod flux;

use std::{
    path::Path,
    str::FromStr,
    time::{Duration, Instant},
};

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
use flux::DefaultRenderUpdater;
use log::{debug, info};
use measure_time::{debug_time, trace_time};
use num_format::{Locale, ToFormattedString};
use strum::ParseError;

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    info!("{:?}", args);

    let scene = load_scene(&args)?;
    let renderer = setup_renderer(&args);

    let result = {
        info!("rendering...");

        let t_render_film = Instant::now();
        let result = renderer.render_film(&scene);
        let elapsed = t_render_film.elapsed();

        print_stats(RenderStats {
            total_rays: result.rays,
            elapsed,
        });

        result
    };

    let output_dir = Path::new(&args.out_dir);

    let beauty_path = output_dir.join("output.png");
    result.film.to_srgb_image().save(&beauty_path)?;

    let beauty_raw_path = output_dir.join("output-raw.png");
    std::fs::copy(&beauty_path, beauty_raw_path)?;

    let denoised = {
        info!("denoising...");

        let denoiser = setup_denoiser(&scene, &args)?;

        trace_time!("denoise filter");
        unsafe { denoiser.denoise(&result.film) }
    };

    denoised.to_srgb_image().save(&beauty_path)?;

    info!("done");
    Ok(())
}

fn print_stats(stats: RenderStats) {
    info!("render finished in {:.3?}", stats.elapsed);

    debug!(
        "rays:     {:>16}",
        stats.total_rays.to_formatted_string(&Locale::en)
    );

    let rays_per_sec = (stats.total_rays as f64 / stats.elapsed.as_secs_f64()) as usize;
    debug!(
        "rays/sec: {:>16}",
        rays_per_sec.to_formatted_string(&Locale::en)
    );
}

fn setup_renderer(args: &Args) -> Renderer {
    let integrator = Box::new(PathTracingIntegrator::new(
        args.min_depth,
        args.max_depth,
        args.rr_stop_prob,
    ));

    let sampler = StratifiedSampler::new(args.spp);

    let num_passes = {
        let num_cpus = num_cpus::get();
        args.sweeps * num_cpus
    };

    let updater = {
        let filepath = Path::new(&args.out_dir).join("output.png");
        Box::new(DefaultRenderUpdater::new(
            Duration::from_secs(args.update_interval),
            filepath,
        ))
    };

    Renderer::new(integrator, sampler, num_passes, Some(updater))
}

fn setup_denoiser(scene: &Scene, args: &Args) -> Result<Denoiser> {
    debug!("initializing denoiser");

    let output_dir = Path::new(&args.out_dir);

    let albedo_path = output_dir.join("output-albedo.png");
    let albedo = {
        trace_time!("rendering albedo channel");

        let result = render_aux_channel(scene, Box::new(AlbedoIntegrator::new()), args);
        result.film.to_srgb_image().save(&albedo_path)?;
        result.film
    };

    let normal_path = output_dir.join("output-normal.png");
    let normal = {
        trace_time!("rendering normal channel");

        let result = render_aux_channel(scene, Box::new(NormalIntegrator::new()), args);
        result.film.to_srgb_image().save(&normal_path)?;

        // OIDN expects normals to be in range [-1, 1], but the integrator generates colors in
        // range [0, 1], so we scale the normals here.
        result.film.mapped(|s| s * 2.0 - 1.0)
    };

    let denoiser = unsafe {
        trace_time!("initializing denoiser");
        Denoiser::new(scene.camera.resolution(), &albedo, &normal)
    };

    let albedo_raw_path = output_dir.join("output-albedo-raw.png");
    std::fs::copy(&albedo_path, albedo_raw_path)?;
    denoiser
        .albedo_denoised
        .to_srgb_image()
        .save(&albedo_path)?;

    // again, we need to map the normals back to our domain of [0, 1]
    let normal_raw_path = output_dir.join("output-normal-raw.png");
    std::fs::copy(&normal_path, normal_raw_path)?;
    denoiser
        .normal_denoised
        .mapped(|s| (s + 1.0) / 2.0)
        .to_srgb_image()
        .save(&normal_path)?;

    Ok(denoiser)
}

fn load_scene(args: &Args) -> Result<Scene> {
    info!("loading scene...");
    debug_time!("loading scene");

    let example_scene = ExampleScene::from_str(&args.scene)
        .map_err(|parse_err| FluxError::Scene(args.scene.clone(), parse_err))?;
    Ok(load_example_scene(example_scene))
}

fn render_aux_channel(scene: &Scene, integrator: Box<dyn Integrator>, args: &Args) -> RenderResult {
    let sampler = StratifiedSampler::new(args.aux_spp);

    let passes = args.aux_sweeps * num_cpus::get();
    let renderer = Renderer::new(integrator, sampler, passes, None);

    renderer.render_film(scene)
}

#[derive(Debug, thiserror::Error)]
enum FluxError {
    #[error("Failed to parse scene '{0}': {1}")]
    Scene(String, ParseError),
}

struct RenderStats {
    total_rays: usize,
    elapsed: Duration,
}

#[derive(Clone, Debug, Parser)]
#[command(version)]
pub struct Args {
    /// The example scene to render
    #[arg(long = "scene", short = 's', default_value = "cornellbox")]
    scene: String,

    ///Number of full CPU sweeps to run
    #[arg(long = "sweeps", default_value = "8")]
    sweeps: usize,

    /// Samples/pixel/pass
    #[arg(long = "spp", default_value = "4")]
    spp: usize,

    /// Minimum tracing path depth
    #[arg(long = "min-depth", default_value = "8")]
    min_depth: u32,

    /// Maximum tracing path depth
    #[arg(long = "max-depth", default_value = "32")]
    max_depth: u32,

    /// Russian roulette path termination probability
    #[arg(long = "rr-stop-prob", default_value = "0.1")]
    rr_stop_prob: f32,

    ///Number of full CPU sweeps for auxiliary channels
    #[arg(long = "aux-sweeps", default_value = "1")]
    aux_sweeps: usize,

    /// Samples/pixel/pass for auxiliary channels
    #[arg(long = "aux-spp", default_value = "4")]
    aux_spp: usize,

    /// Output directory for rendered images
    #[arg(long = "out-dir", short = 'o', default_value = "./output")]
    out_dir: String,

    /// Update interval for intermediate render results
    #[arg(long = "update-interval", short = 'u', default_value = "1")]
    update_interval: u64,
}
