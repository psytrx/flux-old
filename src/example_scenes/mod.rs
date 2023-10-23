mod material_demo;

use crate::flux::Scene;
use log::debug;
use material_demo::material_demo;

pub enum ExampleScene {
    MaterialDemo,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    debug!("load_example_scene");
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
    }
}
