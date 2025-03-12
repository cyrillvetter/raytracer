pub mod sphere;
pub use sphere::Sphere;

use crate::math::Ray;
use crate::Color;
use crate::math::Vec3;
use crate::Light;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color;
}
