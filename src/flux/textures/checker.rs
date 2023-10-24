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
        let x_int = (int.p.x / self.scale).floor() as i32;
        let y_int = (int.p.y / self.scale).floor() as i32;
        let z_int = (int.p.z / self.scale).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.evaluate(int)
        } else {
            self.odd.evaluate(int)
        }
    }
}
