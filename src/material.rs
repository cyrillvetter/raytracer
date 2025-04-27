use crate::{
    primitive::{Color, Ray},
    triangle::HitRecord,
    Scene
};

use glam::Vec3A;
use fastrand::f32;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color);
}

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    Glass(Glass),
    Texture(Texture),
    Emissive(Emissive),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
            Material::Glass(glass) => glass.scatter(ray, hit_record, scene),
            Material::Texture(texture) => texture.scatter(ray, hit_record, scene),
            Material::Emissive(emissive) => emissive.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diffuse {
    pub color: Color,
    pub roughness: f32,
}

impl Scatterable for Diffuse {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        let ray_direction = random_on_hemisphere(hit_record.normal);
        (Some(Ray::new(hit_record.point + ray_direction * 1e-5, ray_direction)), self.color * 0.8)
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
    pub color: Color,
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();
        (Some(Ray::new(hit_record.point + reflection_dir * 1e-5, reflection_dir)), self.color)
    }
}

#[derive(Debug, Clone)]
pub struct Glass {
    pub color: Color
}

const GLASS_IOR: f32 = 1.52;

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        let eta = if hit_record.front_face { GLASS_IOR.recip() } else { GLASS_IOR };

        let cos_theta = (-ray.direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = eta * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, eta) > f32() {
            ray.direction.reflect(hit_record.normal)
        } else {
            ray.direction.refract(hit_record.normal, eta)
        };

        (Some(Ray::new(hit_record.point + direction * 1e-5, direction)), self.color)
    }
}

// Schlick's approximation.
fn reflectance(cosine: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture_index: usize
}

impl Scatterable for Texture {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let texture_color = scene.textures[self.texture_index].sample(hit_record.uv);
        let ray_direction = random_on_hemisphere(hit_record.normal);
        (Some(Ray::new(hit_record.point + ray_direction * 1e-5, ray_direction)), texture_color)
    }
}

#[derive(Debug, Clone)]
pub struct Emissive {
    pub color: Color
}

impl Scatterable for Emissive {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        (None, self.color)
    }
}
