use crate::math::Vec3;
use crate::Color;
use crate::math::Ray;
use crate::Light;

use super::Hittable;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    normal: Vec3
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self { v1, v2, v3, normal: v2.cross(v3) }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        Some((self.v1 - ray.origin).dot(self.normal) / ray.direction.dot(self.normal))
    }

    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color {
        Color::WHITE
    }
}
