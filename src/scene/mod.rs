pub mod camera;
pub use camera::Camera;

pub mod light;
pub use light::Light;

use crate::primitive::*;
use crate::hittable::{Hittable, Mesh, Triangle};

use gltf::mesh::util::ReadIndices::U16;

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn import(path: &str) -> Self {
        let (gltf, buffers, _) = gltf::import(path).unwrap();

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let positions: Vec<Vec3> = reader.read_positions().unwrap().map(|a| a.into()).collect();

                let Some(U16(index_buffer)) = reader.read_indices() else {
                    panic!("Index type not supported");
                };

                let indices: Vec<usize> = index_buffer.map(|i| i as usize).collect();
                let triangle_amount = indices.len() / 3;
                let mut triangles: Vec<Triangle> = Vec::with_capacity(triangle_amount);

                for i in (0..indices.len()).step_by(3) {
                    triangles.push(Triangle::new(positions[indices[i]], positions[indices[i + 1]], positions[indices[i + 2]]));
                }

                objects.push(Box::new(Mesh::new(triangles, Color::WHITE)));
            }
        }

        Scene {
            camera: Camera::new(1.0),
            lights: vec![
                Light::new(Vec3::new(-10.0, 7.0, 12.0), Color::rgb(0.992, 0.973, 0.918), 1.0),
            ],
            objects
        }
    }
}
