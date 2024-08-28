// ====================================
// ===== VTX Dumb Vector and Matrix
// ====================================
#![allow(unused)]

mod vtx2f32;
mod vtx2f64;

mod vtx3f32;
mod vtx3f64;

mod mtx2f32;
mod mtx2f64;

mod mtx3f32;
mod mtx3f64;

mod qtxf32;
mod qtxf64;

pub use vtx2f32::Vtx2f32 as Vtx2f32;
pub use vtx2f64::Vtx2f64 as Vtx2f64;
pub use vtx2f64::Vtx2f64 as Vtx2;

pub use vtx3f32::Vtx3f32 as Vtx3f32;
pub use vtx3f64::Vtx3f64 as Vtx3f64;
pub use vtx3f64::Vtx3f64 as Vtx3;

pub use mtx2f32::Mtx2f32 as Mtx2f32;
pub use mtx2f64::Mtx2f64 as Mtx2f64;
pub use mtx2f64::Mtx2f64 as Mtx2;

pub use mtx3f32::Mtx3f32 as Mtx3f32;
pub use mtx3f64::Mtx3f64 as Mtx3f63;
pub use mtx3f64::Mtx3f64 as Mtx3;

pub use qtxf32::Qtxf32 as Qtxf32;
pub use qtxf64::Qtxf64 as Qtxf64;
pub use qtxf64::Qtxf64 as Qtx;


