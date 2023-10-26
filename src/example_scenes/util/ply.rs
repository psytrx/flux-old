use std::{fmt::Debug, path::Path};

use anyhow::Result;
use glam::vec3;
use log::trace;
use ply_rs::{
    parser::Parser,
    ply::{Property, PropertyAccess},
};

use crate::example_scenes::util::ModelResult;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl PropertyAccess for Vertex {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("x", Property::Float(v)) => self.x = v,
            ("y", Property::Float(v)) => self.y = v,
            ("z", Property::Float(v)) => self.z = v,
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

struct Face {
    vertex_index: Vec<usize>,
}

impl PropertyAccess for Face {
    fn new() -> Self {
        Face {
            vertex_index: Vec::new(),
        }
    }
    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("vertex_index", Property::ListUInt(vec)) => {
                self.vertex_index = vec.iter().map(|i| *i as usize).collect()
            }
            ("vertex_indices", Property::ListInt(vec)) => {
                self.vertex_index = vec.iter().map(|i| *i as usize).collect()
            }
            (k, prop) => panic!(
                "Face: Unexpected key/value combination: key: {}, prop: {:?}",
                k, prop
            ),
        }
    }
}

pub fn load_ply<P: AsRef<Path> + Debug>(path: P) -> Result<ModelResult> {
    let f = std::fs::File::open(&path).unwrap();
    let mut f = std::io::BufReader::new(f);

    let vertex_parser = Parser::<Vertex>::new();
    let face_parser = Parser::<Face>::new();

    let header = vertex_parser.read_header(&mut f).unwrap();

    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();

    for (_ignore_key, element) in &header.elements {
        match element.name.as_ref() {
            "vertex" => {
                vertex_list = vertex_parser
                    .read_payload_for_element(&mut f, element, &header)
                    .unwrap();
            }
            "face" => {
                face_list = face_parser
                    .read_payload_for_element(&mut f, element, &header)
                    .unwrap();
            }
            _ => panic!("Unexpeced element!"),
        }
    }

    trace!(
        "Loaded PLY model from {:?} ({} vertices, {} faces)",
        &path,
        vertex_list.len(),
        face_list.len()
    );

    let vertices = vertex_list.iter().map(|v| vec3(v.x, v.y, v.z)).collect();

    let indices = face_list
        .iter()
        .flat_map(|f| {
            // We currently only support triangle meshes
            assert_eq!(f.vertex_index.len(), 3);
            f.vertex_index.clone()
        })
        // .copied()
        .collect();

    Ok(ModelResult { vertices, indices })
}
