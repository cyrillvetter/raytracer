use crate::primitive::Ray;

use glam::Vec3A;

#[derive(Debug)]
pub struct Aabb {
    pub minimum: Vec3A,
    pub maximum: Vec3A
}

impl Aabb {
    pub const MAX: Self = Aabb::new(Vec3A::INFINITY, Vec3A::NEG_INFINITY);

    pub const fn new(minimum: Vec3A, maximum: Vec3A) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let mut t1 = (self.minimum.x - ray.origin.x) * ray.dir_inv.x;
        let mut t2 = (self.maximum.x - ray.origin.x) * ray.dir_inv.x;

        let mut tmin = fast_f32_min(t1, t2);
        let mut tmax = fast_f32_max(t1, t2);

        for i in 1..3 {
            t1 = (self.minimum[i] - ray.origin[i]) * ray.dir_inv[i];
            t2 = (self.maximum[i] - ray.origin[i]) * ray.dir_inv[i];

            tmin = fast_f32_max(tmin, fast_f32_min(t1, t2));
            tmax = fast_f32_min(tmax, fast_f32_max(t1, t2));
        }

        (tmax >= fast_f32_max(tmin, 0.0)).then_some(tmin)
    }

    pub fn grow(&mut self, v: Vec3A) {
        self.minimum = self.minimum.min(v);
        self.maximum = self.maximum.max(v);
    }

    pub fn area(&self) -> f32 {
        let extent = self.maximum - self.minimum;
        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }
}

fn fast_f32_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn fast_f32_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
