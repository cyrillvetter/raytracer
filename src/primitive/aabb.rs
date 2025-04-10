use crate::primitive::Ray;

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3
}

impl Aabb {
    pub const MAX: Self = Aabb::new(Vec3::INFINITY, Vec3::NEG_INFINITY);

    pub const fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let mut t1 = (self.minimum.x - ray.origin.x) * ray.dir_inv.x;
        let mut t2 = (self.maximum.x - ray.origin.x) * ray.dir_inv.x;

        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);

        for i in 1..3 {
            t1 = (self.minimum[i] - ray.origin[i]) * ray.dir_inv[i];
            t2 = (self.maximum[i] - ray.origin[i]) * ray.dir_inv[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax >= tmin.max(0.0)
    }

    pub fn grow(&mut self, v: Vec3) {
        self.minimum = self.minimum.min(v);
        self.maximum = self.maximum.max(v);
    }

    pub fn area(&self) -> f32 {
        let extent = self.maximum - self.minimum; // Box extent.
        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }
}
