mod cornell_box;
mod defocus_blur;
mod many_spheres;
mod material_demo;
mod util;
mod wavefront_obj;

use glam::{vec2, vec3, Vec2, Vec3};
use measure_time::debug_time;
use rand::{rngs::StdRng, Rng};

use crate::{
    example_scenes::cornell_box::cornell_box,
    flux::{
        lights::{Light, SkyLight},
        Bounds2, Scene,
    },
};

use defocus_blur::defocus_blur;
use many_spheres::many_spheres;
use material_demo::material_demo;
use wavefront_obj::wavefront_obj;

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    DefocusBlur,
    ManySpheres,
    CornellBox,
    WavefrontObj,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug_time!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::ManySpheres => many_spheres(),
        ExampleScene::CornellBox => cornell_box(),
        ExampleScene::WavefrontObj => wavefront_obj(),
    }
}

pub fn default_sky_light() -> Box<dyn Light> {
    let horizon_color = vec3(0.5, 0.7, 1.0);
    let zenith_color = Vec3::ONE;
    let light = SkyLight::new(horizon_color, zenith_color);
    Box::new(light)
}

pub fn sample_disks(
    bounds: Bounds2<Vec2>,
    r: f32,
    n: usize,
    k: usize,
    rng: &mut StdRng,
) -> Vec<Vec2> {
    let mut samples: Vec<Vec2> = Vec::with_capacity(32);

    // generate an initial sample
    samples.push(vec2(
        rng.gen_range(bounds.min.x..bounds.max.x),
        rng.gen_range(bounds.min.y..bounds.max.y),
    ));

    let mut i = 0;
    while i < n {
        // generate k candidates
        let mut candidates: Vec<Vec2> = Vec::with_capacity(k);
        for _ in 0..k {
            let sample = vec2(
                rng.gen_range(bounds.min.x..bounds.max.x),
                rng.gen_range(bounds.min.y..bounds.max.y),
            );

            let collides = samples.iter().any(|other| other.distance(sample) < r);
            if !collides {
                candidates.push(sample);
            }
        }

        // find the candidate with the largest distance to any sample
        let best_candidate = candidates.iter().max_by(|candidate_a, candidate_b| {
            let distance_a = samples
                .iter()
                .map(|sample| sample.distance(**candidate_a))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            let distance_b = samples
                .iter()
                .map(|sample| sample.distance(**candidate_b))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            distance_a.partial_cmp(&distance_b).unwrap()
        });

        match best_candidate {
            None => i += 1,
            Some(candidate) => {
                i = 0;
                samples.push(*candidate);
            }
        }
    }

    samples
}
