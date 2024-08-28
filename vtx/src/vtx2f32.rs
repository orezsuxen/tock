// ====================================
// ===== Vector 2d f32
// ====================================

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vtx2f32 {
    pub x: f32,
    pub y: f32,
}

impl Vtx2f32 {
    pub fn new() -> Vtx2f32 {
        Vtx2f32 { x: 0.0, y: 0.0 }
    }
    pub fn build(x: f32, y: f32) -> Vtx2f32 {
        Vtx2f32 { x, y }
    }
    pub fn from_rot(rot: f32) -> Vtx2f32 {
        Vtx2f32 {
            x: rot.cos(),
            y: rot.sin(),
        }
    }

    pub fn dot(&self, v: Vtx2f32) -> f32 {
        (self.x * v.x) + (self.y * v.y)
    }
    pub fn cross(&self, v: Vtx2f32) -> f32 {
        (self.x * v.y) - (self.y * v.x)
    }
    pub fn dist(&self, v: Vtx2f32) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }
    pub fn magn(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn unit(&self) -> Vtx2f32 {
        Vtx2f32 {
            x: self.x / self.magn(),
            y: self.y / self.magn(),
        }
    }
    pub fn as_rot(&self) -> f32 {
        f32::atan2(self.unit().y, self.unit().x)
    }
}
// ========== std::ops ==========
// Addition
impl Add<Vtx2f32> for Vtx2f32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<f32> for Vtx2f32 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
// Subtraction
impl Sub<Vtx2f32> for Vtx2f32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<f32> for Vtx2f32 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
// Multiplication
impl Mul<Vtx2f32> for Vtx2f32 {
    type Output = Self;
    fn mul(self, rhs: Vtx2f32) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<f32> for Vtx2f32 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
// Division
impl Div<Vtx2f32> for Vtx2f32 {
    type Output = Self;
    fn div(self, rhs: Vtx2f32) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Div<f32> for Vtx2f32 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
// ========== From ==========
impl From<(f32, f32)> for Vtx2f32 {
    fn from(value: (f32, f32)) -> Self {
        Vtx2f32 {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<Vtx2f32> for (f32, f32) {
    fn from(value: Vtx2f32) -> Self {
        (value.x, value.y)
    }
}
