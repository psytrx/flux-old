use super::{camera::Camera, list::ShapeList, Sphere};

pub struct Scene {
    pub camera: Camera,
    pub aggregate: ShapeList,
}

impl Scene {
    pub fn new(camera: Camera, aggregate: Vec<Sphere>) -> Self {
        let accel = ShapeList::new(aggregate);
        Self {
            camera,
            aggregate: accel,
        }
    }
}
