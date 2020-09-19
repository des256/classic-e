// E
// Desmond Germans, 2020

//! # E
//!
//! It's E. E for everything.

mod zeroone;
pub use zeroone::*;

/// Trait for anything that needs a color specification.
pub trait ColorParameter {

    /// Convert into u32.
    fn into_u32(self) -> u32;
    fn into_vec4(self) -> f32x4;
}

impl ColorParameter for u32 {

    fn into_u32(self) -> u32 {
        self
    }

    fn into_vec4(self) -> f32x4 {
        let r = (((self >> 16) & 0xFF) as f32) / 255.0;
        let g = (((self >> 8) & 0xFF) as f32) / 255.0;
        let b = ((self & 0xFF) as f32) / 255.0;
        let a = (((self >> 24) & 0xFF) as f32) / 255.0;
        f32x4::from_xyzw(r,g,b,a)
    }
}

impl ColorParameter for f32x4 {

    fn into_u32(self) -> u32 {
        let r = ((self.x * 255.0) as u32) << 16;
        let g = ((self.y * 255.0) as u32) << 8;
        let b = (self.z * 255.0) as u32;
        let a = ((self.w * 255.0) as u32) << 24;
        a | r | g | b
    }

    fn into_vec4(self) -> f32x4 {
        self
    }
}

mod mat;
pub use mat::*;

mod ten;
pub use ten::*;

mod vec2;
pub use vec2::*;

mod vec3;
pub use vec3::*;

mod vec4;
pub use vec4::*;

mod matrix;
pub use matrix::*;

#[macro_use]
mod multivector;
pub use multivector::*;

#[macro_use]
mod rect;
pub use rect::*;

mod platform;
pub use platform::*;

pub mod pixel;

pub mod image;

pub mod gpu;

pub mod ui;
