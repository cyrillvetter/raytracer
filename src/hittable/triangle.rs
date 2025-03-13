use crate::math::Vec3;
use crate::Color;
use crate::math::Ray;
use crate::Light;

use super::{Hittable, AMBIENT_FACTOR};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub color: Color,
    normal: Vec3
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, color: Color) -> Self {
        Self { v1, v2, v3, color, normal: v2.cross(v3) }
    }
}

impl Hittable for Triangle {
    // Möller–Trumbore intersection algorithm.
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;

        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);
        if det > -f32::EPSILON && det < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v1;
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

    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color {
        let mut color = self.color * AMBIENT_FACTOR;

        for light in lights {
            let s = (light.origin - q).normalize();
            let diffuse = self.color * s.dot(self.normal).max(0.0) * light.color * light.intensity;
            color += diffuse;
        }

        color.clamp()
    }
}
