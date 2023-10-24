use std::rc::Rc;

use glam::{uvec2, vec2, vec3, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::flux::{
    shapes::{Floor, Sphere},
    Bounds2, Camera, DielectricMaterial, Material, MatteMaterial, MetalMaterial, Primitive, Scene,
};

use super::sample_disks;

pub fn many_spheres() -> Scene {
    let camera = {
        let resolution = uvec2(800, 450);
        let look_from = vec3(13.0, 4.0, -3.0);
        let look_at = vec3(2.5, 0.5, 0.0);

        Camera::new(
            resolution,
            look_from,
            Vec3::ZERO,
            35.0,
            0.025,
            look_at.distance(look_from),
        )
    };

    let mut aggregate = {
        let floor_mat = Rc::new(MatteMaterial::new(Vec3::splat(0.5)));
        let left_mat = Rc::new(MatteMaterial::new(vec3(0.4, 0.2, 0.1)));
        let center_mat = Rc::new(DielectricMaterial::new(1.5));
        let right_mat = Rc::new(MetalMaterial::new(vec3(0.7, 0.6, 0.5), 0.025));

        let floor = {
            let shape = Box::new(Floor::new());
            Primitive::new(shape, floor_mat.clone())
        };

        let left_sphere = {
            let shape = Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, left_mat.clone())
        };

        let center_sphere = {
            let shape = Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, center_mat.clone())
        };

        let right_sphere = {
            let shape = Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, right_mat.clone())
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
                Rc::new(MatteMaterial::new(albedo))
            } else if choose_mat < 0.9 {
                let albedo = vec3(
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                );
                let fuzz = rng.gen_range(0.0..0.5);
                Rc::new(MetalMaterial::new(albedo, fuzz))
            } else {
                Rc::new(DielectricMaterial::new(1.5))
            };

            let shape = Box::new(Sphere::new(sphere_pos, radius));
            let primitive = Primitive::new(shape, material);
            aggregate.push(primitive);
        });

    Scene::new(camera, aggregate)
}
