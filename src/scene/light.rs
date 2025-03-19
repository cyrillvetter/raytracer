use crate::primitive::*;

#[derive(Debug, Clone)]
pub struct Light {
    pub origin: Vec3,
    pub color: Color,
    pub intensity: f32
}

impl Light {
    pub const fn new(origin: Vec3, color: Color, intensity: f32) -> Self {
        Self { origin, color, intensity }
    }
}
