use crate::primitive::Color;

use glam::Vec3;

#[derive(Debug, Clone)]
pub enum Material {
    Solid(Color)
}

impl Material {
    pub fn color_at(&self, _hit: Vec3) -> Color {
        match self {
            Self::Solid(color) => *color,
        }
    }
}
