use std::path::Path;

use crate::primitive::Color;
use crate::triangle::{Triangle, Vertex};
use crate::material::Material;
use crate::Camera;
use crate::Light;

use super::{IMAGE_HEIGHT, IMAGE_WIDTH};

use gltf::Document;
use gltf::camera::Projection::Perspective;
use glam::{Vec3, Quat, Affine3A};

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

        let camera = import_camera(&gltf);

        let materials: Vec<Material> = gltf
            .materials()
            .map(|material| {
                let base_color = material.pbr_metallic_roughness().base_color_factor();
                Material::Solid(Color::from_linear(
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

                // TODO: Remove into_32 to avoid casting twice.
                let indices: Vec<usize> = reader
                    .read_indices()
                    .expect("No indices found")
                    .into_u32()
                    .map(|i| i as usize)
                    .collect();

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

        fn import_camera(gltf: &Document) -> Camera {
            let gltf_camera = gltf.cameras().next();
            match gltf_camera {
                Some(cam) => match cam.projection() {
                    Perspective(persp) => {
                        let transform = gltf.nodes().find_map(|n| match n.camera() {
                            Some(c) if c.index() == cam.index() => {
                                let (trans, rot, _) = n.transform().decomposed();
                                Some(Affine3A::from_rotation_translation(Quat::from_array(rot), Vec3::from_array(trans)))
                            },
                            _ => None
                        })
                        .unwrap_or(Affine3A::ZERO);

                        // TODO: Maybe always set the aspect ratio based on the image height/width.
                        Camera::new(
                            persp.aspect_ratio().unwrap_or((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32)),
                            persp.yfov(),
                            transform
                        )
                    },
                    _ => unimplemented!()
                },
                _ => Camera::new((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32) / 2.0, 0.4, Affine3A::ZERO),
            }
        }

        Scene {
            camera,
            lights: vec![
                Light::new(Vec3::new(-0.62623, 2.0163, 2.2484), Color::WHITE, 1.5),
            ],
            triangles,
            materials
        }
    }
}
