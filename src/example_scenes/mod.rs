mod cornell_box;
mod defocus_blur;
mod dragon;
mod many_spheres;
mod material_demo;
mod suzanne;
mod util;

use glam::{vec2, vec3, Vec2, Vec3};
use rand::{rngs::StdRng, Rng};
use strum::EnumString;

use crate::{
    example_scenes::cornell_box::cornell_box,
    flux::{
        lights::{Light, SkyLight},
        Bounds2, Scene,
    },
};

use defocus_blur::defocus_blur;
use dragon::dragon;
use many_spheres::many_spheres;
use material_demo::material_demo;
use suzanne::suzanne;

use self::cornell_box::simple_cornell_box;

#[allow(dead_code)]
#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ExampleScene {
    CornellBox,
    DefocusBlur,
    Dragon,
    ManySpheres,
    MaterialDemo,
    SimpleCornellBox,
    Suzanne,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    match scene {
        ExampleScene::CornellBox => cornell_box(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::Dragon => dragon(),
        ExampleScene::ManySpheres => many_spheres(),
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::SimpleCornellBox => simple_cornell_box(),
        ExampleScene::Suzanne => suzanne(),
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
