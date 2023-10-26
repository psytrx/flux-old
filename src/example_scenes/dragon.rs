use std::rc::Rc;

use glam::{uvec2, vec3, Affine3A, Quat, Vec3};

use crate::{
    example_scenes::util::{build_matte_constant, hdr_light_dome, load_ply},
    flux::{
        shapes::{Floor, Sphere, Transform, TriangleMesh},
        textures::ConstantTexture,
        DielectricMaterial, DiffuseLightMaterial, PerspectiveCamera, Primitive, Scene,
    },
};

pub fn dragon() -> Scene {
    let camera = {
        let resolution = uvec2(1024, 1024);
        let look_from = vec3(-3.0, 2.0, -5.0);
        let look_at = vec3(0.0, 0.5, 0.0);
        Box::new(PerspectiveCamera::new(
            resolution,
            look_from,
            look_at,
            25.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    let aggregate = build_aggregate();

    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}

fn build_aggregate() -> Vec<Primitive> {
    let floor = {
        let mat = build_matte_constant(Vec3::splat(0.5));
        let shape = Box::new(Floor::new());
        Primitive::new(shape, mat)
    };

    let dragon = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(Vec3::splat(1.0)));
            Rc::new(DielectricMaterial::new(tex, 1.5))
        };

        let result = load_ply("./assets/dragon/dragon_vrip.ply").unwrap();
        let shape = Box::new(TriangleMesh::new(result.vertices, result.indices));
        let transform = Affine3A::from_scale_rotation_translation(
            Vec3::splat(10.0),
            Quat::IDENTITY,
            vec3(0.0, -0.53, 0.0),
        );
        let shape = Box::new(Transform::new(transform, shape));
        Primitive::new(shape, mat)
    };

    let sphere_light = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(vec3(100.0, 50.0, 25.0)));
            Rc::new(DiffuseLightMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(vec3(-40.0, 40.0, 50.0), 5.0));
        Primitive::new(shape, mat)
    };

    let light_dome = hdr_light_dome("./assets/lightprobes/ennis.exr");

    vec![floor, dragon, sphere_light, light_dome]
}
