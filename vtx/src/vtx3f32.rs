// ====================================
// ===== Vector 3d f32
// ====================================

use std::ops::{Add, Div, Mul, Sub};
use crate::Vtx2f32;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vtx3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vtx3f32 {
    pub fn new() -> Vtx3f32 {
        Vtx3f32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn build(x: f32, y: f32, z: f32) -> Vtx3f32 {
        Vtx3f32 { x, y, z }
    }
    pub fn dot(&self, v: Vtx3f32) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }
    pub fn cross(&self, v: Vtx3f32) -> Vtx3f32 {
        Vtx3f32 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
    pub fn dist(&self, v: Vtx3f32) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2)).sqrt()
    }
    pub fn magn(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn unit(&self) -> Vtx3f32 {
        Vtx3f32 {
            x: self.x / self.magn(),
            y: self.y / self.magn(),
            z: self.z / self.magn(),
        }
    }
    pub fn xy(&self) -> Vtx2f32 {
        Vtx2f32 { x: self.x, y: self.y }
    }
    pub fn xz(&self) -> Vtx2f32 {
        Vtx2f32 { x: self.x, y: self.z }
    }
    pub fn yz(&self) -> Vtx2f32 {
        Vtx2f32 { x: self.y, y: self.z }
    }
}
// ========== std::ops ==========
// Addition
impl Add<Vtx3f32> for Vtx3f32 {
    type Output = Self;
    fn add(self, rhs: Vtx3f32) -> Self {
        Vtx3f32 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<f32> for Vtx3f32 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Vtx3f32 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
//Subtraction
impl Sub<Vtx3f32> for Vtx3f32 {
    type Output = Self;
    fn sub(self, rhs: Vtx3f32) -> Self {
        Vtx3f32 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<f32> for Vtx3f32 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Vtx3f32 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
// Multiplication
impl Mul<Vtx3f32> for Vtx3f32 {
    type Output = Self;
    fn mul(self, rhs: Vtx3f32) -> Self {
        Vtx3f32 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f32> for Vtx3f32 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vtx3f32 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
// Division
impl Div<Vtx3f32> for Vtx3f32 {
    type Output = Self;
    fn div(self, rhs: Vtx3f32) -> Self {
        Vtx3f32 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<f32> for Vtx3f32 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Vtx3f32 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
// ========== From ===========
impl From<(f32, f32, f32)> for Vtx3f32 {
    fn from(value: (f32, f32, f32)) -> Self {
        Vtx3f32 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl From<Vtx3f32> for (f32, f32, f32) {
    fn from(value: Vtx3f32) -> Self {
        (value.x, value.y, value.z)
    }
}
