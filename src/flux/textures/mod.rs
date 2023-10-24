mod checker;
mod constant;
mod image;
mod uv;

pub use checker::CheckerTexture;
pub use constant::ConstantTexture;
pub use image::ImageTexture;
pub use uv::UvTexture;

use super::interaction::Interaction;

pub trait Texture<T> {
    fn evaluate(&self, int: &Interaction) -> T;
}