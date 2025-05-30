use crate::{
    primitive::{Color, Ray},
    util::random_unit_vector,
    triangle::HitRecord,
    Sampler,
    Scene
};

use fastrand::f32;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color);
}

#[derive(Debug)]
pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    Glass(Glass),
    Emissive(Emissive),
}

impl Material {
    pub fn get_color_texture_index(&self) -> Option<usize> {
        use Material::*;
        match self {
            Diffuse(diffuse) => diffuse.color_sampler.texture_index(),
            Metal(metal) => metal.color_sampler.texture_index(),
            Glass(glass) => glass.color_sampler.texture_index(),
            _ => None
        }
    }
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        use Material::*;
        match self {
            Diffuse(diffuse) => diffuse.scatter(ray, hit_record, scene),
            Metal(metal) => metal.scatter(ray, hit_record, scene),
            Glass(glass) => glass.scatter(ray, hit_record, scene),
            Emissive(emissive) => emissive.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug)]
pub struct Diffuse {
    pub color_sampler: Sampler
}

impl Scatterable for Diffuse {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let ray_direction = (hit_record.normal + random_unit_vector()).normalize();
        let color = self.color_sampler.sample(hit_record.uv, scene);
        (Some(Ray::new(hit_record.point + ray_direction * 1e-5, ray_direction)), color)
    }
}

#[derive(Debug)]
pub struct Metal {
    pub color_sampler: Sampler
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();
        let color = self.color_sampler.sample(hit_record.uv, scene);
        (Some(Ray::new(hit_record.point + reflection_dir * 1e-5, reflection_dir)), color)
    }
}

#[derive(Debug)]
pub struct Glass {
    pub color_sampler: Sampler
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        const GLASS_IOR: f32 = 1.52;
        let eta = if hit_record.front_face { GLASS_IOR.recip() } else { GLASS_IOR };

        let cos_theta = (-ray.direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = eta * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, eta) > f32() {
            ray.direction.reflect(hit_record.normal)
        } else {
            ray.direction.refract(hit_record.normal, eta)
        };

        let color = self.color_sampler.sample(hit_record.uv, scene);
        (Some(Ray::new(hit_record.point + direction * 1e-5, direction)), color)
    }
}

// Schlick's approximation.
fn reflectance(cosine: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Debug)]
pub struct Emissive {
    pub color: Color
}

impl Scatterable for Emissive {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        (None, self.color)
    }
}
