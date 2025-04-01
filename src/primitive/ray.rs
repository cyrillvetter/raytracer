use glam::Vec3A;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray {
    pub const fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}
