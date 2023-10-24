#![feature(float_next_up_down)]

mod example_scenes;
mod flux;

use anyhow::Result;
use log::{debug, info};

use num_format::{Locale, ToFormattedString};

use crate::{
    example_scenes::{load_example_scene, ExampleScene},
    flux::{Denoiser, Renderer},
};

fn main() -> Result<()> {
    env_logger::init();

    let sweeps = 1;
    let num_cpus = num_cpus::get();
    let num_passes = sweeps * num_cpus;

    let scene = {
        info!("Loading scene...");
        load_example_scene(ExampleScene::ManySpheres)
    };

    let renderer = Renderer::new(4, 4, 16, 0.1, num_passes);

    let t0 = std::time::Instant::now();
    let result = {
        info!("rendering...");
        renderer.render_film(&scene)
    };
    let elapsed = t0.elapsed();

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

    unsafe {
        let denoiser = Denoiser::new(scene.camera.resolution);
        let denoised = denoiser.denoise(&result.film);
        denoised.to_srgb_image().save("./output/output.png")?;
    }

    Ok(())
}
