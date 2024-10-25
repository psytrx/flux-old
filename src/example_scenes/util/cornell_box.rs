use glam::{uvec2, vec3, Vec3};

use crate::{
    example_scenes::util::{build_diffuse_constant, build_matte_constant},
    flux::{shapes::Quad, Camera, PerspectiveCamera, Primitive},
};

pub fn cornell_box_camera(box_size: f32) -> Box<dyn Camera> {
    let resolution = uvec2(1024, 1024);

    let z_offset = 1.7 * box_size;
    let look_from = vec3(0.0, 0.0, -z_offset);
    let look_at = Vec3::ZERO;

    Box::new(PerspectiveCamera::new(
        resolution,
        look_from,
        look_at,
        45.0,
        0.3,
        look_at.distance(look_from),
    ))
}

pub fn empty_cornell_box_prims(box_size: f32) -> Vec<Primitive> {
    let white_mat = build_matte_constant(Vec3::splat(0.73));

    let ulf = vec3(-box_size, box_size, -box_size) / 2.0;
    let dlf = vec3(-box_size, -box_size, -box_size) / 2.0;
    let dlb = vec3(-box_size, -box_size, box_size) / 2.0;
    let ulb = vec3(-box_size, box_size, box_size) / 2.0;
    let urf = vec3(box_size, box_size, -box_size) / 2.0;
    let drf = vec3(box_size, -box_size, -box_size) / 2.0;
    let drb = vec3(box_size, -box_size, box_size) / 2.0;
    let urb = vec3(box_size, box_size, box_size) / 2.0;

    let left_wall = {
        let green_mat = build_matte_constant(vec3(0.12, 0.45, 0.15));
        let shape = Box::new(Quad::new([ulf, ulb, dlb, dlf]));
        Primitive::new(shape, green_mat)
    };

    let right_wall = {
        let red_mat = build_matte_constant(vec3(0.65, 0.05, 0.05));
        let shape = Box::new(Quad::new([urf, drf, drb, urb]));
        Primitive::new(shape, red_mat)
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
        let light_mat = build_diffuse_constant(Vec3::splat(15.0));

        let size = 0.1 * box_size;
        let y = box_size / 2.0 - 32.0 * f32::EPSILON;

        let lf = vec3(-size, y, -size);
        let rf = vec3(size, y, -size);
        let rb = vec3(size, y, size);
        let lb = vec3(-size, y, size);

        let shape = Box::new(Quad::new([lf, rf, rb, lb]));
        Primitive::new(shape, light_mat)
    };

    vec![left_wall, right_wall, floor, ceiling, back_wall, light]
}
