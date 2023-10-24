use std::ptr::null_mut;

use embree4_sys::{
    rtcNewGeometry, rtcSetGeometryBoundsFunction, rtcSetGeometryIntersectFunction,
    rtcSetGeometryUserPrimitiveCount, RTCBounds, RTCBoundsFunctionArguments, RTCDevice,
    RTCGeometry, RTCGeometryType, RTCIntersectFunctionNArguments, RTCRayHit,
};
use glam::{vec2, Vec2, Vec3};

use super::Shape;

pub struct Floor;

impl Floor {
    pub fn new() -> Self {
        Floor
    }
}

impl Shape for Floor {
    unsafe fn build_geometry(&self, device: RTCDevice) -> RTCGeometry {
        let geometry = rtcNewGeometry(device, RTCGeometryType::USER);

        rtcSetGeometryUserPrimitiveCount(geometry, 1);
        rtcSetGeometryBoundsFunction(geometry, Some(bounds_fn), null_mut());
        rtcSetGeometryIntersectFunction(geometry, Some(intersect_fn));

        geometry
    }

    fn uv(&self, p: Vec3) -> Vec2 {
        vec2(p.x, p.z)
    }
}

unsafe extern "C" fn bounds_fn(args: *const RTCBoundsFunctionArguments) {
    let args = *args;
    *args.bounds_o = RTCBounds {
        lower_x: -999_999.0,
        lower_y: -0.1,
        lower_z: -999_999.0,
        align0: Default::default(),
        upper_x: 999_999.0,
        upper_y: 0.1,
        upper_z: 999_999.0,
        align1: Default::default(),
    }
}

unsafe extern "C" fn intersect_fn(args: *const RTCIntersectFunctionNArguments) {
    let args = *args;
    assert_eq!(1, args.N);

    let valid = *args.valid;
    if valid == 0 {
        return;
    }

    let ray_hit_ptr = args.rayhit as *mut RTCRayHit;
    let ray_hit = &mut *ray_hit_ptr;

    let t = -ray_hit.ray.org_y / ray_hit.ray.dir_y;
    if t < ray_hit.ray.tnear || t > ray_hit.ray.tfar {
        return;
    }

    if ray_hit.ray.org_y > 0.0 {
        ray_hit.hit.Ng_y = 1.0;
    } else {
        ray_hit.hit.Ng_y = -1.0;
    }

    ray_hit.ray.tfar = t;
    ray_hit.hit.primID = args.primID;
    ray_hit.hit.geomID = args.geomID;
}
