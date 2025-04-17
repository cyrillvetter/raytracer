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
    Glass(Glass),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        match self {
            Material::Phong(phong) => phong.scatter(ray, hit_record, scene),
            Material::Metal(metal) => metal.scatter(ray, hit_record, scene),
            Material::Glass(glass) => glass.scatter(ray, hit_record, scene),
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
            let light_vec = light.origin - hit_record.point;
            let light_distance = light_vec.length();
            let light_dir = light_vec / light_distance;
            let light_intensity = light.intensity / (scene.lights.len() as f32);

            let shadow_ray = Ray::new(hit_record.point + light_dir * 1e-5, light_dir);

            // TODO: Maybe add another function that finds intersections between the surface and a lightsource.
            let in_shadow = scene.bvh.intersects(&shadow_ray).map_or(f32::INFINITY, |h| h.t) < light_distance;

            if !in_shadow {
                let s = (light.origin - hit_record.point).normalize();
                let diffuse = self.color *
                    s.dot(hit_record.normal).max(0.0) *
                    light_distance.powf(2.0).recip() *
                    light.color *
                    light_intensity;
                let specular = light.color *
                    (1.0 - self.roughness) *
                    reflection_dir.dot(light_dir).powf((1.0 - self.roughness) * 128.0).max(0.0);

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

#[derive(Debug, Clone)]
pub struct Glass {
    pub color: Color
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        let eta = if hit_record.front_face { 1.5f32.recip() } else { 1.5f32 };
        let refraction_dir = ray.direction.refract(hit_record.normal, eta);
        (Some(Ray::new(hit_record.point + refraction_dir * 1e-5, refraction_dir)), self.color)
    }
}
