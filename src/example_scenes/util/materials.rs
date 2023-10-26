use std::rc::Rc;

use glam::Vec3;

use crate::flux::{textures::ConstantTexture, DiffuseLightMaterial, Material, MatteMaterial};

pub fn build_matte_constant(albedo: Vec3) -> Rc<dyn Material> {
    let tex = Rc::new(ConstantTexture::new(albedo));
    Rc::new(MatteMaterial::new(tex))
}

pub fn build_diffuse_constant(emit: Vec3) -> Rc<dyn Material> {
    let tex = Rc::new(ConstantTexture::new(emit));
    Rc::new(DiffuseLightMaterial::new(tex))
}
