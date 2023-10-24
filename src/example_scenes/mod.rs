mod cornell_box;
mod defocus_blur;
mod many_spheres;
mod material_demo;

use std::rc::Rc;

use glam::{vec2, vec3, Vec2, Vec3};
use log::debug;
use rand::{rngs::StdRng, Rng};

use crate::{
    example_scenes::cornell_box::cornell_box,
    flux::{
        shapes::{Floor, Quad, Sphere},
        textures::{CheckerTexture, ConstantTexture, UvTexture},
        Bounds2, DielectricMaterial, MatteMaterial, MetalMaterial, Primitive, Scene,
    },
};

use defocus_blur::defocus_blur;
use many_spheres::many_spheres;
use material_demo::material_demo;

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    DefocusBlur,
    ManySpheres,
    CornellBox,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::ManySpheres => many_spheres(),
        ExampleScene::CornellBox => cornell_box(),
    }
}

pub fn material_demo_primitives() -> Vec<Primitive> {
    let mat_floor = {
        let even = Rc::new(ConstantTexture::new(vec3(0.8, 0.8, 0.0)));
        let odd = Rc::new(ConstantTexture::new(vec3(0.8, 0.0, 0.6)));
        let tex = Rc::new(CheckerTexture::new(0.5, even, odd));
        Rc::new(MatteMaterial::new(tex))
    };
    let mat_left = {
        let tex = Rc::new(ConstantTexture::new(Vec3::ONE));
        Rc::new(DielectricMaterial::new(tex, 1.5))
    };
    let mat_center = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.1, 0.2, 0.5)));
        Rc::new(MatteMaterial::new(tex))
    };
    let mat_right = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.8, 0.6, 0.2)));
        Rc::new(MetalMaterial::new(tex, 0.05))
    };
    let mat_uv = {
        let tex = Rc::new(UvTexture::new());
        Rc::new(MatteMaterial::new(tex))
    };
    let mat_checkered = {
        let even = Rc::new(ConstantTexture::new(Vec3::ZERO));
        let odd = Rc::new(ConstantTexture::new(Vec3::ONE));
        let tex = Rc::new(CheckerTexture::new(0.1, even, odd));
        Rc::new(MatteMaterial::new(tex))
    };

    let floor = {
        let shape = Box::new(Floor::new());
        Primitive::new(shape, mat_floor.clone())
    };
    let left_sphere = {
        let shape = Box::new(Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_left.clone())
    };
    let center_sphere = {
        let shape = Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_center.clone())
    };
    let right_sphere = {
        let shape = Box::new(Sphere::new(vec3(2.0, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_right.clone())
    };
    let uv_sphere = {
        let shape = Box::new(Sphere::new(vec3(0.5, 0.2, -1.5), 0.2));
        Primitive::new(shape, mat_uv.clone())
    };
    let checkered_sphere = {
        let shape = Box::new(Sphere::new(vec3(1.0, 0.2, -1.5), 0.2));
        Primitive::new(shape, mat_checkered.clone())
    };

    vec![
        floor,
        left_sphere,
        center_sphere,
        right_sphere,
        uv_sphere,
        checkered_sphere,
    ]
}

pub fn empty_cornell_box_primitives(box_size: f32) -> Vec<Primitive> {
    let white_mat = {
        let tex = Rc::new(ConstantTexture::new(Vec3::splat(0.73)));
        Rc::new(MatteMaterial::new(tex))
    };
    let green_mat = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.12, 0.45, 0.15)));
        Rc::new(MatteMaterial::new(tex))
    };
    let red_mat = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.65, 0.05, 0.05)));
        Rc::new(MatteMaterial::new(tex))
    };

    let ulf = vec3(-box_size, box_size, -box_size) / 2.0;
    let dlf = vec3(-box_size, -box_size, -box_size) / 2.0;
    let dlb = vec3(-box_size, -box_size, box_size) / 2.0;
    let ulb = vec3(-box_size, box_size, box_size) / 2.0;
    let urf = vec3(box_size, box_size, -box_size) / 2.0;
    let drf = vec3(box_size, -box_size, -box_size) / 2.0;
    let drb = vec3(box_size, -box_size, box_size) / 2.0;
    let urb = vec3(box_size, box_size, box_size) / 2.0;

    let left_wall = {
        let shape = Box::new(Quad::new([ulf, ulb, dlb, dlf]));
        Primitive::new(shape, green_mat.clone())
    };
    let right_wall = {
        let shape = Box::new(Quad::new([urf, drf, drb, urb]));
        Primitive::new(shape, red_mat.clone())
    };
    let floor = {
        let shape = Box::new(Quad::new([dlf, dlb, drb, drf]));
        Primitive::new(shape, white_mat.clone())
    };
    let ceiling = {
        let shape = Box::new(Quad::new([ulf, urf, urb, ulb]));
        Primitive::new(shape, white_mat.clone())
    };
    let back_wall = {
        let shape = Box::new(Quad::new([dlb, ulb, urb, drb]));
        Primitive::new(shape, white_mat.clone())
    };

    vec![left_wall, right_wall, floor, ceiling, back_wall]
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
