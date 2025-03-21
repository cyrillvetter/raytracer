pub mod sphere;
pub use sphere::Sphere;

pub mod mesh;
pub use mesh::Mesh;

pub mod triangle;
pub use triangle::Triangle;

use crate::primitive::*;

use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub q: Vec3,
    pub normal: Vec3,
    pub color: Color,
}

impl HitRecord {
    pub const fn new(t: f32, q: Vec3, normal: Vec3, color: Color) -> Self {
        Self { t, q, normal, color }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
