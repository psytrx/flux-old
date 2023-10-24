mod defocus_blur;
mod many_spheres;
mod material_demo;

use log::debug;

use crate::flux::Scene;

use defocus_blur::defocus_blur;
use many_spheres::many_spheres;
use material_demo::material_demo;

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    DefocusBlur,
    ManySpheres,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::DefocusBlur => defocus_blur(),
        ExampleScene::ManySpheres => many_spheres(),
    }
}
