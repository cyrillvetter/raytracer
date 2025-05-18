use crate::{
    IMAGE_WIDTH, IMAGE_HEIGHT,
    triangle::{Triangle, Vertex},
    material, Material,
    Sampler,
    Camera,
    Bvh,
    Texture
};

use std::path::Path;
use gltf::{
    Document,
    buffer::Data,
    camera::Projection::Perspective,
};
use glam::{Vec3A, Vec2, Affine3A, Mat4};

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub camera: Camera,
    pub bvh: Bvh,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>
}

impl Scene {
    pub fn import(path: &Path) -> Self {
        let (gltf, buffers, images) = gltf::import(path).unwrap();

        let materials = import_materials(&gltf);
        let textures = import_textures(&images, &materials);

        let triangles = import_triangles(&gltf, &buffers);

        Scene {
            name: path.file_stem().map_or("image", |s| s.to_str().unwrap()).to_owned(),
            camera: import_camera(&gltf),
            bvh: Bvh::new(triangles),
            materials,
            textures
        }
    }
}

fn import_camera(gltf: &Document) -> Camera {
    gltf
        .nodes()
        .find_map(|node| node.camera().map(|cam| {
            let Perspective(persp) = cam.projection() else {
                panic!("Orthographic camera not supported");
            };

            let aspect_ratio = persp.aspect_ratio().unwrap_or((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32));
            let transform = get_node_transform(&node);

            Camera::new(
                aspect_ratio,
                persp.yfov(),
                transform
            )
        }))
        .expect("Cannot import camera")
}

fn import_triangles(gltf: &Document, buffers: &Vec<Data>) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();

    for node in gltf.nodes() {
        let Some(mesh) = node.mesh() else {
            continue;
        };
        let transform = get_node_transform(&node);

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions: Vec<Vec3A> = reader.read_positions().unwrap().map(|a| a.into()).collect();
            let normals: Vec<Vec3A> = reader.read_normals().unwrap().map(|a| a.into()).collect();
            let uvs: Option<Vec<Vec2>> = reader
                .read_tex_coords(0)
                .map(|tex| tex
                    .into_f32()
                    .map(|a| a.into())
                    .collect());

            let indices: Vec<usize> = reader
                .read_indices()
                .unwrap()
                .into_u32()
                .map(|i| i as usize)
                .collect();

            let triangle_amount = indices.len() / 3;
            triangles.reserve(triangle_amount);

            let material_index = primitive.material().index();

            let load_vertex = |i: usize| {
                let idx = indices[i];

                let position = transform.transform_point3a(positions[idx]);
                let normal = transform.transform_vector3a(normals[idx]).normalize();

                Vertex::new(position, normal, uvs.as_ref().map(|uv| uv[idx]))
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
            let color_sampler = match pbr.base_color_texture() {
                Some(texture_info) => Sampler::Texture(texture_info.texture().index()),
                _ => Sampler::Color(pbr.base_color_factor().into())
            };

            if let Some(_) = material.transmission() {
                Material::Glass(material::Glass {
                    color_sampler
                })
            } else if material.emissive_factor().iter().any(|v| *v > 0.0) {
                Material::Emissive(material::Emissive {
                    color: material.emissive_factor().into()
                })
            } else if pbr.metallic_factor() < 1.0 {
                Material::Diffuse(material::Diffuse {
                    color_sampler
                })
            } else {
                Material::Metal(material::Metal {
                    color_sampler,
                })
            }
        })
        .collect()
}

fn import_textures(images: &Vec<gltf::image::Data>, materials: &Vec<Material>) -> Vec<Texture> {
    let color_texture_indices: Vec<usize> = materials
        .iter()
        .filter_map(|mat| mat.get_color_texture_index())
        .collect();

    images
        .iter()
        .enumerate()
        .map(|(i, data)| Texture::new(&data, color_texture_indices.contains(&i)))
        .collect()
}

fn get_node_transform(node: &gltf::Node) -> Affine3A {
    let transform_matrix = Mat4::from_cols_array_2d(&node.transform().matrix());
    Affine3A::from_mat4(transform_matrix)
}
