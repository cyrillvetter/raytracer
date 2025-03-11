use crate::vec3::Vec3;
use crate::color::Color;

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
