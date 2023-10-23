use embree4_sys::{
    rtcAttachGeometryByID, rtcCommitGeometry, rtcCommitScene, rtcNewDevice, rtcNewScene,
    rtcReleaseGeometry, rtcSetSceneBuildQuality, rtcSetSceneFlags, RTCBuildQuality, RTCScene,
    RTCSceneFlags,
};
use measure_time::trace_time;

use super::primitive::Primitive;

pub struct EmbreeAccel {
    pub scene: RTCScene,
}

impl EmbreeAccel {
    pub unsafe fn build(primitives: &[Primitive]) -> EmbreeAccel {
        trace_time!("build");

        let device = rtcNewDevice(b"verbose=0" as *const _ as _);

        let scene = rtcNewScene(device);
        rtcSetSceneBuildQuality(scene, RTCBuildQuality::HIGH);
        rtcSetSceneFlags(scene, RTCSceneFlags::ROBUST);

        for (id, prim) in primitives.iter().enumerate() {
            let geometry_id = (id + 1) as u32;
            let geometry = prim.build_geometry(device);

            rtcCommitGeometry(geometry);
            rtcAttachGeometryByID(scene, geometry, geometry_id);
            rtcReleaseGeometry(geometry);
        }

        rtcCommitScene(scene);

        Self { scene }
    }
}
