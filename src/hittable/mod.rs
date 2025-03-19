pub mod sphere;
pub use sphere::Sphere;

pub mod mesh;
pub use mesh::Mesh;

mod triangle;
use triangle::Triangle;

use crate::primitive::*;
use crate::scene::Scene;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self, q: Vec3, scene: &Scene) -> Color;
}
