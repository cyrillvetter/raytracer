use crate::primitive::{Color, Ray};
use crate::triangle::HitRecord;
use crate::scene::Scene;

use glam::Vec3;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color);
}

#[derive(Debug, Clone)]
pub enum Material {
    Phong(Phong),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        match self {
            Material::Phong(phong) => phong.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phong {
    pub color: Color,
    pub shininess: f32,
}

impl Scatterable for Phong {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub color: Color
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        todo!()
    }
}
