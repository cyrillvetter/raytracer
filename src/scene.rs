use std::path::Path;

use crate::bvh::Bvh;
use crate::primitive::Color;
use crate::triangle::{Triangle, Vertex};
use crate::material::{Material, Phong, Metal, Glass, Texture, Emissive};
use crate::Camera;
use crate::Light;

use super::{IMAGE_HEIGHT, IMAGE_WIDTH};

use gltf::Document;
use gltf::buffer::Data;
use gltf::camera::Projection::Perspective;
use glam::{Vec3A, Vec3, Vec2, Quat, Affine3A};

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub bvh: Bvh,
    pub materials: Vec<Material>,
    pub images: Vec<gltf::image::Data>,
}

impl Scene {
    pub fn import(path: &Path) -> Self {
        let (gltf, buffers, images) = gltf::import(path).unwrap();
        let triangles = import_triangles(&gltf, &buffers);

        Scene {
            camera: import_camera(&gltf),
            lights: import_lights(&gltf),
            bvh: Bvh::new(triangles),
            materials: import_materials(&gltf),
            images
        }
    }
}

fn import_lights(gltf: &Document) -> Vec<Light> {
    gltf.nodes()
        .filter_map(|node| {
            node.light().map(|light| {
                Light::new(
                    node.transform().decomposed().0.into(),
                    light.color().into(),
                    light.intensity() / 1000.0
                )
            })
        })
        .collect()
}

fn import_camera(gltf: &Document) -> Camera {
    gltf.nodes()
        .find_map(|node| {
            node.camera().map(|cam| {
                let Perspective(persp) = cam.projection() else {
                    panic!("Orthographic camera not supported.");
                };

                let aspect_ratio = persp.aspect_ratio().unwrap_or((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32));
                let (trans, rot, _) = node.transform().decomposed();
                let transform = Affine3A::from_rotation_translation(Quat::from_array(rot), Vec3::from_array(trans));

                Camera::new(
                    aspect_ratio,
                    persp.yfov(),
                    transform
                )
            })
        })
        .unwrap_or(Camera::new((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32), 0.4, Affine3A::ZERO))
}

fn import_triangles(gltf: &Document, buffers: &Vec<Data>) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();

    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions: Vec<Vec3A> = reader.read_positions().unwrap().map(|a| a.into()).collect();
            let normals: Vec<Vec3A> = reader.read_normals().unwrap().map(|a| a.into()).collect();
            let uvs: Vec<Vec2> = reader.read_tex_coords(0).unwrap().into_f32().map(|a| a.into()).collect();

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

            let load_vertex = |i: usize| {
                let idx = indices[i];
                Vertex::new(positions[idx], normals[idx], uvs[idx])
            };

            for i in (0..indices.len()).step_by(3) {
                let triangle = Triangle::new(
                    load_vertex(i),
                    load_vertex(i + 1),
                    load_vertex(i + 2),
                    material_index,
                );

                triangles.push(triangle);
            }
        }
    }

    triangles
}

fn import_materials(gltf: &Document) -> Vec<Material> {
    gltf
        .materials()
        .map(|material| {
            let pbr = material.pbr_metallic_roughness();

            if let Some(texture_info) = pbr.base_color_texture() {
                return Material::Texture( Texture {
                    index: texture_info.texture().index(),
                });
            }

            let base_color_factor = pbr.base_color_factor();
            let color = Color::rgb(base_color_factor[0], base_color_factor[1], base_color_factor[2]);
            let metallic = pbr.metallic_factor();

            if let Some(_) = material.transmission() {
                Material::Glass(Glass {
                    color
                })
            } else if material.emissive_factor() == [1.0, 1.0, 1.0] {
                Material::Emissive(Emissive {
                    color: material.emissive_factor().into()
                })
            } else if metallic < 1.0 {
                Material::Phong(Phong {
                    color,
                    roughness: pbr.roughness_factor()
                })
            } else {
                Material::Metal(Metal {
                    color
                })
            }
        })
        .collect()
}
