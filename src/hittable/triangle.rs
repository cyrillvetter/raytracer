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
        todo!()
    }

    fn get_color(&self, _r: HitRecord, _scene: &Scene) -> Color {
        unimplemented!()
    }
}
