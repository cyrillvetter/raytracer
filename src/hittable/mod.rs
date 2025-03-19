pub mod sphere;
pub use sphere::Sphere;

pub mod mesh;
pub use mesh::Mesh;

pub mod triangle;
pub use triangle::Triangle;

use crate::primitive::*;
use crate::scene::Scene;

const AMBIENT_FACTOR: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub hit: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub const fn new(t: f32, hit: Vec3, normal: Vec3) -> Self {
        Self { t, hit, normal }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
    fn get_color(&self, r: HitRecord, scene: &Scene) -> Color;
}
