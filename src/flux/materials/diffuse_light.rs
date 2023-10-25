use std::rc::Rc;

use glam::Vec3;
use rand::rngs::StdRng;

use crate::flux::{interaction::Interaction, ray::Ray, textures::Texture};

use super::{BxdfType, Material, ScatterRec};

pub struct DiffuseLightMaterial {
    emit: Rc<dyn Texture<Vec3>>,
}

impl DiffuseLightMaterial {
    pub fn new(emit: Rc<dyn Texture<Vec3>>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLightMaterial {
    fn scatter(&self, _ray: &Ray, _int: &Interaction, _rng: &mut StdRng) -> Option<ScatterRec> {
        None
    }

    fn emitted(&self, int: &Interaction) -> Vec3 {
        self.emit.evaluate(int)
    }

    fn bxdf_type(&self) -> BxdfType {
        BxdfType::Other
    }
}
