use std::rc::Rc;

use crate::flux::interaction::Interaction;

use super::Texture;

pub struct CheckerTexture<T> {
    scale: f32,
    even: Rc<dyn Texture<T>>,
    odd: Rc<dyn Texture<T>>,
}

impl<T: Copy> CheckerTexture<T> {
    pub fn new(scale: f32, even: Rc<dyn Texture<T>>, odd: Rc<dyn Texture<T>>) -> Self {
        Self { scale, even, odd }
    }
}

impl<T: Copy> Texture<T> for CheckerTexture<T> {
    fn evaluate(&self, int: &Interaction) -> T {
        let uv = int.primitive.shape.uv(int.p);

        let u_int = (uv.x / self.scale).floor() as i32;
        let v_int = (uv.y / self.scale).floor() as i32;

        let is_even = (u_int + v_int) % 2 == 0;
        if is_even {
            self.even.evaluate(int)
        } else {
            self.odd.evaluate(int)
        }
    }
}
