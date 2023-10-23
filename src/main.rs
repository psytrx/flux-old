#![feature(float_next_up_down)]

mod flux;

use anyhow::Result;
use flux::{Camera, Scene, Sphere};
use glam::{uvec2, vec3};
use measure_time::trace_time;

use crate::flux::render_image;

fn main() -> Result<()> {
    env_logger::init();
    trace_time!("main");

    let scene = load_scene();
    let img = render_image(&scene);
    img.save("./output/output.png")?;

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
