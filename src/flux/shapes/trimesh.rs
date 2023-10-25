use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::{Vec2, Vec3};

use super::Shape;

pub struct TriangleMesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
}

impl TriangleMesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }
}

impl Shape for TriangleMesh {
    unsafe fn build_geometry(&self, _id: u32, device: RTCDevice) -> RTCGeometry {
        let geometry = rtcNewGeometry(device, RTCGeometryType::TRIANGLE);

        let vertex_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::VERTEX,
            0,
            RTCFormat::FLOAT3,
            3 * std::mem::size_of::<f32>(),
            self.vertices.len(),
        );
        let vertex_buf =
            std::slice::from_raw_parts_mut(vertex_buf_ptr as *mut f32, 3 * self.vertices.len());
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertex_buf[i * 3] = vertex.x;
            vertex_buf[i * 3 + 1] = vertex.y;
            vertex_buf[i * 3 + 2] = vertex.z;
        }

        let index_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::INDEX,
            0,
            RTCFormat::UINT3,
            3 * std::mem::size_of::<u32>(),
            self.indices.len(),
        );
        let index_buf =
            std::slice::from_raw_parts_mut(index_buf_ptr as *mut u32, self.indices.len());
        for (i, index) in self.indices.iter().enumerate() {
            index_buf[i] = *index as u32;
        }

        geometry
    }

    fn uv(&self, _p: Vec3) -> Vec2 {
        // TODO: implement UV coordinates for triangle meshes
        Vec2::ZERO
    }
}
