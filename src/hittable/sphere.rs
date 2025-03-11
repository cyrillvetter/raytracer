use crate::vec3::Vec3;
use crate::color::Color;
use crate::ray::Ray;
use crate::light::Light;

use super::{Hittable, AMBIENT_FACTOR};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Self { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let v = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * (ray.direction.dot(v));
        let c = v.length_squared() - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }

    fn get_color(&self, q: Vec3, lights: &[Light]) -> Color {
        let mut color = self.color * AMBIENT_FACTOR;
        let n = (q - self.center).normalize();

        for light in lights {
            let s = (light.origin - q).normalize();
            let diffuse = self.color * s.dot(n).max(0.0) * light.color * light.intensity;
            color += diffuse
        }

        color.clamp()
    }
}
