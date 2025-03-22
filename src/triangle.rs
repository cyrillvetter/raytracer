use crate::primitive::*;

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    pub material_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

impl Triangle {
    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let e1 = self.v2.position - self.v1.position;
        let e2 = self.v3.position - self.v1.position;

        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);
        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let inv_det = det.recip();
        let s = ray.origin - self.v1.position;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray.direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot(s_cross_e1);
        if t > f32::EPSILON {
            Some(t)
        } else {
            None
        }
    }
}

impl Vertex {
    pub const fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}
