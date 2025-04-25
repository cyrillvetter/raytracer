use crate::primitive::{Color, Ray};
use crate::triangle::HitRecord;
use crate::scene::Scene;

use glam::Vec3A;
use fastrand::f32;
use gltf::image::Format;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color, f32);
}

#[derive(Debug, Clone)]
pub enum Material {
    Phong(Phong),
    Metal(Metal),
    Glass(Glass),
    Texture(Texture)
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color, f32) {
        match self {
            Material::Phong(phong) => phong.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
            Material::Glass(glass) => glass.scatter(ray, hit_record, scene),
            Material::Texture(texture) => texture.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phong {
    pub color: Color,
    pub roughness: f32,
}

impl Scatterable for Phong {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color, f32) {
        let ray_direction = random_on_hemisphere(hit_record.normal);
        (Some(Ray::new(hit_record.point + ray_direction * 1e-5, ray_direction)), self.color, 0.5)
    }
}

fn rand() -> f32 {
    f32() * 2.0 - 1.0
}

fn random_unit_vector() -> Vec3A {
    loop {
        let p = Vec3A::new(rand(), rand(), rand());
        let lensq = p.length_squared();
        if 1e-30 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

fn random_on_hemisphere(normal: Vec3A) -> Vec3A {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub color: Color
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color, f32) {
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();
        (Some(Ray::new(hit_record.point + reflection_dir * 1e-5, reflection_dir)), self.color, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct Glass {
    pub color: Color
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color, f32) {
        let eta = if hit_record.front_face { 1.45f32.recip() } else { 1.45f32 };
        let refraction_dir = ray.direction.refract(hit_record.normal, eta);
        (Some(Ray::new(hit_record.point + refraction_dir * 1e-5, refraction_dir)), self.color, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub index: usize
}

impl Scatterable for Texture {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let image = &scene.images[self.index];
        let x = (hit_record.uv.x.clamp(0.0, 1.0) * (image.width - 1) as f32).round() as usize;
        let y = ((hit_record.uv.y).clamp(0.0, 1.0) * (image.height - 1) as f32).round() as usize;

        let index = (y * image.width as usize + x) * get_image_components(image.format);
        let pixels = &image.pixels;

        let base_color = Color::rgb_u8(
            pixels[index],
            pixels[index + 1],
            pixels[index + 2],
        );

        let mut color = base_color * AMBIENT_FACTOR;

        for light in scene.lights.iter() {
            let light_vec = light.origin - hit_record.point;
            let light_distance = light_vec.length();
            let light_dir = light_vec / light_distance;
            let light_intensity = light.intensity / (scene.lights.len() as f32);

            let shadow_ray = Ray::new(hit_record.point + light_dir * 1e-5, light_dir);

            // TODO: Maybe add another function that finds intersections between the surface and a lightsource.
            let in_shadow = scene.bvh.intersects(&shadow_ray).map_or(f32::INFINITY, |h| h.t) < light_distance;

            if !in_shadow {
                let s = (light.origin - hit_record.point).normalize();
                let diffuse = base_color *
                    s.dot(hit_record.normal).max(0.0) *
                    light_distance.powf(2.0).recip() *
                    light.color *
                    light_intensity;

                color += diffuse;
            }
        }

        (None, color.clamp())
    }
}

fn get_image_components(format: Format) -> usize {
    use Format::*;
    match format {
        R8G8B8A8 | R16G16B16A16 | R32G32B32A32FLOAT => 4,
        _ => 3
    }
}
