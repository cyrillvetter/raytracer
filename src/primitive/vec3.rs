use core::{f32, ops::*};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self * self.length().recip()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X: {:.4}, Y: {:.4}, Z: {:.4}", self.x, self.y, self.z)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z)
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs)
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z)
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z)
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs)
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z)
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z)
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs)
        }
    }
}

impl AddAssign<f32> for Vec3 {

    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z)
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z)
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs)
        }
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z)
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}
