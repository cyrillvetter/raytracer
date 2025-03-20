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
        let t = ((self.v1 - ray.origin).dot(self.normal)) / ray.direction.dot(self.normal);
        let p = ray.at(t);
        let calc_area = |v1: Vec3, v2: Vec3| {
            0.5 * (v1.cross(v2)).length()
        };

        let triangle_area = calc_area(self.v1, self.v2);

        let t1 = calc_area(p, self.v2) / triangle_area;
        let t2 = calc_area(p, self.v3) / triangle_area;
        let t3 = calc_area(p, self.v1) / triangle_area;

        if t1 >= 0.0 && t2 >= 0.0 && t3 >= 0.0 && (t1 + t2 + t3) == 1.0 {
            Some(HitRecord::new(t, p, self.normal))
        } else {
            None
        }
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
