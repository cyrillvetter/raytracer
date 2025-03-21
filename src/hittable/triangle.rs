use crate::primitive::*;
use crate::scene::Scene;
use super::{Hittable, HitRecord};

use glam::{Vec3, Mat3};

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
    // Equation system intersection.
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let v = self.v2 - self.v1;
        let w = self.v3- self.v1;

        let a = Mat3::from_cols(ray.direction, -v, -w);
        let tmp0 = a.x_axis.cross(a.y_axis);
        let det = a.z_axis.dot(tmp0);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let b = self.v1 - ray.origin;

        let tmp1 = a.y_axis.cross(a.z_axis);
        let tmp2 = a.z_axis.cross(a.x_axis);

        let inv_det = Vec3::splat(det.recip());
        let inv = Mat3::from_cols(tmp1 * inv_det, tmp2 * inv_det, tmp0 * inv_det).transpose();
        let r = inv * b;

        if r.x >= 0.0 && r.y >= 0.0 && r.z >= 0.0 && r.y + r.z <= 1.0 {
            Some(HitRecord::new(r.x, ray.at(r.x), self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
