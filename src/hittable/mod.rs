pub mod sphere;

use crate::Ray;
use crate::Color;
use crate::Vec3;
use crate::Light;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color;
}
