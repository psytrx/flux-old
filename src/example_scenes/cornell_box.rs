use std::rc::Rc;

use glam::{vec3, Affine3A, Quat, Vec3};

use crate::flux::{
    shapes::{QuadBox, Sphere, SubdivisionMesh, Transform},
    textures::ConstantTexture,
    DielectricMaterial, DiffuseLightMaterial, MetalMaterial, Primitive, Scene,
};

use super::util::{build_matte_constant, cornell_box_camera, empty_cornell_box_prims, load_ply};

pub fn cornell_box() -> Scene {
    let box_size = 100.0;
    let camera = cornell_box_camera(box_size);

    let mut aggregate = empty_cornell_box_prims(box_size);
    aggregate.append(&mut build_box_prims(box_size));
    aggregate.append(&mut build_extra_prims(box_size));

    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}

pub fn simple_cornell_box() -> Scene {
    let box_size = 100.0;
    let camera = cornell_box_camera(box_size);

    let mut aggregate = empty_cornell_box_prims(box_size);
    aggregate.append(&mut build_box_prims(box_size));

    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}

fn build_box_prims(box_size: f32) -> Vec<Primitive> {
    let white_mat = build_matte_constant(Vec3::splat(0.73));

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

    vec![left_box, right_box]
}

fn build_extra_prims(box_size: f32) -> Vec<Primitive> {
    let glass_sphere = {
        let glass_mat = {
            let tex = Rc::new(ConstantTexture::new(vec3(0.8, 0.8, 1.0)));
            Rc::new(DielectricMaterial::new(tex, 1.5))
        };
        let radius = box_size / 10.0;
        let shape = Box::new(Sphere::new(
            vec3(
                box_size / 6.0,
                -box_size / 2.0 + box_size / 3.0 + radius,
                -box_size / 6.0,
            ),
            radius,
        ));
        Primitive::new(shape, glass_mat)
    };

    let metal_spheres = {
        (0..3).map(|i| {
            let radius = box_size / 20.0;
            let fuzz = 0.25 * (i + 1) as f32;
            let mat = {
                let tex = Rc::new(ConstantTexture::new(vec3(
                    if i % 3 == 0 { 0.8 } else { 0.1 },
                    if i % 3 == 1 { 0.8 } else { 0.1 },
                    if i % 3 == 2 { 0.8 } else { 0.1 },
                )));
                Rc::new(MetalMaterial::new(tex, fuzz))
            };
            let shape = Box::new(Sphere::new(
                vec3(
                    -box_size / 2.5 + (i as f32) * 2.5 * radius,
                    -box_size / 2.0 + radius,
                    -box_size / 2.5 + (i as f32) * box_size / 15.0,
                ),
                radius,
            ));
            Primitive::new(shape, mat)
        })
    };

    let glow_sphere = {
        let glass_mat = {
            let tex = Rc::new(ConstantTexture::new(vec3(2.0, 2.0, 0.0)));
            Rc::new(DiffuseLightMaterial::new(tex))
        };
        let radius = box_size / 20.0;
        let shape = Box::new(Sphere::new(
            vec3(box_size / 2.7, -box_size / 2.0 + radius, -box_size / 2.7),
            radius,
        ));
        Primitive::new(shape, glass_mat)
    };

    let ruby_dragon = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(vec3(1.0, 0.2, 0.4)));
            Rc::new(DielectricMaterial::new(tex, 1.77))
        };

        let result = load_ply("./assets/dragon/dragon_vrip_res2.ply").unwrap();
        let shape = Box::new(SubdivisionMesh::new(2.0, result.vertices, result.indices));
        let transform = Affine3A::from_scale_rotation_translation(
            Vec3::splat(200.0),
            Quat::from_rotation_y((-55_f32).to_radians()),
            vec3(-0.15 * box_size, 0.065 * box_size, 0.115 * box_size),
        );
        let shape = Box::new(Transform::new(transform, shape));
        Primitive::new(shape, mat)
    };

    let mut aggregate = empty_cornell_box_prims(box_size);
    aggregate.extend([glass_sphere, glow_sphere, ruby_dragon]);
    aggregate.extend(metal_spheres);
    aggregate
}
