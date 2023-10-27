use glam::Vec3;

pub struct Onb {
    u: Vec3,
    v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn from_w(w: Vec3) -> Self {
        // INFO: We assume w is normalized
        let u = if w.x.abs() > 0.9 { Vec3::Y } else { Vec3::X };
        let v = w.cross(u).normalize();
        let u = w.cross(v).normalize();
        Self { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
