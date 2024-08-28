// ====================================
// ===== Quaternion f32
// ====================================

//NOTE:  Everything Untested !!!

use crate::Vtx3f32;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Qtxf32 {
    pub s: f32,
    pub u: Vtx3f32,
}
impl Qtxf32 {
    pub fn new() -> Qtxf32 {
        Qtxf32 {
            s: 0.0,
            u: Vtx3f32 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    pub fn from_axis_angle(axis: Vtx3f32, angle: f32) -> Qtxf32 {
        Qtxf32 {
            s: (angle / 2.0).cos(),
            u: Vtx3f32 {
                x: (angle / 2.0).sin() * axis.x,
                y: (angle / 2.0).sin() * axis.y,
                z: (angle / 2.0).sin() * axis.z,
            },
        }
    }
    pub fn put_axis(&self, axis: Vtx3f32) -> Qtxf32 {
        Qtxf32 {
            s: self.s,
            u: Vtx3f32 {
                x: (self.s / 2.0).sin() * axis.x,
                y: (self.s / 2.0).sin() * axis.y,
                z: (self.s / 2.0).sin() * axis.z,
            },
        }
    }
    pub fn put_angle(&self, angle: f32) -> Qtxf32 {
        Qtxf32 {
            s: (angle / 2.0).cos(),
            u: self.u,
        }
    }

    // v = vector to be rotated; u = vector part of quaternion; s = scalar part of quatenion
    // rotated_vec = 2 * dot(uv) * u + (s * s - dot(uu) ) * v + 2 * s * cross(uv)

    pub fn apply(&self, v: Vtx3f32) -> Vtx3f32 {
        (self.u * (2.0 * self.u.dot(v)))
            + (((self.u.cross(v)) * (2.0 * self.s)) - (v * (self.u.dot(self.u))))
            + (self.s * self.s)
    }
}
