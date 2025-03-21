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
    // Inverse matrix intersection.
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

        let b_p = q - self.v2;
        let c_p = q - self.v3;
        let a_p = q - self.v1;

        let b_a = self.v1 - self.v2;
        let c_b = self.v2 - self.v3;
        let a_c = self.v3 - self.v1;

        let c1 = b_p.cross(b_a);
        let c2 = c_p.cross(c_b);
        let c3 = a_p.cross(a_c);

        if c1.z.signum() == c2.z.signum() && c2.z.signum() == c3.z.signum() {
            Some(HitRecord::new(lambda, q, self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
