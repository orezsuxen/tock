// ====================================
// ===== Matrix 2d f32
// ====================================

use crate::Vtx2f32;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mtx2f32 {
    pub ix: f32,
    pub iy: f32,

    pub jx: f32,
    pub jy: f32,
}

impl Mtx2f32 {
    pub fn new() -> Mtx2f32 {
        Mtx2f32 {
            ix: 0.0,
            jx: 0.0,
            iy: 0.0,
            jy: 0.0,
        }
    }
    pub fn build(ix: f32, iy: f32, jx: f32, jy: f32) -> Mtx2f32 {
        Mtx2f32 { ix, iy, jx, jy }
    }
    pub fn ident() -> Mtx2f32 {
        Mtx2f32 {
            ix: 1.0,
            jx: 0.0,
            iy: 0.0,
            jy: 1.0,
        }
    }
    pub fn rot_90() -> Mtx2f32 {
        Mtx2f32 {
            ix: (0.0),
            jx: (1.0),
            iy: (-1.0),
            jy: (0.0),
        }
    }
    pub fn rot_180() -> Mtx2f32 {
        Mtx2f32 {
            ix: (-1.0),
            jx: (0.0),
            iy: (0.0),
            jy: (-1.0),
        }
    }
    pub fn rot_270() -> Mtx2f32 {
        Mtx2f32 {
            ix: (0.0),
            jx: (-1.0),
            iy: (1.0),
            jy: (0.0),
        }
    }
    pub fn from_rot(rad: f32) -> Mtx2f32 {
        Mtx2f32 {
            ix: (rad.cos()),
            jx: (-(rad.sin())),
            iy: (rad.sin()),
            jy: (rad.cos()),
        }
    }

    fn i(&self) -> Vtx2f32 {
        Vtx2f32 {
            x: self.ix,
            y: self.iy,
        }
    }
    fn j(&self) -> Vtx2f32{
        Vtx2f32{
            x: self.jx,
            y: self.jy,
        }
    }
    fn x(&self) -> Vtx2f32{
        Vtx2f32{
            x: self.ix,
            y: self.jx,
        }
    }
    fn y(&self) -> Vtx2f32{
        Vtx2f32{
            x: self.iy,
            y: self.jy,
        }
    }
    fn put_i(&self, v: Vtx2f32) -> Mtx2f32 {
        Mtx2f32 { ix: v.x, iy :v.y, jx: self.jx, jy: self.jy }
    }
    fn put_j(&self, v: Vtx2f32) -> Mtx2f32 {
        Mtx2f32 { ix: self.ix, iy: self.iy, jx: v.x, jy: v.y }
    }
    fn put_x(&self, v: Vtx2f32) -> Mtx2f32 {
        Mtx2f32 { ix: v.x, iy: self.iy, jx: v.x, jy: self.jy }
    }
    fn put_y(&self, v: Vtx2f32) -> Mtx2f32 {
        Mtx2f32 { ix: self.ix, iy: v.y, jx: self.jx, jy: v.y }
    }
    fn det(&self) -> f32 {
        (self.ix * self.jy) - (self.iy * self.jx)
    }
    fn inverse(&self) -> Option<Mtx2f32> {
        let det = self.det();
        if det == 0.0 {
            return None;
        } else {
            Some(Mtx2f32 {
                ix: self.jy / det,
                jx: -self.jx / det,
                iy: -self.iy / det,
                jy: self.ix / det,
            })
        }
    }
    fn combine(&self, rhs: Mtx2f32) -> Mtx2f32 {
        Mtx2f32 {
            ix: (self.ix * rhs.ix) + (self.jx * rhs.iy),
            jx: (self.ix * rhs.jx) + (self.jx * rhs.jy),
            iy: (self.iy * rhs.ix) * (self.jy * rhs.iy),
            jy: (self.iy * rhs.jx) * (self.jy * rhs.jy),
        }
    }
    fn apply(self, rhs: Vtx2f32) -> Vtx2f32 {
        Vtx2f32 {
            x: (self.ix * rhs.x) + (self.jx * rhs.y),
            y: (self.iy * rhs.x) + (self.jy * rhs.y),
        }
    }

}



