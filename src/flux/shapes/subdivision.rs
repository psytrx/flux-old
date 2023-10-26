use embree4_sys::{
    rtcNewGeometry, rtcSetNewGeometryBuffer, RTCBufferType, RTCDevice, RTCFormat, RTCGeometry,
    RTCGeometryType,
};
use glam::{Vec2, Vec3};

use super::Shape;

pub struct SubdivisionMesh {
    tesselation: f32,
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
}

impl SubdivisionMesh {
    pub fn new(tesselation: f32, vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self {
            tesselation,
            vertices,
            indices,
        }
    }
}

impl Shape for SubdivisionMesh {
    unsafe fn build_geometry(&self, _id: u32, device: RTCDevice) -> RTCGeometry {
        let geometry = rtcNewGeometry(device, RTCGeometryType::SUBDIVISION);

        let vertex_count = self.vertices.len();
        let vertex_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::VERTEX,
            0,
            RTCFormat::FLOAT3,
            3 * std::mem::size_of::<f32>(),
            vertex_count,
        );
        let vertex_buf =
            std::slice::from_raw_parts_mut(vertex_buf_ptr as *mut f32, 3 * vertex_count);
        for (i, vertex) in self.vertices.iter().enumerate() {
            vertex_buf[i * 3] = vertex.x;
            vertex_buf[i * 3 + 1] = vertex.y;
            vertex_buf[i * 3 + 2] = vertex.z;
        }

        let index_count = self.indices.len();
        let index_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::INDEX,
            0,
            RTCFormat::UINT,
            std::mem::size_of::<u32>(),
            index_count,
        );
        let index_buf = std::slice::from_raw_parts_mut(index_buf_ptr as *mut u32, index_count);
        for (i, index) in self.indices.iter().enumerate() {
            index_buf[i] = *index as u32;
        }

        let face_count = self.indices.len() / 3;
        let face_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::FACE,
            0,
            RTCFormat::UINT,
            std::mem::size_of::<u32>(),
            face_count,
        );
        let face_buf = std::slice::from_raw_parts_mut(face_buf_ptr as *mut u32, face_count);
        for face in face_buf.iter_mut() {
            *face = 3;
        }

        let level_buf_ptr = rtcSetNewGeometryBuffer(
            geometry,
            RTCBufferType::LEVEL,
            0,
            RTCFormat::FLOAT,
            std::mem::size_of::<f32>(),
            index_count,
        );
        let level_buf = std::slice::from_raw_parts_mut(level_buf_ptr as *mut f32, index_count);
        for level in level_buf.iter_mut() {
            *level = self.tesselation;
        }

        geometry
    }

    fn uv(&self, _p: Vec3) -> Vec2 {
        // TODO: implement UV coordinates for triangle meshes
        Vec2::ZERO
    }
}
