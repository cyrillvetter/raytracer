pub mod camera;
pub use camera::Camera;

pub mod light;
pub use light::Light;

use crate::primitive::Color;
use crate::triangle::{Triangle, Vertex};

use gltf::mesh::util::ReadIndices::U16;
use glam::Vec3;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub triangles: Vec<Triangle>
}

impl Scene {
    pub fn import(path: &str) -> Self {
        let (gltf, buffers, _) = gltf::import(path).unwrap();

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

                for i in (0..indices.len()).step_by(3) {
                    let triangle = Triangle {
                        v1: Vertex::new(positions[indices[i]], normals[indices[i]]),
                        v2: Vertex::new(positions[indices[i + 1]], normals[indices[i + 1]]),
                        v3: Vertex::new(positions[indices[i + 2]], normals[indices[i + 2]]),
                        color: Color::rgb_u8(54, 55, 207)
                    };

                    triangles.push(triangle);
                }
            }
        }

        Scene {
            camera: Camera::new(1.0),
            lights: vec![
                Light::new(Vec3::new(-10.0, 7.0, 18.0), Color::rgb(0.992, 0.973, 0.918), 1.0),
            ],
            triangles
        }
    }
}
