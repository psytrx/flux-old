mod cornell_box;
mod defocus_blur;
mod many_spheres;
mod material_demo;

use std::rc::Rc;

use glam::{vec2, vec3, Affine3A, Quat, Vec2, Vec3};
use measure_time::debug_time;
use rand::{rngs::StdRng, Rng};

use crate::{
    example_scenes::cornell_box::cornell_box,
    flux::{
        lights::{Light, SkyLight},
        shapes::{Floor, Quad, QuadBox, Sphere, Transform},
        textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture, UvTexture},
        Bounds2, DielectricMaterial, DiffuseLightMaterial, MatteMaterial, MetalMaterial, Primitive,
        Scene,
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
    debug_time!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::ManySpheres => many_spheres(),
        ExampleScene::CornellBox => cornell_box(),
    }
}

pub fn default_sky_light() -> Box<dyn Light> {
    let horizon_color = vec3(0.5, 0.7, 1.0);
    let zenith_color = Vec3::ONE;
    let light = SkyLight::new(horizon_color, zenith_color);
    Box::new(light)
}

pub fn material_demo_primitives() -> Vec<Primitive> {
    let mat_floor = {
        let even = Rc::new(ConstantTexture::new(Vec3::splat(0.7)));
        let odd = Rc::new(ConstantTexture::new(Vec3::splat(0.5)));
        let tex = Rc::new(CheckerTexture::new(0.5, even, odd));
        Rc::new(MatteMaterial::new(tex))
    };
    let mat_earth = {
        let img = image::open("./assets/earthmap.jpg").unwrap();
        let tex = Rc::new(ImageTexture::new(img));
        Rc::new(MatteMaterial::new(tex))
    };
    let mat_left = {
        let tex = Rc::new(ConstantTexture::new(Vec3::ONE));
        Rc::new(DielectricMaterial::new(tex, 1.5))
    };
    let mat_center = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.2, 0.5, 0.1)));
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
    let mat_noise = {
        let tex = Rc::new(NoiseTexture::new(0.025));
        Rc::new(MatteMaterial::new(tex))
    };

    let floor = {
        let shape = Box::new(Floor::new());
        Primitive::new(shape, mat_floor.clone())
    };
    let earth = {
        let shape = Box::new(Sphere::new(vec3(0.0, 3.0, 4.0), 3.0));
        Primitive::new(shape, mat_earth.clone())
    };
    let left_sphere = {
        let shape = Box::new(Sphere::new(vec3(-2.5, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_left.clone())
    };
    let center_sphere = {
        let shape = Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_center.clone())
    };
    let right_sphere = {
        let shape = Box::new(Sphere::new(vec3(2.5, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat_right.clone())
    };
    let uv_sphere = {
        let shape = Box::new(Sphere::new(vec3(-2.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat_uv.clone())
    };
    let checkered_sphere = {
        let shape = Box::new(Sphere::new(vec3(0.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat_checkered.clone())
    };
    let noise_sphere = {
        let shape = Box::new(Sphere::new(vec3(2.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat_noise.clone())
    };

    vec![
        floor,
        earth,
        left_sphere,
        center_sphere,
        right_sphere,
        uv_sphere,
        checkered_sphere,
        noise_sphere,
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
    let light_mat = {
        let tex = Rc::new(ConstantTexture::new(25.0 * Vec3::ONE));
        Rc::new(DiffuseLightMaterial::new(tex))
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
    let light = {
        let size = box_size / 5.0 / 2.0;
        let y = box_size / 2.0 - 32.0 * f32::EPSILON;

        let lf = vec3(-size, y, -size);
        let rf = vec3(size, y, -size);
        let rb = vec3(size, y, size);
        let lb = vec3(-size, y, size);

        let shape = Box::new(Quad::new([lf, rf, rb, lb]));
        Primitive::new(shape, light_mat.clone())
    };
    let left_box = {
        let size = box_size / 3.0;
        let shape = Box::new(QuadBox::new(size, 2.0 * size, size));
        let transform = Affine3A::from_rotation_translation(
            Quat::from_rotation_y(-20_f32.to_radians()),
            vec3(-0.5 * size, -0.5 * size, 0.25 * size),
        );
        let shape = Box::new(Transform::new(transform, shape));
        Primitive::new(shape, white_mat.clone())
    };
    let right_box = {
        let size = box_size / 3.0;
        let shape = Box::new(QuadBox::new(size, size, size));
        let transform = Affine3A::from_rotation_translation(
            Quat::from_rotation_y(20_f32.to_radians()),
            vec3(0.5 * size, -1.0 * size, -0.5 * size),
        );
        let shape = Box::new(Transform::new(transform, shape));
        Primitive::new(shape, white_mat.clone())
    };

    vec![
        left_wall, right_wall, floor, ceiling, back_wall, light, left_box, right_box,
    ]
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
