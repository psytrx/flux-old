use embree4_sys::{RTCDevice, RTCGeometry};
use glam::{vec3, Vec3};

use super::{Shape, TriangleMesh};

pub struct QuadBox {
    shape: TriangleMesh,
}

impl QuadBox {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        let p = vec3(-width, -height, -depth) / 2.0;
        let px = vec3(width, 0.0, 0.0);
        let py = vec3(0.0, height, 0.0);
        let pz = vec3(0.0, 0.0, depth);

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
            0, 2, 1, 0, 3, 2, // F
            1, 2, 5, 5, 2, 6, // R
            5, 6, 4, 6, 7, 4, // B
            4, 7, 3, 4, 0, 3, // L
            3, 7, 2, 2, 7, 6, // U
            0, 1, 4, 4, 1, 5, // D
        ];
        let shape = TriangleMesh::new(vertices, indices);
        Self { shape }
    }
}

impl Shape for QuadBox {
    unsafe fn build_geometry(&self, id: u32, device: RTCDevice) -> RTCGeometry {
        self.shape.build_geometry(id, device)
    }

    fn uv(&self, p: Vec3) -> glam::Vec2 {
        self.shape.uv(p)
    }
}
