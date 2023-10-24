mod defocus_blur;
mod material_demo;

use std::rc::Rc;

use glam::vec3;
use log::debug;

use crate::flux::{DielectricMaterial, MatteMaterial, MetalMaterial, Primitive, Scene, Sphere};

use defocus_blur::defocus_blur;
use material_demo::material_demo;

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    DefocusBlur,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
    }
}

pub fn material_demo_primitives() -> Vec<Primitive> {
    let mat_floor = Rc::new(MatteMaterial::new(vec3(0.8, 0.8, 0.0)));
    let mat_left = Rc::new(DielectricMaterial::new(1.5));
    let mat_center = Rc::new(MatteMaterial::new(vec3(0.1, 0.2, 0.5)));
    let mat_right = Rc::new(MetalMaterial::new(vec3(0.8, 0.6, 0.2), 0.05));

    let floor = {
        let shape = Sphere::new(vec3(0.0, -100.0, 0.0), 100.0);
        Primitive::new(shape, mat_floor.clone())
    };

    let left_sphere = {
        let shape = Sphere::new(vec3(-2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_left.clone())
    };

    let center_sphere = {
        let shape = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_center.clone())
    };

    let right_sphere = {
        let shape = Sphere::new(vec3(2.0, 1.0, 0.0), 1.0);
        Primitive::new(shape, mat_right.clone())
    };

    vec![floor, left_sphere, center_sphere, right_sphere]
}
