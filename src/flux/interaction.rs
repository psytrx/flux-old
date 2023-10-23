use glam::Vec3;

pub struct Interaction {
    pub p: Vec3,
    pub n: Vec3,
    pub front_face: bool,
}
