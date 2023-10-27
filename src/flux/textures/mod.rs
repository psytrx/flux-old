mod checker;
mod constant;
mod image;
mod multiply;
mod noise;
mod uv;

pub use checker::*;
pub use constant::*;
pub use image::*;
pub use multiply::*;
pub use noise::*;
pub use uv::*;

use super::interaction::Interaction;

pub trait Texture<T> {
    fn evaluate(&self, int: &Interaction) -> T;
}
