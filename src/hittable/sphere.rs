use crate::primitive::*;
use super::{Hittable, HitRecord};

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Self { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let v = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * (ray.direction.dot(v));
        let c = v.length_squared() - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            let hit = ray.at(t);
            let normal = (hit - self.center).normalize();
            Some(HitRecord::new(t, hit, normal, self.color))
        }
    }
}
