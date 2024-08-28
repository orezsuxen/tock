// ====================================
// ===== Matrix 2d f64
// ====================================

use crate::Vtx2f64;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mtx2f64 {
    pub ix: f64,
    pub iy: f64,

    pub jx: f64,
    pub jy: f64,
}

impl Mtx2f64 {
    pub fn new() -> Mtx2f64 {
        Mtx2f64 {
            ix: 0.0,
            jx: 0.0,
            iy: 0.0,
            jy: 0.0,
        }
    }
    pub fn build(ix: f64, iy: f64, jx: f64, jy: f64) -> Mtx2f64 {
        Mtx2f64 { ix, iy, jx, jy }
    }
    pub fn ident() -> Mtx2f64 {
        Mtx2f64 {
            ix: 1.0,
            jx: 0.0,
            iy: 0.0,
            jy: 1.0,
        }
    }
    pub fn rot_90() -> Mtx2f64 {
        Mtx2f64 {
            ix: (0.0),
            jx: (1.0),
            iy: (-1.0),
            jy: (0.0),
        }
    }
    pub fn rot_180() -> Mtx2f64 {
        Mtx2f64 {
            ix: (-1.0),
            jx: (0.0),
            iy: (0.0),
            jy: (-1.0),
        }
    }
    pub fn rot_270() -> Mtx2f64 {
        Mtx2f64 {
            ix: (0.0),
            jx: (-1.0),
            iy: (1.0),
            jy: (0.0),
        }
    }
    pub fn from_rot(rad: f64) -> Mtx2f64 {
        Mtx2f64 {
            ix: (rad.cos()),
            jx: (-(rad.sin())),
            iy: (rad.sin()),
            jy: (rad.cos()),
        }
    }

    fn i(&self) -> Vtx2f64 {
        Vtx2f64 {
            x: self.ix,
            y: self.iy,
        }
    }
    fn j(&self) -> Vtx2f64{
        Vtx2f64{
            x: self.jx,
            y: self.jy,
        }
    }
    fn x(&self) -> Vtx2f64{
        Vtx2f64{
            x: self.ix,
            y: self.jx,
        }
    }
    fn y(&self) -> Vtx2f64{
        Vtx2f64{
            x: self.iy,
            y: self.jy,
        }
    }
    fn set_i(&self, v: Vtx2f64) -> Mtx2f64 {
        Mtx2f64 { ix: v.x, iy :v.y, jx: self.jx, jy: self.jy }
    }
    fn set_j(&self, v: Vtx2f64) -> Mtx2f64 {
        Mtx2f64 { ix: self.ix, iy: self.iy, jx: v.x, jy: v.y }
    }
    fn set_x(&self, v: Vtx2f64) -> Mtx2f64 {
        Mtx2f64 { ix: v.x, iy: self.iy, jx: v.x, jy: self.jy }
    }
    fn set_y(&self, v: Vtx2f64) -> Mtx2f64 {
        Mtx2f64 { ix: self.ix, iy: v.y, jx: self.jx, jy: v.y }
    }
    fn det(&self) -> f64 {
        (self.ix * self.jy) - (self.iy * self.jx)
    }
    fn inverse(&self) -> Option<Mtx2f64> {
        let det = self.det();
        if det == 0.0 {
            return None;
        } else {
            Some(Mtx2f64 {
                ix: self.jy / det,
                jx: -self.jx / det,
                iy: -self.iy / det,
                jy: self.ix / det,
            })
        }
    }
    fn combine(&self, rhs: Mtx2f64) -> Mtx2f64 {
        Mtx2f64 {
            ix: (self.ix * rhs.ix) + (self.jx * rhs.iy),
            jx: (self.ix * rhs.jx) + (self.jx * rhs.jy),
            iy: (self.iy * rhs.ix) * (self.jy * rhs.iy),
            jy: (self.iy * rhs.jx) * (self.jy * rhs.jy),
        }
    }
    fn apply(self, rhs: Vtx2f64) -> Vtx2f64 {
        Vtx2f64 {
            x: (self.ix * rhs.x) + (self.jx * rhs.y),
            y: (self.iy * rhs.x) + (self.jy * rhs.y),
        }
    }




}



