use embree4_sys::{
    rtcAttachGeometryByID, rtcCommitGeometry, rtcCommitScene, rtcNewGeometry, rtcNewScene,
    rtcReleaseGeometry, rtcSetGeometryInstancedScene, rtcSetGeometryTransform, RTCDevice,
    RTCGeometry, RTCGeometryType,
};
use glam::{Affine3A, Vec2, Vec3};
use log::trace;

use crate::flux::interaction::Interaction;

use super::Shape;

pub struct Transform {
    transform: Affine3A,
    shape: Box<dyn Shape>,
}

impl Transform {
    pub fn new(transform: Affine3A, shape: Box<dyn Shape>) -> Self {
        Self { transform, shape }
    }
}

impl Shape for Transform {
    unsafe fn build_geometry(&self, id: u32, device: RTCDevice) -> RTCGeometry {
        let shape_geom = self.shape.build_geometry(id, device);
        rtcCommitGeometry(shape_geom);

        let sub_scene = rtcNewScene(device);
        rtcAttachGeometryByID(sub_scene, shape_geom, id);
        rtcReleaseGeometry(shape_geom);
        rtcCommitScene(sub_scene);

        let instance = rtcNewGeometry(device, RTCGeometryType::INSTANCE);
        rtcSetGeometryInstancedScene(instance, sub_scene);

        let xfm = self.transform.to_cols_array();
        let xfm_ptr = xfm.as_ptr();
        rtcSetGeometryTransform(
            instance,
            0,
            embree4_sys::RTCFormat::FLOAT3X4_COLUMN_MAJOR,
            xfm_ptr as _,
        );

        instance
    }

    fn uv(&self, _p: Vec3) -> Vec2 {
        // TODO: implement UV coordinates for Transform geometry
        Vec2::ZERO
    }

    fn adjust_interaction(&self, int: &mut Interaction) {
        self.shape.adjust_interaction(int);
        int.n = self.transform.transform_vector3(int.n);
    }
}
