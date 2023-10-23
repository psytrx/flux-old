use glam::{vec3, Vec3};

use super::{primitive::Primitive, ray::Ray};

pub struct Interaction<'a> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub front_face: bool,
    pub time: f32,
    pub primitive: &'a Primitive,
}

impl<'a> Interaction<'a> {
    pub fn spawn_ray(&self, direction: Vec3) -> Ray {
        let origin = offset_ray_origin(self.p, 8.0 * self.n);
        Ray::new(origin, direction, self.time)
    }
}

fn offset_ray_origin(p: Vec3, d: Vec3) -> Vec3 {
    const ORIGIN: f32 = 1.0 / 32.0;
    const FLOAT_SCALE: f32 = 1.0 / 65536.0;
    const INT_SCALE: f32 = 256.0;

    let of_i = (INT_SCALE * d).as_ivec3();

    let p_i = vec3(
        f32::from_bits(((p.x.to_bits() as i32) + if p.x < 0.0 { -of_i.x } else { of_i.x }) as u32),
        f32::from_bits(((p.y.to_bits() as i32) + if p.y < 0.0 { -of_i.y } else { of_i.y }) as u32),
        f32::from_bits(((p.z.to_bits() as i32) + if p.z < 0.0 { -of_i.z } else { of_i.z }) as u32),
    );

    let mut p_o = vec3(
        if p.x.abs() < ORIGIN {
            p.x + FLOAT_SCALE * d.x
        } else {
            p_i.x
        },
        if p.y.abs() < ORIGIN {
            p.y + FLOAT_SCALE * d.y
        } else {
            p_i.y
        },
        if p.z.abs() < ORIGIN {
            p.z + FLOAT_SCALE * d.z
        } else {
            p_i.z
        },
    );

    for i in 0..3 {
        if d[i] < 0.0 {
            p_o[i] = p_o[i].next_down()
        } else {
            p_o[i] = p_o[i].next_up()
        }
    }
    p_o
}
