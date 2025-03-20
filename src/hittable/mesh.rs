use crate::primitive::*;
use crate::scene::Scene;
use super::{Hittable, HitRecord, Triangle, AMBIENT_FACTOR};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub color: Color,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, color: Color) -> Self {
        Self { triangles, color }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut nearest = f32::INFINITY;
        let mut nearest_record: Option<HitRecord> = None;

        for triangle in self.triangles.iter() {
            match triangle.hit(ray) {
                Some(r) if r.t < nearest => {
                    nearest = r.t;
                    nearest_record = Some(r);
                },
                _ => ()
            }
        }

        nearest_record
    }

    fn get_color(&self, r: HitRecord, scene: &Scene) -> Color {
        let mut color = self.color * AMBIENT_FACTOR;

        let l1 = &scene.lights[0];
        let light_dir = l1.origin - r.hit;
        let shadow_ray = Ray::new(r.hit + light_dir * f32::EPSILON, light_dir);
        let mut in_shadow = false;

        for object in scene.objects.iter() {
            if object.hit(&shadow_ray).is_some() {
                in_shadow = true;
                break;
            }
        }

        if in_shadow {
            return (color).clamp();
        }

        for light in scene.lights.iter() {
            // Diffuse.
            let s = (light.origin - r.hit).normalize();
            let diffuse = self.color * s.dot(r.normal).max(0.0) * light.color * light.intensity;
            color += diffuse;
        }

        color.clamp()
    }
}
