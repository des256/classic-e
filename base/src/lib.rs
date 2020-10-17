// E
// Desmond Germans, 2020

//! # E
//!
//! It's E. E for everything.

use {
    std::{
        fmt::{
            Display,
            Debug,
        },
        ops::{
            Add,
            Sub,
            Mul,
            Div,
            AddAssign,
            SubAssign,
            MulAssign,
            DivAssign,
            Neg,
        },
        cmp::{
            PartialEq,
            PartialOrd,
        },
    },
};

/// System error result.
#[derive(Copy,Clone,Debug)]
pub enum SystemError {
    /// (temporary) Generic error.
    Generic,
}

/// Additive identity.
pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }

/// Multiplicative identity.
pub trait One {
    fn one() -> Self;
}

impl One for u8 { fn one() -> Self { 1 } }
impl One for i8 { fn one() -> Self { 1 } }
impl One for u16 { fn one() -> Self { 1 } }
impl One for i16 { fn one() -> Self { 1 } }
impl One for u32 { fn one() -> Self { 1 } }
impl One for i32 { fn one() -> Self { 1 } }
impl One for u64 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }
impl One for usize { fn one() -> Self { 1 } }
impl One for isize { fn one() -> Self { 1 } }

#[doc(hidden)]
pub trait Number: Sized + Copy + Clone + Zero + One + Display + Debug + PartialEq + PartialOrd + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + AddAssign + SubAssign + MulAssign + DivAssign { }

#[doc(hidden)]
pub trait SignedNumber: Number + Neg<Output=Self> { }

#[doc(hidden)]
pub trait FloatNumber: SignedNumber { }

impl Number for u8 { }
impl Number for i8 { }
impl Number for u16 { }
impl Number for i16 { }
impl Number for u32 { }
impl Number for i32 { }
impl Number for u64 { }
impl Number for i64 { }
impl Number for f32 { }
impl Number for f64 { }
impl Number for usize { }
impl Number for isize { }

impl SignedNumber for i8 { }
impl SignedNumber for i16 { }
impl SignedNumber for i32 { }
impl SignedNumber for i64 { }
impl SignedNumber for f32 { }
impl SignedNumber for f64 { }
impl SignedNumber for isize { }

impl FloatNumber for f32 { }
impl FloatNumber for f64 { }

/// (MAYBE) Trait for anything that needs a color specification.
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
        let r = ((self.x * 255.0) as u32) << 16;
        let g = ((self.y * 255.0) as u32) << 8;
        let b = (self.z * 255.0) as u32;
        let a = ((self.w * 255.0) as u32) << 24;
        a | r | g | b
    }

    fn as_vec4(&self) -> Vec4<f32> {
        *self
    }
}

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

mod mat2x2;
pub use mat2x2::*;

mod mat3x3;
pub use mat3x3::*;

mod mat4x4;
pub use mat4x4::*;

mod multivec2;
pub use multivec2::*;

mod multivec3;
pub use multivec3::*;

mod multivec4;
pub use multivec4::*;

mod matrix;
pub use matrix::*;

pub mod pixel;
