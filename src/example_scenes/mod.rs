mod defocus_blur;
mod material_demo;

use crate::flux::Scene;
use defocus_blur::defocus_blur;
use log::debug;
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
