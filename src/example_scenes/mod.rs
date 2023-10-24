mod defocus_blur;
mod many_spheres;
mod material_demo;

use std::rc::Rc;

use glam::{vec2, vec3, Vec2};
use log::debug;
use rand::{rngs::StdRng, Rng};

use crate::flux::{
    Bounds2, DielectricMaterial, MatteMaterial, MetalMaterial, Primitive, Scene, Sphere,
};

use defocus_blur::defocus_blur;
use many_spheres::many_spheres;
use material_demo::material_demo;

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    DefocusBlur,
    ManySpheres,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::ManySpheres => many_spheres(),
    }
}

pub fn material_demo_primitives() -> Vec<Primitive> {
    let mat_floor = Rc::new(MatteMaterial::new(vec3(0.8, 0.8, 0.0)));
    let mat_left = Rc::new(DielectricMaterial::new(1.5));
    let mat_center = Rc::new(MatteMaterial::new(vec3(0.1, 0.2, 0.5)));
    let mat_right = Rc::new(MetalMaterial::new(vec3(0.8, 0.6, 0.2), 0.05));

    let floor = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, mat_floor.clone())
    };

    let left_sphere = {
        let shape = Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_left.clone())
    };

    let center_sphere = {
        let shape = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_center.clone())
    };

    let right_sphere = {
        let shape = Sphere::new(vec3(2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_right.clone())
    };

    vec![floor, left_sphere, center_sphere, right_sphere]
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
