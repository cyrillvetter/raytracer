use crate::vec3::*;
use crate::color::*;
use crate::ray::*;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    pub const fn new(origin: Vec3, radius: f32, color: Color) -> Self {
        Self { origin, radius, color }
    }

    pub fn hits(self, ray: Ray) -> Option<f32> {
        let v = self.origin - ray.origin;

        todo!()
    }
}
