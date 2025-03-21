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
        let t = (self.v1 - ray.origin).dot(self.normal) / ray.direction.dot(self.normal);
        let p = ray.at(t);

        let v1 = (self.v2 - p).cross(self.v2 - self.v1);
        let v2 = (self.v3 - p).cross(self.v3 - self.v2);
        let v3 = (self.v1 - p).cross(self.v1 - self.v3);

        if v1.z.signum() == v2.z.signum() && v2.z.signum() == v3.z.signum() {
            Some(HitRecord::new(t, p, self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
