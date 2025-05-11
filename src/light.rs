use crate::primitive::Color;

use glam::Vec3A;

#[derive(Debug, Clone)]
pub struct Light {
    pub origin: Vec3A,
    pub color: Color,
    pub intensity: f32
}

impl Light {
    pub const fn new(origin: Vec3A, color: Color, intensity: f32) -> Self {
        Self { origin, color, intensity }
    }
}
