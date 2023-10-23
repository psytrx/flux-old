use std::ops::Range;

use super::{interaction::Interaction, ray::Ray, sphere::Sphere};

pub struct ShapeList {
    shapes: Vec<Sphere>,
}

impl ShapeList {
    pub fn new(shapes: Vec<Sphere>) -> Self {
        Self { shapes }
    }

    pub fn intersect(&self, ray: &Ray, t: Range<f32>) -> Option<Interaction> {
        self.shapes.iter().fold(None, |acc, shape| {
            let t_max = acc.as_ref().map(|int| int.t).unwrap_or(t.end);
            shape.intersect(ray, t.start..t_max).or(acc)
        })
    }
}
