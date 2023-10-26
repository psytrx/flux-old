mod cornell_box;
mod material_demo;
mod obj;
mod ply;

use std::{path::Path, rc::Rc};

pub use cornell_box::*;
use glam::Vec3;
pub use material_demo::*;
pub use obj::load_obj;
pub use ply::load_ply;

use crate::flux::{
    shapes::Sphere,
    textures::{ImageTexture, MultiplyTexture},
    DiffuseLightMaterial, Primitive,
};

pub fn hdr_light_dome<P: AsRef<Path>>(filename: P) -> Primitive {
    let mat = {
        let img = image::open(filename).unwrap();
        let tex = Rc::new(ImageTexture::new(img));
        let tex = Rc::new(MultiplyTexture::new(2.0, tex));
        Rc::new(DiffuseLightMaterial::new(tex))
    };
    let shape = Box::new(Sphere::new(Vec3::ZERO, 1_000.0));
    Primitive::new(shape, mat)
}

pub struct ModelResult {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
}
