mod checker;
mod constant;

pub use checker::CheckerTexture;
pub use constant::ConstantTexture;

use super::interaction::Interaction;

pub trait Texture<T> {
    fn evaluate(&self, int: &Interaction) -> T;
}
