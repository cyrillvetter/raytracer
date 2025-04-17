use crate::primitive::{Color, Ray};
use crate::triangle::HitRecord;
use crate::scene::Scene;

use glam::Vec3A;
use fastrand::f32;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color, f32);
}

#[derive(Debug, Clone)]
pub enum Material {
    Phong(Phong),
    Metal(Metal),
    Glass(Glass),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color, f32) {
        match self {
            Material::Phong(phong) => phong.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
            Material::Glass(glass) => glass.scatter(ray, hit_record, scene),
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
