use crate::primitive::*;

use glam::{Vec3A, Vec2};

#[derive(Debug)]
pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    pub centroid: Vec3A,
    pub material_index: Option<usize>,
}

#[derive(Debug)]
pub struct Vertex {
    pub position: Vec3A,
    pub normal: Vec3A,
    pub uv: Vec2,
}

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3A,
    pub normal: Vec3A,
    pub uv: Vec2,
    pub front_face: bool,
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

    pub fn create_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let point = ray.at(t);
        let barycentric = self.get_barycentric_coordinates(point);

        let uv = self.v1.uv * barycentric.x + self.v2.uv * barycentric.y + self.v3.uv * barycentric.z;
        let mut normal = (self.v1.normal * barycentric.x + self.v2.normal * barycentric.y + self.v3.normal * barycentric.z).normalize();

        let mut front_face = true;

        // Hits back face.
        if ray.direction.dot(normal) > 0.0 {
            normal = -normal;
            front_face = false;
        }

        HitRecord {
            t,
            point,
            normal,
            front_face,
            uv,
            material_index: self.material_index
        }
    }

    fn get_barycentric_coordinates(&self, p: Vec3A) -> Vec3A {
        let v1v2 = self.v2.position - self.v1.position;
        let v1v3 = self.v3.position - self.v1.position;
        let v0p = p - self.v1.position;

        let d11 = v1v2.dot(v1v2);
        let d12 = v1v2.dot(v1v3);
        let d22 = v1v3.dot(v1v3);
        let d31 = v0p.dot(v1v2);
        let d32 = v0p.dot(v1v3);

        let denom = d11 * d22 - d12 * d12;
        let v = (d22 * d31 - d12 * d32) / denom;
        let w = (d11 * d32 - d12 * d31) / denom;
        let u = 1.0 - v - w;

        Vec3A::new(u, v, w)
    }
}

impl Vertex {
    pub const fn new(position: Vec3A, normal: Vec3A, uv: Vec2) -> Self {
        Self { position, normal, uv }
    }
}
