use crate::primitive::{Color, Ray};
use crate::triangle::HitRecord;
use crate::scene::Scene;

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color);
}

#[derive(Debug, Clone)]
pub enum Material {
    Phong(Phong),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        match self {
            Material::Phong(phong) => phong.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phong {
    pub color: Color,
    pub roughness: f32,
}

impl Scatterable for Phong {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let mut color = self.color * AMBIENT_FACTOR;
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();

        for light in scene.lights.iter() {
            let light_dir = (light.origin - hit_record.point).normalize();
            let light_intensity = light.intensity / (scene.lights.len() as f32);
            let shadow_ray = Ray::new(hit_record.point + light_dir * 1e-4, light_dir);
            let mut in_shadow = false;

            for triangle in scene.triangles.iter() {
                if triangle.hit(&shadow_ray).is_some() {
                    in_shadow = true;
                }
            }

            if !in_shadow {
                let s = (light.origin - hit_record.point).normalize();
                let diffuse = self.color * s.dot(hit_record.normal).max(0.0) * light.color * light_intensity;
                let specular = light.color * (1.0 - self.roughness) * reflection_dir.dot(light_dir).powf((1.0 - self.roughness) * 128.0).max(0.0);

                color += diffuse + specular;
            }
        }

        (None, color.clamp())
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub color: Color
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();
        (Some(Ray::new(hit_record.point + reflection_dir * 1e-5, reflection_dir)), self.color)
    }
}
