use embree4_sys::{RTCDevice, RTCGeometry};
use glam::Vec3;

use super::{Shape, TriangleMesh};

pub struct QuadBox {
    mesh: TriangleMesh,
}

impl QuadBox {
    pub fn new(p: Vec3, px: Vec3, py: Vec3, pz: Vec3) -> Self {
        let vertices = vec![
            p,
            p + px,
            p + px + py,
            p + py,
            p + pz,
            p + pz + px,
            p + pz + px + py,
            p + pz + py,
        ];
        let indices = vec![
            0, 2, 1, 0, 3, 2, 1, 2, 5, 5, 2, 6, 5, 6, 4, 6, 7, 4, 4, 7, 3, 4, 3, 0, 3, 7, 2, 2, 7,
            6, 0, 1, 4, 4, 1, 5,
        ];
        let mesh = TriangleMesh::new(vertices, indices);
        Self { mesh }
    }
}

impl Shape for QuadBox {
    unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry {
        self.mesh.build_geometry(device)
    }

    fn uv(&self, p: Vec3) -> glam::Vec2 {
        self.mesh.uv(p)
    }
}
