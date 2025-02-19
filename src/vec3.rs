use core::{f32, ops::*};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);

    pub const ONE: Self = Self::splat(1.0);

    pub const NEG_ONE: Self = Self::splat(-1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y
        }
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    pub fn normalize(self) -> f32 {
        todo!()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

// TODO: Implement Div, Mul, Add, Sub.
