use crate::{
    primitive::{Color, Ray},
    triangle::HitRecord,
    Sampler,
    Scene
};

const AMBIENT_FACTOR: f32 = 0.05;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color);
}

#[derive(Debug)]
pub enum Material {
    Phong(Phong),
    Metal(Metal),
    Glass(Glass),
    Emissive(Emissive),
}

impl Material {
    pub fn get_color_texture_index(&self) -> Option<usize> {
        use Material::*;
        match self {
            Phong(phong) => phong.color_sampler.texture_index(),
            Metal(metal) => metal.color_sampler.texture_index(),
            Glass(glass) => glass.color_sampler.texture_index(),
            _ => None
        }
    }
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        use Material::*;
        match self {
            Phong(phong) => phong.scatter(ray, hit_record, scene),
            Metal(metal) => metal.scatter(ray, hit_record, scene),
            Glass(glass) => glass.scatter(ray, hit_record, scene),
            Emissive(emissive) => emissive.scatter(ray, hit_record, scene),
        }
    }
}

#[derive(Debug)]
pub struct Phong {
    pub color_sampler: Sampler,
    pub roughness_sampler: Sampler
}

impl Scatterable for Phong {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        let base_color = self.color_sampler.sample(hit_record.uv, scene);
        let roughness = self.roughness_sampler.sample(hit_record.uv, scene).g;

        let mut color = base_color * AMBIENT_FACTOR;
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();

        for light in scene.lights.iter() {
            let light_vec = light.origin - hit_record.point;
            let light_distance = light_vec.length();
            let light_dir = light_vec / light_distance;
            let light_intensity = light.intensity / (scene.lights.len() as f32);

            let shadow_ray = Ray::new(hit_record.point + light_dir * 1e-3, light_dir);
            let in_shadow = scene.bvh.intersects(&shadow_ray).map_or(f32::INFINITY, |h| h.t) < light_distance;

            if !in_shadow {
                let s = (light.origin - hit_record.point).normalize();
                let diffuse = base_color *
                    s.dot(hit_record.normal).max(0.0) *
                    light_distance.recip() *
                    light.color *
                    light_intensity;
                let specular = light.color *
                    (1.0 - roughness) *
                    reflection_dir.dot(light_dir).powf((1.0 - roughness) * 128.0).max(0.0);

                color += diffuse + specular;
            }
        }

        (None, color.clamp())
    }
}

#[derive(Debug)]
pub struct Metal {
    pub color_sampler: Sampler
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        // Roughness values are samples from the G channel (https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#_material_pbrmetallicroughness_metallicroughnesstexture).
        let reflection_dir = ray.direction.reflect(hit_record.normal).normalize();
        let color = self.color_sampler.sample(hit_record.uv, scene);
        (Some(Ray::new(hit_record.point + reflection_dir * 1e-5, reflection_dir)), color)
    }
}

#[derive(Debug)]
pub struct Glass {
    pub color_sampler: Sampler
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, scene: &Scene) -> (Option<Ray>, Color) {
        const GLASS_IOR: f32 = 1.52;
        let eta = if hit_record.front_face { GLASS_IOR.recip() } else { GLASS_IOR };
        let refraction_dir = ray.direction.refract(hit_record.normal, eta);
        let color = self.color_sampler.sample(hit_record.uv, scene);
        (Some(Ray::new(hit_record.point + refraction_dir * 1e-5, refraction_dir)), color)
    }
}

#[derive(Debug)]
pub struct Emissive {
    pub color: Color
}

impl Scatterable for Emissive {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord, _scene: &Scene) -> (Option<Ray>, Color) {
        (None, self.color)
    }
}
