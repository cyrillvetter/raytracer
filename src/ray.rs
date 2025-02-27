use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}
