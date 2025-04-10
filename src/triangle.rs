use crate::primitive::*;

use glam::Vec3A;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    pub centroid: Vec3A,
    pub material_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3A,
    pub normal: Vec3A,
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3A,
    pub normal: Vec3A,
    pub material_index: Option<usize>,
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex, material_index: Option<usize>) -> Self {
        let centroid = (v1.position + v2.position + v3.position) * 0.3333333;
        Self {
            v1,
            v2,
            v3,
            centroid,
            material_index
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
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
            Some(HitRecord {
                t,
                point: ray.at(t),
                normal: self.v1.normal,
                material_index: self.material_index
            })
        } else {
            None
        }
    }
}

impl Vertex {
    pub const fn new(position: Vec3A, normal: Vec3A) -> Self {
        Self { position, normal }
    }
}
