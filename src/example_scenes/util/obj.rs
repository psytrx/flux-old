use std::{fmt::Debug, path::Path};

use anyhow::Result;
use glam::{vec3, Vec3};
use log::{trace, warn};
use tobj::{LoadOptions, Material};

use super::ModelResult;

pub fn load_obj<P: AsRef<Path> + Debug>(filename: P) -> Result<(Vec<ModelResult>, Vec<Material>)> {
    let (models, maybe_materials) = tobj::load_obj(
        &filename,
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let materials = maybe_materials.unwrap_or_else(|err| {
        warn!(
            "Failed to load materials from obj file {:?}: {}",
            &filename, err
        );
        vec![]
    });

    let models = models
        .iter()
        .enumerate()
        .map(|(i, model)| {
            let vertices: Vec<Vec3> = model
                .mesh
                .positions
                .chunks(3)
                .map(|chunk| match chunk {
                    [x, y, z] => vec3(*x, *y, *z),
                    _ => panic!("Invalid chunk size in obj mesh"),
                })
                .collect();
            let indices = model
                .mesh
                .indices
                .iter()
                .map(|index| *index as usize)
                .collect();

            trace!(
                "Loaded model {} from {:?} ({} vertices)",
                i + 1,
                &filename,
                vertices.len()
            );

            ModelResult { vertices, indices }
        })
        .collect();

    Ok((models, materials))
}
