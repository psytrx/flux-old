use std::{ops::Mul, rc::Rc};

use crate::flux::interaction::Interaction;

use super::Texture;

pub struct MultiplyTexture<T> {
    scale: f32,
    tex: Rc<dyn Texture<T>>,
}

impl<T: Copy> MultiplyTexture<T> {
    pub fn new(scale: f32, tex: Rc<dyn Texture<T>>) -> Self {
        Self { scale, tex }
    }
}

impl<T: Copy + Mul<f32, Output = T>> Texture<T> for MultiplyTexture<T> {
    fn evaluate(&self, _int: &Interaction) -> T {
        self.tex.evaluate(_int) * self.scale
    }
}
