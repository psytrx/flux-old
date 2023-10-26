use std::rc::Rc;

use glam::{vec3, Vec3};

use crate::{
    example_scenes::util::build_matte_constant,
    flux::{
        shapes::{Floor, Sphere},
        textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture, UvTexture},
        DielectricMaterial, MatteMaterial, MetalMaterial, Primitive,
    },
};

pub fn material_demo_aggregate() -> Vec<Primitive> {
    let floor = {
        let mat = {
            let even = Rc::new(ConstantTexture::new(Vec3::splat(0.7)));
            let odd = Rc::new(ConstantTexture::new(Vec3::splat(0.5)));
            let tex = Rc::new(CheckerTexture::new(0.5, even, odd));
            Rc::new(MatteMaterial::new(tex))
        };
        let shape = Box::new(Floor::new());
        Primitive::new(shape, mat)
    };

    let earth_sphere = {
        let mat = {
            let img = image::open("./assets/earthmap.jpg").unwrap();
            let tex = Rc::new(ImageTexture::new(img));
            Rc::new(MatteMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(vec3(0.0, 3.0, 4.0), 3.0));
        Primitive::new(shape, mat)
    };

    let dielectric_sphere = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(Vec3::ONE));
            Rc::new(DielectricMaterial::new(tex, 1.5))
        };
        let shape = Box::new(Sphere::new(vec3(-2.5, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat)
    };

    let matte_sphere = {
        let mat = build_matte_constant(vec3(0.2, 0.5, 0.1));
        let shape = Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat)
    };

    let metal_sphere = {
        let mat = {
            let tex = Rc::new(ConstantTexture::new(vec3(0.8, 0.6, 0.2)));
            Rc::new(MetalMaterial::new(tex, 0.05))
        };
        let shape = Box::new(Sphere::new(vec3(2.5, 1.0, 0.0), 1.0));
        Primitive::new(shape, mat)
    };

    let uv_sphere = {
        let mat = {
            let tex = Rc::new(UvTexture::new());
            Rc::new(MatteMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(vec3(-2.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat)
    };

    let checkered_sphere = {
        let mat = {
            let even = Rc::new(ConstantTexture::new(Vec3::ZERO));
            let odd = Rc::new(ConstantTexture::new(Vec3::ONE));
            let tex = Rc::new(CheckerTexture::new(0.1, even, odd));
            Rc::new(MatteMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(vec3(0.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat)
    };

    let noise_sphere = {
        let mat = {
            let tex = Rc::new(NoiseTexture::new(0.025));
            Rc::new(MatteMaterial::new(tex))
        };
        let shape = Box::new(Sphere::new(vec3(2.0, 0.5, -2.0), 0.5));
        Primitive::new(shape, mat)
    };

    vec![
        floor,
        earth_sphere,
        dielectric_sphere,
        matte_sphere,
        metal_sphere,
        uv_sphere,
        checkered_sphere,
        noise_sphere,
    ]
}
