use std::rc::Rc;

use glam::{vec3, Affine3A, Quat, Vec3};

use crate::flux::{
    shapes::{Quad, QuadBox, Sphere, Transform},
    textures::ConstantTexture,
    DielectricMaterial, DiffuseLightMaterial, MatteMaterial, Primitive,
};

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
    let glass_mat = {
        let tex = Rc::new(ConstantTexture::new(vec3(0.8, 0.8, 1.0)));
        Rc::new(DielectricMaterial::new(tex, 1.5))
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
    let glass_sphere = {
        let radius = box_size / 8.0;
        let shape = Box::new(Sphere::new(
            vec3(
                box_size / 6.0,
                -box_size / 2.0 + box_size / 3.0 + radius,
                -box_size / 6.0,
            ),
            radius,
        ));
        Primitive::new(shape, glass_mat.clone())
    };

    vec![
        left_wall,
        right_wall,
        floor,
        ceiling,
        back_wall,
        light,
        left_box,
        right_box,
        glass_sphere,
    ]
}
