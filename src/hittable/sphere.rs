use crate::primitive::*;
use crate::scene::Scene;

use super::{Hittable, HitRecord, AMBIENT_FACTOR};

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
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let v = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * (ray.direction.dot(v));
        let c = v.length_squared() - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            let hit = ray.at(t);
            let normal = (hit - self.center).normalize();
            Some(HitRecord::new(t, hit, normal))
        }
    }

    fn get_color(&self, r: HitRecord, scene: &Scene) -> Color {
        let mut color = self.color * AMBIENT_FACTOR;

        for light in scene.lights.iter() {
            let s = (light.origin - r.hit).normalize();
            let diffuse = self.color * s.dot(r.normal).max(0.0) * light.color * light.intensity;
            color += diffuse
        }

        color.clamp()
    }
}
