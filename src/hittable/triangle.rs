use crate::primitive::*;
use crate::scene::Scene;
use super::{Hittable, HitRecord};

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub normal: Vec3
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, normal: Vec3) -> Self {
        Self { v1, v2, v3, normal }
    }
}

impl Hittable for Triangle {
    // Barycentric coordinates intersection.
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let denominator = ray.direction.dot(self.normal);
        if denominator > -f32::EPSILON && denominator < f32::EPSILON {
            return None;
        }

        let lambda = (self.v1 - ray.origin).dot(self.normal) / denominator;
        if lambda <= 0.0 {
            return None;
        }

        let q = ray.at(lambda);

        let area = |p1: Vec3, p2: Vec3, p3: Vec3| {
            0.5 * (p2 - p1).cross(p3 - p1).length()
        };

        let triangle_area = area(self.v1, self.v2, self.v3);
        let lambda1 = area(q, self.v2, self.v3) / triangle_area;
        let lambda2 = area(q, self.v3, self.v1) / triangle_area;
        let lambda3 = area(q, self.v1, self.v2) / triangle_area;

        let r = lambda1 + lambda2 + lambda3 - 1.0;
        if r < f32::EPSILON && r > -f32::EPSILON {
            Some(HitRecord::new(lambda, q, self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
