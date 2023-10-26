use std::rc::Rc;

use glam::{uvec2, vec2, vec3, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    example_scenes::util::build_matte_constant,
    flux::{
        shapes::{Floor, Sphere},
        textures::ConstantTexture,
        Bounds2, DielectricMaterial, Material, MetalMaterial, PerspectiveCamera, Primitive, Scene,
    },
};

use super::{default_sky_light, sample_disks};

pub fn many_spheres() -> Scene {
    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(13.0, 4.0, -3.0);
        let look_at = vec3(2.5, 0.5, 0.0);

        Box::new(PerspectiveCamera::new(
            resolution,
            look_from,
            Vec3::ZERO,
            35.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    let aggregate = build_aggregate();
    let lights = vec![default_sky_light()];

    Scene::new(camera, aggregate, lights)
}

fn build_aggregate() -> Vec<Primitive> {
    let mut aggregate = {
        let floor = {
            let mat = build_matte_constant(Vec3::splat(0.5));
            let shape = Box::new(Floor::new());
            Primitive::new(shape, mat)
        };

        let left_sphere = {
            let mat = build_matte_constant(vec3(0.4, 0.2, 0.1));
            let shape = Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, mat)
        };

        let center_sphere = {
            let mat = {
                let tex = Rc::new(ConstantTexture::new(Vec3::ONE));
                Rc::new(DielectricMaterial::new(tex, 1.5))
            };
            let shape = Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, mat)
        };

        let right_sphere = {
            let mat = {
                let tex = Rc::new(ConstantTexture::new(vec3(0.7, 0.6, 0.5)));
                Rc::new(MetalMaterial::new(tex, 0.025))
            };
            let shape = Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, mat)
        };

        vec![floor, left_sphere, center_sphere, right_sphere]
    };

    let radius = 0.2;
    let bounds_dim = 20.0;
    let bounds = Bounds2::new(vec2(-bounds_dim, -bounds_dim), vec2(bounds_dim, bounds_dim));
    let mut rng = StdRng::seed_from_u64(0);
    sample_disks(bounds, 4.0 * radius, 32, 8, &mut rng)
        .into_iter()
        .for_each(|sample| {
            let sphere_pos = vec3(sample.x, radius, sample.y);

            let safe_box = vec3(sample.x / 6.0, radius, sample.y);
            let safe_box_dist = safe_box.length();
            if safe_box_dist < 1.0 {
                return;
            }

            let choose_mat: f32 = rng.gen();
            let material: Rc<dyn Material> = if choose_mat < 0.6 {
                // diffuse
                let albedo = rng.gen::<Vec3>() * rng.gen::<Vec3>();
                build_matte_constant(albedo)
            } else if choose_mat < 0.9 {
                // metal
                let albedo = vec3(
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                );
                let tex = Rc::new(ConstantTexture::new(albedo));
                let fuzz = rng.gen_range(0.0..0.5);
                Rc::new(MetalMaterial::new(tex, fuzz))
            } else {
                // dielectric
                let albedo = vec3(
                    rng.gen::<f32>().powf(1.0 / 4.0),
                    rng.gen::<f32>().powf(1.0 / 4.0),
                    rng.gen::<f32>().powf(1.0 / 4.0),
                );
                let tex = Rc::new(ConstantTexture::new(albedo));
                Rc::new(DielectricMaterial::new(tex, 1.5))
            };

            let shape = Box::new(Sphere::new(sphere_pos, radius));
            let primitive = Primitive::new(shape, material);
            aggregate.push(primitive);
        });

    aggregate
}
