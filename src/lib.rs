// E
// Desmond Germans, 2020

//! # E
//!
//! It's E. E for everything.

/// Trait for anything that needs a color specification.
pub trait ColorParameter {

    /// Convert into u32.
    fn as_u32(&self) -> u32;
    fn as_vec4(&self) -> Vec4<f32>;
}

impl ColorParameter for u32 {

    fn as_u32(&self) -> u32 {
        *self
    }

    fn as_vec4(&self) -> Vec4<f32> {
        let r = (((self >> 16) & 0xFF) as f32) / 255.0;
        let g = (((self >> 8) & 0xFF) as f32) / 255.0;
        let b = ((self & 0xFF) as f32) / 255.0;
        let a = (((self >> 24) & 0xFF) as f32) / 255.0;
        vec4!(r,g,b,a)
    }
}

impl ColorParameter for Vec4<f32> {

    fn as_u32(&self) -> u32 {
        let r = ((self.x() * 255.0) as u32) << 16;
        let g = ((self.y() * 255.0) as u32) << 8;
        let b = (self.z() * 255.0) as u32;
        let a = ((self.w() * 255.0) as u32) << 24;
        a | r | g | b
    }

    fn as_vec4(&self) -> Vec4<f32> {
        *self
    }
}

mod platform;
pub use platform::*;

mod simd;
pub use simd::*;

mod mat;
pub use mat::*;

mod ten;
pub use ten::*;

mod complex;
pub use complex::*;

mod quat;
pub use quat::*;

mod rect;
pub use rect::*;

mod vec2;
pub use vec2::*;

mod vec3;
pub use vec3::*;

mod vec3a;
pub use vec3a::*;

mod vec4;
pub use vec4::*;

mod multivec2;
pub use multivec2::*;

mod multivec3;
pub use multivec3::*;

mod multivec4;
pub use multivec4::*;

mod matrix;
pub use matrix::*;

pub mod pixel;

pub mod image;

pub mod gpu;

pub mod ui;
