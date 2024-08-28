// ====================================
// ===== Matrix 3d f64
// ====================================

use crate::Vtx3f64;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mtx3f64 {
    pub ix: f64,
    pub iy: f64,
    pub iz: f64,

    pub jx: f64,
    pub jy: f64,
    pub jz: f64,

    pub kx: f64,
    pub ky: f64,
    pub kz: f64,
}

impl Mtx3f64 {
    pub fn new() -> Mtx3f64 {
        Mtx3f64 {
            ix: 0.0,
            jx: 0.0,
            kx: 0.0,
            iy: 0.0,
            jy: 0.0,
            ky: 0.0,
            iz: 0.0,
            jz: 0.0,
            kz: 0.0,
        }
    }
    pub fn ident() -> Mtx3f64 {
        Mtx3f64 {
            ix: 1.0,
            jx: 0.0,
            kx: 0.0,
            iy: 0.0,
            jy: 1.0,
            ky: 0.0,
            iz: 0.0,
            jz: 0.0,
            kz: 1.0,
        }
    }

    pub fn from_rot_x(rad: f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: 1.0,
            jx: 0.0,
            kx: 0.0,
            iy: 0.0,
            jy: rad.cos(),
            ky: -(rad.sin()),
            iz: 0.0,
            jz: rad.sin(),
            kz: rad.cos(),
        }
    }
    pub fn from_rot_y(rad: f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: rad.cos(),
            jx: 0.0,
            kx: rad.sin(),
            iy: 0.0,
            jy: 1.0,
            ky: 0.0,
            iz: -(rad.sin()),
            jz: 0.0,
            kz: rad.cos(),
        }
    }
    pub fn from_rot_z(rad: f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: rad.cos(),
            jx: -(rad.sin()),
            kx: 0.0,
            iy: rad.sin(),
            jy: rad.cos(),
            ky: 0.0,
            iz: 0.0,
            jz: 0.0,
            kz: 1.0,
        }
    }
    fn i(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.ix,
            y: self.iy,
            z: self.iz,
        }
    }
    fn j(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.jx,
            y: self.jy,
            z: self.jz,
        }
    }
    fn k(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.kx,
            y: self.ky,
            z: self.kz,
        }
    }
    fn x(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.ix,
            y: self.jx,
            z: self.kx,
        }
    }
    fn y(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.iy,
            y: self.jy,
            z: self.ky,
        }
    }
    fn z(&self) -> Vtx3f64 {
        Vtx3f64 {
            x: self.iz,
            y: self.jz,
            z: self.kz,
        }
    }
    pub fn put_i(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: v.x,
            iy: v.y,
            iz: v.z,
            ..*self
        }
    }
    pub fn put_j(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            jx: v.x,
            jy: v.y,
            jz: v.z,
            ..*self
        }
    }
    pub fn put_k(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            kx: v.x,
            ky: v.y,
            kz: v.z,
            ..*self
        }
    }
    pub fn put_x(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: v.x,
            jx: v.y,
            kx: v.z,
            ..*self
        }
    }
    pub fn put_y(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            iy: v.x,
            jy: v.y,
            ky: v.z,
            ..*self
        }
    }
    pub fn put_z(&self, v: Vtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            iz: v.x,
            jz: v.y,
            kz: v.z,
            ..*self
        }
    }
    fn det(&self) -> f64 {
        (self.ix * self.jy * self.kz)
            + (self.jx * self.ky * self.iz)
            + (self.kx * self.iy * self.jz)
            - (self.iz * self.jy * self.kx)
            - (self.jz * self.ky * self.ix)
            - (self.kz * self.iy * self.jx)
    }
    fn inverse(&self) -> Option<Mtx3f64> {
        let det = self.det();
        if det == 0.0 {
            return None;
        } else {
            Some(Mtx3f64 {
                ix: ((self.jy * self.kz) - (self.ky * self.jz)) / det,
                jx: ((self.kx * self.jz) - (self.jx * self.kz)) / det,
                kx: ((self.jx * self.ky) - (self.kx * self.jy)) / det,

                iy: ((self.ky * self.iz) - (self.iy * self.kz)) / det,
                jy: ((self.ix * self.kz) - (self.kx * self.iz)) / det,
                ky: ((self.kx * self.iy) - (self.ix * self.ky)) / det,

                iz: ((self.iy * self.jz) - (self.jy * self.iz)) / det,
                jz: ((self.jx * self.iz) - (self.ix * self.jz)) / det,
                kz: ((self.ix * self.jy) - (self.jx * self.iy)) / det,
            })
        }
    }
    fn combine(self, m: Mtx3f64) -> Mtx3f64 {
        Mtx3f64 {
            ix: (self.ix * m.ix) + (self.jx * m.iy) + (self.kx * m.iz),
            iy: (self.iy * m.ix) + (self.jy * m.iy) + (self.ky * m.iz),
            iz: (self.iz * m.ix) + (self.jz * m.iy) + (self.kz * m.iz),

            jx: (self.ix * m.jx) + (self.jx * m.jy) + (self.kx * m.jz),
            jy: (self.iy * m.jx) + (self.jy * m.jy) + (self.ky * m.jz),
            jz: (self.iz * m.jx) + (self.jz * m.jy) + (self.kz * m.jz),

            kx: (self.ix * m.kx) + (self.jx * m.ky) + (self.kx * m.kz),
            ky: (self.iy * m.kx) + (self.jy * m.ky) + (self.ky * m.kz),
            kz: (self.iz * m.kx) + (self.jz * m.ky) + (self.kz * m.kz),
        }
    }
    fn apply(self, m: Vtx3f64) -> Vtx3f64 {
        Vtx3f64 {
            x: (self.ix * m.x) + (self.jx * m.y) + (self.kx * m.z),
            y: (self.iy * m.x) + (self.jy * m.y) + (self.ky * m.z),
            z: (self.iz * m.x) + (self.jz * m.y) + (self.kz * m.z),
        }
    }
}
