mod dielectric;
mod diffuse_light;
mod matte;
mod metal;

pub use dielectric::*;
pub use diffuse_light::*;
pub use matte::*;
pub use metal::*;

use glam::Vec3;
use rand::rngs::StdRng;

use super::{interaction::Interaction, ray::Ray};

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub scattered: Ray,
    pub pdf: f32,
}

#[derive(PartialEq)]
pub enum BxdfType {
    Diffuse,
    DiffuseLight,
    Specular,
    Other,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec>;

    fn emitted(&self, _ray: &Ray, _int: &Interaction) -> Vec3 {
        Vec3::ZERO
    }

    fn bxdf_type(&self) -> BxdfType;

    fn scattering_pdf(&self, _ray: &Ray, _int: &Interaction, _scattered: &Ray) -> f32 {
        0.0
    }
}

#[allow(dead_code)]
pub fn is_near_zero(v: Vec3) -> bool {
    v.x.abs() <= f32::EPSILON && v.y.abs() <= f32::EPSILON && v.z.abs() <= f32::EPSILON
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
