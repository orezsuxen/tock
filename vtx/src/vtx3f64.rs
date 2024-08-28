// ====================================
// ===== Vector 3d f64
// ====================================

use std::ops::{Add, Div, Mul, Sub};
use crate::Vtx2f64;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vtx3f64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vtx3f64 {
    pub fn new() -> Vtx3f64 {
        Vtx3f64 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn build(x: f64, y: f64, z: f64) -> Vtx3f64 {
        Vtx3f64 { x, y, z }
    }
    pub fn dot(&self, v: Vtx3f64) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
    pub fn cross(&self, v: Vtx3f64) -> Vtx3f64 {
        Vtx3f64 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
    pub fn dist(&self, v: Vtx3f64) -> f64 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2)).sqrt()
    }
    pub fn magn(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn unit(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.x / self.magn(),
            y: self.y / self.magn(),
            z: self.z / self.magn(),
        }
    }
    pub fn xy(&self) -> Vtx2f64 {
        Vtx2f64 { x: self.x, y: self.y }
    }
    pub fn xz(&self) -> Vtx2f64 {
        Vtx2f64 { x: self.x, y: self.z }
    }
    pub fn yz(&self) -> Vtx2f64 {
        Vtx2f64 { x: self.y, y: self.z }
    }
}
// ========== std::ops ==========
// Addition
impl Add<Vtx3f64> for Vtx3f64 {
    type Output = Self;
    fn add(self, rhs: Vtx3f64) -> Self {
        Vtx3f64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<f64> for Vtx3f64 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Vtx3f64 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
//Subtraction
impl Sub<Vtx3f64> for Vtx3f64 {
    type Output = Self;
    fn sub(self, rhs: Vtx3f64) -> Self {
        Vtx3f64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<f64> for Vtx3f64 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Vtx3f64 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
// Multiplication
impl Mul<Vtx3f64> for Vtx3f64 {
    type Output = Self;
    fn mul(self, rhs: Vtx3f64) -> Self {
        Vtx3f64 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Vtx3f64 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vtx3f64 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
// Division
impl Div<Vtx3f64> for Vtx3f64 {
    type Output = Self;
    fn div(self, rhs: Vtx3f64) -> Self {
        Vtx3f64 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<f64> for Vtx3f64 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vtx3f64 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
// ========== From ===========
impl From<(f64, f64, f64)> for Vtx3f64 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vtx3f64 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl From<Vtx3f64> for (f64, f64, f64) {
    fn from(value: Vtx3f64) -> Self {
        (value.x, value.y, value.z)
    }
}
