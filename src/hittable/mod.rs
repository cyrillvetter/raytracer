pub mod sphere;

use crate::ray::Ray;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::light::Light;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color;
}
