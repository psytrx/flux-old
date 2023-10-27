use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::{vec2, Vec2, Vec3};
use rand::{rngs::StdRng, Rng};

use super::Shape;

pub struct Quad {
    vertices: [Vec3; 4],
    p: Vec3,
    u: Vec3,
    v: Vec3,
    area: f32,
}

impl Quad {
    pub fn new(vertices: [Vec3; 4]) -> Self {
        let p = vertices[0];
        let u = vertices[1] - vertices[0];
        let v = vertices[3] - vertices[0];
        let area = u.cross(v).length();
        Self {
            vertices,
            p,
            u,
            v,
            area,
        }
    }
}

impl Shape for Quad {
    unsafe fn build_geometry(&self, _id: u32, device: RTCDevice) -> RTCGeometry {
        let geometry = rtcNewGeometry(device, RTCGeometryType::QUAD);

        let vertex_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::VERTEX,
            0,
            RTCFormat::FLOAT3,
            3 * std::mem::size_of::<f32>(),
            4,
        );
        let vertex_buf = std::slice::from_raw_parts_mut(vertex_buf_ptr as *mut f32, 3 * 4);
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertex_buf[i * 3] = vertex.x;
            vertex_buf[i * 3 + 1] = vertex.y;
            vertex_buf[i * 3 + 2] = vertex.z;
        }

        let index_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::INDEX,
            0,
            RTCFormat::UINT4,
            4 * std::mem::size_of::<u32>(),
            1,
        );
        let index_buf = std::slice::from_raw_parts_mut(index_buf_ptr as *mut u32, 4);
        index_buf.copy_from_slice(&[0, 1, 2, 3]);

        geometry
    }

    fn uv(&self, p: Vec3) -> Vec2 {
        let u_vec = self.vertices[1] - self.vertices[0];
        let v_vec = self.vertices[3] - self.vertices[0];

        let pc = p - self.vertices[0];
        let u = pc.dot(u_vec) / u_vec.length_squared();
        let v = pc.dot(v_vec) / v_vec.length_squared();

        vec2(u, v)
    }

    fn sample_point(&self, _origin: Vec3, rng: &mut StdRng) -> Vec3 {
        let s: f32 = rng.gen();
        let t = rng.gen::<f32>();
        self.p + s * self.u + t * self.v
    }

    fn area(&self) -> f32 {
        self.area
    }
}
