use glam::Vec3A;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub dir_inv: Vec3A
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction, dir_inv: direction.recip() }
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}
