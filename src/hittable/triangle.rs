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

        let v0 = self.v2 - self.v1;
        let v1 = self.v3 - self.v1;
        let v2 = q - self.v1;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);

        let discriminant = d00 * d11 - d01 * d01;
        if discriminant > -f32::EPSILON && discriminant < f32::EPSILON {
            return None;
        }

        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let mu = (d11 * d20 - d01 * d21) / discriminant;
        let tau = (d00 * d21 - d01 * d20) / discriminant;
        if mu >= 0.0 && mu <= 1.0 && tau >= 0.0 && tau <= 1.0 && (mu + tau) <= 1.0 {
            Some(HitRecord::new(lambda, q, self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
