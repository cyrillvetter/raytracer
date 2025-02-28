use crate::vec3::Vec3;
use crate::color::Color;
use crate::ray::Ray;

use super::Hittable;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    pub const fn new(origin: Vec3, radius: f32, color: Color) -> Self {
        Self { origin, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let v = self.origin - ray.origin;
        let a = ray.direction.length_squared();
        let b = 2.0 * (ray.direction.dot(v));
        let c = v.length_squared() - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}
