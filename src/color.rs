use core::ops::*;

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);

    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        let n = value.normalize();
        Color::rgb(n.x, n.y, n.z)
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r.add(rhs.r),
            g: self.g.add(rhs.g),
            b: self.b.add(rhs.b)
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r.mul(rhs.r),
            g: self.g.mul(rhs.g),
            b: self.b.mul(rhs.b)
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r.mul(rhs),
            g: self.g.mul(rhs),
            b: self.b.mul(rhs),
        }
    }
}
