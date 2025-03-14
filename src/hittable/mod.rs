pub mod sphere;
pub use sphere::Sphere;

pub mod triangle;
pub use triangle::Triangle;

use crate::primitive::*;
use crate::Light;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color;
}
