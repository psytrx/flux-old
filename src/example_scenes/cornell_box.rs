use std::rc::Rc;

use glam::{vec3, Affine3A, Quat, Vec3};

use crate::flux::{
    shapes::{QuadBox, Sphere, Transform},
    textures::ConstantTexture,
    DielectricMaterial, Primitive, Scene,
};

use super::util::{build_matte_constant, cornell_box_aggregate, cornell_box_camera};

pub fn cornell_box() -> Scene {
    let box_size = 100.0;
    let camera = cornell_box_camera(box_size);

    let aggregate = build_aggregate(box_size);
    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}

fn build_aggregate(box_size: f32) -> Vec<Primitive> {
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

    let mut aggregate = cornell_box_aggregate(box_size);
    aggregate.extend([left_box, right_box, glass_sphere]);
    aggregate
}
