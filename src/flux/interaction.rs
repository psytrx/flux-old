use glam::Vec3;

pub struct Interaction {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub front_face: bool,
}
