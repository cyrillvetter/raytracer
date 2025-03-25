use std::path::Path;

use crate::primitive::Color;
use crate::triangle::{Triangle, Vertex};
use crate::material::Material;
use crate::Camera;
use crate::Light;

use gltf::mesh::util::ReadIndices::U16;
use gltf::camera::Projection::Orthographic;
use glam::Vec3;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>,
    pub materials: Vec<Material>
}

impl Scene {
    pub fn import(path: &Path) -> Self {
        let (gltf, buffers, _) = gltf::import(path).unwrap();

        let gltf_camera = gltf.cameras().next();
        let camera = match gltf_camera {
            Some(cam) => match cam.projection() {
                Orthographic(orth) => Camera::orthographic(orth.ymag(), -orth.znear()),
                _ => unimplemented!()
            },
            _ => Camera::orthographic(0.5, -0.1),
        };

        let materials: Vec<Material> = gltf
            .materials()
            .map(|material| {
                let base_color = material.pbr_metallic_roughness().base_color_factor();
                Material::Solid(Color::rgb(
                    base_color[0],
                    base_color[1],
                    base_color[2],
                ))
            })
            .collect();

        let mut triangles: Vec<Triangle> = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let positions: Vec<Vec3> = reader.read_positions().unwrap().map(|a| a.into()).collect();
                let normals: Vec<Vec3> = reader.read_normals().unwrap().map(|a| a.into()).collect();

                let Some(U16(index_buffer)) = reader.read_indices() else {
                    panic!("Index type not supported");
                };

                let indices: Vec<usize> = index_buffer.map(|i| i as usize).collect();

                let triangle_amount = indices.len() / 3;
                triangles.reserve(triangle_amount);

                let material_index = primitive.material().index();

                for i in (0..indices.len()).step_by(3) {
                    let triangle = Triangle {
                        v1: Vertex::new(positions[indices[i]], normals[indices[i]]),
                        v2: Vertex::new(positions[indices[i + 1]], normals[indices[i + 1]]),
                        v3: Vertex::new(positions[indices[i + 2]], normals[indices[i + 2]]),
                        material_index,
                    };

                    triangles.push(triangle);
                }
            }
        }

        Scene {
            camera,
            lights: vec![
                Light::new(Vec3::new(-10.0, 7.0, 18.0), Color::WHITE, 3.0),
            ],
            triangles,
            materials
        }
    }
}
