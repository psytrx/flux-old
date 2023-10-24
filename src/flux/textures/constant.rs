use crate::flux::interaction::Interaction;

use super::Texture;

pub struct ConstantTexture<T> {
    value: T,
}

impl<T: Copy> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Copy> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _int: &Interaction) -> T {
        self.value
    }
}
