use core::ops::*;
use std::fmt;

use glam::Vec3;

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

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub const fn gray(v: f32) -> Self {
        Self { r: v, g: v, b: v }
    }

    pub fn gray_u8(v: u8) -> Self {
        let v = (v as f32) / 255.0;
        Self { r: v, g: v, b: v }
    }

    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        const DENOM: f32 = 255.0f32.recip();

        Self {
            r: (r as f32) * DENOM,
            g: (g as f32) * DENOM,
            b: (b as f32) * DENOM
        }
    }

    pub fn into_u32(self) -> u32 {
        ((self.r * 255.0) as u32) << 16 | ((self.g * 255.0) as u32) << 8 | ((self.b * 255.0) as u32)
    }

    pub fn gamma_correct(self) -> Self {
        let convert = |c: f32| {
            if c <= 0.0031308 {
                return 12.92 * c;
            } else {
                return 1.055 * (c.powf(1.0 / 2.4)) - 0.055;
            }
        };

        Self {
            r: convert(self.r),
            g: convert(self.g),
            b: convert(self.b),
        }
    }

    pub fn gamma_uncorrect(self) -> Self {
        let convert = |c: f32| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        Self {
            r: convert(self.r),
            g: convert(self.g),
            b: convert(self.b),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::rgb(
            v.x.clamp(0.0, 1.0),
            v.y.clamp(0.0, 1.0),
            v.z.clamp(0.0, 1.0)
        )
    }
}

impl From<[f32; 4]> for Color {
    fn from(a: [f32; 4]) -> Self {
        Color::rgb(a[0], a[1], a[2])
    }
}

impl From<[f32; 3]> for Color {
    fn from(a: [f32; 3]) -> Self {
        Color::rgb(a[0], a[1], a[2])
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R: {:.2}, G: {:.2}, B: {:.2}", self.r, self.g, self.b)
    }
}

impl Div<Color> for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            r: self.r.div(rhs.r),
            g: self.g.div(rhs.g),
            b: self.b.div(rhs.b)
        }
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.r.div_assign(rhs.r);
        self.g.div_assign(rhs.g);
        self.b.div_assign(rhs.b);
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            r: self.r.div(rhs),
            g: self.g.div(rhs),
            b: self.b.div(rhs)
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r.div_assign(rhs);
        self.g.div_assign(rhs);
        self.b.div_assign(rhs);
    }
}

impl Div<Color> for f32 {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color {
            r: self.div(rhs.r),
            g: self.div(rhs.g),
            b: self.div(rhs.b)
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

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r.mul_assign(rhs.r);
        self.g.mul_assign(rhs.g);
        self.b.mul_assign(rhs.b);
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r.mul(rhs),
            g: self.g.mul(rhs),
            b: self.b.mul(rhs)
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r.mul_assign(rhs);
        self.g.mul_assign(rhs);
        self.b.mul_assign(rhs);
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.mul(rhs.r),
            g: self.mul(rhs.g),
            b: self.mul(rhs.b)
        }
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

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r.add_assign(rhs.r);
        self.g.add_assign(rhs.g);
        self.b.add_assign(rhs.b);
    }
}

impl Add<f32> for Color {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            r: self.r.add(rhs),
            g: self.g.add(rhs),
            b: self.b.add(rhs)
        }
    }
}

impl AddAssign<f32> for Color {

    fn add_assign(&mut self, rhs: f32) {
        self.r.add_assign(rhs);
        self.g.add_assign(rhs);
        self.b.add_assign(rhs);
    }
}

impl Add<Color> for f32 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.add(rhs.r),
            g: self.add(rhs.g),
            b: self.add(rhs.b)
        }
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r.sub(rhs.r),
            g: self.g.sub(rhs.g),
            b: self.b.sub(rhs.b)
        }
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r.sub_assign(rhs.r);
        self.g.sub_assign(rhs.g);
        self.b.sub_assign(rhs.b);
    }
}

impl Sub<f32> for Color {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            r: self.r.sub(rhs),
            g: self.g.sub(rhs),
            b: self.b.sub(rhs)
        }
    }
}

impl SubAssign<f32> for Color {
    fn sub_assign(&mut self, rhs: f32) {
        self.r.sub_assign(rhs);
        self.g.sub_assign(rhs);
        self.b.sub_assign(rhs);
    }
}

impl Sub<Color> for f32 {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.sub(rhs.r),
            g: self.sub(rhs.g),
            b: self.sub(rhs.b)
        }
    }
}
