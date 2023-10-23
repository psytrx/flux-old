use super::{camera::Camera, list::ShapeList};

pub struct Scene {
    pub camera: Camera,
    pub aggregate: ShapeList,
}
