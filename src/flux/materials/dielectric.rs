use glam::Vec3;

use rand::{rngs::StdRng, Rng};

use crate::flux::{interaction::Interaction, ray::Ray};

use super::{reflect, refract, Material, ScatterRec};

pub struct DielectricMaterial {
    ior: f32,
}

impl DielectricMaterial {
    pub fn new(ior: f32) -> Self {
        Self { ior }
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, int: &Interaction, rng: &mut StdRng) -> Option<ScatterRec> {
        let attenuation = Vec3::ONE;

        let refraction_ratio = if int.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(int.n).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflecting = cannot_refract && reflectance(cos_theta, refraction_ratio) > rng.gen();

        let direction = if reflecting {
            reflect(unit_direction, int.n)
        } else {
            refract(unit_direction, int.n, refraction_ratio)
        };
        let scattered = Some(int.spawn_ray(direction));

        Some(ScatterRec {
            attenuation,
            scattered,
        })
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[test]
fn test_refracting_ray() {
    let mut rng = <StdRng as rand::SeedableRng>::seed_from_u64(0);

    let mat = std::rc::Rc::new(DielectricMaterial::new(1.5));

    let sphere = {
        let shape = crate::flux::Sphere::new(Vec3::ZERO, 1.0);
        crate::flux::Primitive::new(shape, mat.clone())
    };

    let dummy_cam = crate::flux::Camera::new(glam::UVec2::ZERO, Vec3::ZERO, Vec3::ZERO, 0.0);
    let scene = crate::flux::Scene::new(dummy_cam, vec![sphere]);

    let p0 = glam::vec3(0.2, 0.2, -4.0);
    println!("p0: {:?}", p0);

    let ray = Ray::new(p0, Vec3::Z, 0.0);
    let int = scene.intersect(&ray).unwrap();
    let p1 = int.p;
    println!("p1: {:?}", p1);

    let scattered_ray = mat.scatter(&ray, &int, &mut rng).unwrap().scattered;
    println!("scattered: {:?}", scattered_ray);

    let int = scene.intersect(&ray).unwrap();
    let p2 = int.p;
    println!("p2: {:?}", p2);

    let offset = 0.57735;
    let internal_ray = Ray::new(glam::vec3(offset, offset, -offset), Vec3::Z, 0.0);
    let int = scene.intersect(&internal_ray).unwrap();
    let p_internal = int.p;
    println!("p_internal: {:?}", p_internal);
}
