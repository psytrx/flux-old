use std::rc::Rc;

use anyhow::Result;
use glam::{uvec2, vec3, Affine3A, Vec3};

use crate::{
    example_scenes::util::load_obj,
    flux::{
        shapes::{Floor, Sphere, Transform, TriangleMesh},
        textures::{CheckerTexture, ConstantTexture, ImageTexture, MultiplyTexture},
        DiffuseLightMaterial, MatteMaterial, MetalMaterial, PerspectiveCamera, Primitive, Scene,
    },
};

pub fn suzanne() -> Scene {
    let camera = {
        let resolution = uvec2(1024, 1024);
        let look_from = vec3(-1.0, 2.0, 4.0);
        let look_at = vec3(0.0, 0.9, 0.0);
        Box::new(PerspectiveCamera::new(
            resolution,
            look_from,
            look_at,
            45.0,
            0.02,
            look_at.distance(look_from),
        ))
    };

    let aggregate =
        build_aggregate().unwrap_or_else(|err| panic!("Failed to build wavefront scene: {}", err));
    let lights = vec![];

    Scene::new(camera, aggregate, lights)
}

fn build_aggregate() -> Result<Vec<Primitive>> {
    let floor = {
        let mat = {
            let tex = {
                let even = Rc::new(ConstantTexture::new(Vec3::splat(0.7)));
                let odd = Rc::new(ConstantTexture::new(Vec3::splat(0.9)));
                Rc::new(CheckerTexture::new(0.2, even, odd))
            };
            Rc::new(MetalMaterial::new(tex, 0.1))
        };
        let shape = Box::new(Floor::new());
        Primitive::new(shape, mat)
    };

    let suzanne = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(Vec3::splat(0.5)));
            Rc::new(MatteMaterial::new(tex))
        };

        let (models, _) = load_obj("./assets/suzanne/suzanne.obj")?;
        let vertices = models[0].vertices.clone();
        let indices = models[0].indices.clone();
        let shape = Box::new(TriangleMesh::new(vertices, indices));
        let transform = Affine3A::from_translation(vec3(0.0, 0.95, 0.0));
        let shape = Transform::new(transform, shape);
        let shape = Box::new(shape);
        Primitive::new(shape, mat)
    };

    let light_dome = {
        let mat = {
            let img = image::open("./assets/lightprobes/pisa.exr").unwrap();
            let tex = Rc::new(ImageTexture::new(img));
            let tex = Rc::new(MultiplyTexture::new(2.0, tex));
            Rc::new(DiffuseLightMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(Vec3::ZERO, 1_000.0));
        Primitive::new(shape, mat)
    };

    Ok(vec![floor, suzanne, light_dome])
}
