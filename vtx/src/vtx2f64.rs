// ====================================
// ===== Vector 2d f64
// ====================================

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vtx2f64 {
    pub x: f64,
    pub y: f64,
}

impl Vtx2f64 {
    pub fn new() -> Vtx2f64 {
        Vtx2f64 { x: 0.0, y: 0.0 }
    }
    pub fn build(x: f64, y: f64) -> Vtx2f64 {
        Vtx2f64 { x, y }
    }
    pub fn from_rot(rot: f64) -> Vtx2f64 {
        Vtx2f64 {
            x: rot.cos(),
            y: rot.sin(),
        }
    }

    pub fn dot(&self, v: Vtx2f64) -> f64 {
        (self.x * v.x) + (self.y * v.y)
    }
    pub fn cross(&self, v: Vtx2f64) -> f64 {
        (self.x * v.y) - (self.y * v.x)
    }
    pub fn dist(&self, v: Vtx2f64) -> f64 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }
    pub fn magn(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn unit(&self) -> Vtx2f64 {
        Vtx2f64 {
            x: self.x / self.magn(),
            y: self.y / self.magn(),
        }
    }
    pub fn as_rot(&self) -> f64 {
        f64::atan2(self.unit().y, self.unit().x)
    }
}
// ========== std::ops ==========
// Addition
impl Add<Vtx2f64> for Vtx2f64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<f64> for Vtx2f64 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
// Subtraction
impl Sub<Vtx2f64> for Vtx2f64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<f64> for Vtx2f64 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
// Multiplication
impl Mul<Vtx2f64> for Vtx2f64 {
    type Output = Self;
    fn mul(self, rhs: Vtx2f64) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<f64> for Vtx2f64 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
// Division
impl Div<Vtx2f64> for Vtx2f64 {
    type Output = Self;
    fn div(self, rhs: Vtx2f64) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Div<f64> for Vtx2f64 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
// ========== From ==========
impl From<(f64, f64)> for Vtx2f64 {
    fn from(value: (f64, f64)) -> Self {
        Vtx2f64 {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<Vtx2f64> for (f64, f64) {
    fn from(value: Vtx2f64) -> Self {
        (value.x, value.y)
    }
}
