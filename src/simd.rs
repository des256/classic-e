// E - SIMD support
// Desmond Germans, 2020

// This is a very naive and minimal implementation of SIMD access.
// By forcing the data to go through arrays, hopes are that the compiler can
// auto-vectorize this better.

// Refactor when the packed_simd debate in the Rust community is over and
// stable Rust supports proper SIMD access.

use {
    crate::*,
    std::fmt::{
        Display,
        Debug,
        Formatter,
        Result
    },
};

macro_rules! impl_simd {
    ([$t:ty; $n:expr]: $i:ident | $s:tt | $($xs:ident),*) => {
        #[derive(Copy,Clone)]
        #[allow(non_camel_case_types)]
        pub struct $i {
            v: [$t; $n],
        }

        impl $i {
            pub fn new($($xs: $t),*) -> $i {
                $i { v: [$($xs),*], }
            }

            pub fn splat(v: $t) -> $i {
                $i { v: [v; $n], }
            }

            pub fn set(&mut self,n: usize,v: $t) {
                self.v[n] = v;
            }

            pub fn get(&self,n: usize) -> $t {
                self.v[n]
            }

            pub fn eq(&self,other: &Self,mask: u64) -> bool {
                for i in 0..$n {
                    if (mask & (1 << i)) != 0 {
                        if self.v[i] != other.v[i] {
                            return false;
                        }
                    }
                }
                true
            }

            pub fn add(a: &$i,b: &$i) -> $i {
                let mut v: [$t; $n] = a.v;
                for i in 0..$n {
                    v[i] += b.v[i];
                }
                $i { v: v, }
            }

            pub fn sub(a: &$i,b: &$i) -> $i {
                let mut v: [$t; $n] = a.v;
                for i in 0..$n {
                    v[i] -= b.v[i];
                }
                $i { v: v, }
            }

            pub fn mul(a: &$i,b: &$i) -> $i {
                let mut v: [$t; $n] = a.v;
                for i in 0..$n {
                    v[i] *= b.v[i];
                }
                $i { v: v, }
            }

            pub fn div(a: &$i,b: &$i) -> $i {
                let mut v: [$t; $n] = a.v;
                for i in 0..$n {
                    v[i] /= b.v[i];
                }
                $i { v: v, }
            }
        }

        impl Display for $i {
            fn fmt(&self,f: &mut Formatter) -> Result {

                let mut result = String::from(format!("simd_{}x{}({}",$s,$n,self.v[0]));
                for i in 1..$n {
                    result.push_str(&format!(",{}",self.v[i]));
                }
                result.push_str(")");
                write!(f,"{}",result)
            }
        }

        impl Debug for $i {
            fn fmt(&self,f: &mut Formatter) -> Result {

                let mut result = String::from(format!("simd_{}x{}({}",$s,$n,self.v[0]));
                for i in 1..$n {
                    result.push_str(&format!(",{}",self.v[i]));
                }
                result.push_str(")");
                write!(f,"{}",result)
            }
        }

        impl Zero for $i {
            fn zero() -> $i {
                $i { v: [<$t>::zero(); $n], }
            }
        }
    }
}

// Instantiations

// for peephole optimization, implement the specific SIMD intrinsic access where needed

// 16 bits
impl_simd!([u8; 2]: simd_u8x2 | "u8" | x0,x1);
impl_simd!([i8; 2]: simd_i8x2 | "i8" | x0,x1);

// 32 bits
impl_simd!([u8; 4]: simd_u8x4 | "u8" | x0,x1,x2,x3);
impl_simd!([i8; 4]: simd_i8x4 | "i8" | x0,x1,x2,x3);
impl_simd!([u16; 2]: simd_u16x2 | "u16" | x0,x1);
impl_simd!([i16; 2]: simd_i16x2 | "i16" | x0,x1);

// 64 bits (MMX)
impl_simd!([u8; 8]: simd_u8x8 | "u8" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([i8; 8]: simd_i8x8 | "i8" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([u16; 4]: simd_u16x4 | "u16" | x0,x1,x2,x3);
impl_simd!([i16; 4]: simd_i16x4 | "i16" | x0,x1,x2,x3);
impl_simd!([u32; 2]: simd_u32x2 | "u32" | x0,x1);
impl_simd!([i32; 2]: simd_i32x2 | "i32" | x0,x1);
impl_simd!([f32; 2]: simd_f32x2 | "f32" | x0,x1);
#[cfg(target_pointer_width="32")]
impl_simd!([usize; 2]: simd_usizex2 | "usize" | x0,x1);
#[cfg(target_pointer_width="32")]
impl_simd!([isize; 2]: simd_isizex2 | "isize" | x0,x1);

// 128 bits (SSE)
impl_simd!([u8; 16]: simd_u8x16 | "u8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([i8; 16]: simd_i8x16 | "i8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([u16; 8]: simd_u16x8 | "u16" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([i16; 8]: simd_i16x8 | "i16" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([u32; 4]: simd_u32x4 | "u32" | x0,x1,x2,x3);
impl_simd!([i32; 4]: simd_i32x4 | "i32" | x0,x1,x2,x3);
impl_simd!([u64; 2]: simd_u64x2 | "u64" | x0,x1);
impl_simd!([i64; 2]: simd_i64x2 | "i64" | x0,x1);
impl_simd!([f32; 4]: simd_f32x4 | "f32" | x0,x1,x2,x3);
impl_simd!([f64; 2]: simd_f64x2 | "f64" | x0,x1);
#[cfg(target_pointer_width="32")]
impl_simd!([usize; 4]: simd_usizex4 | "usize" | x0,x1,x2,x3);
#[cfg(target_pointer_width="32")]
impl_simd!([isize; 4]: simd_isizex4 | "isize" | x0,x1,x2,x3);
#[cfg(target_pointer_width="64")]
impl_simd!([usize; 2]: simd_usizex2 | "usize" | x0,x1);
#[cfg(target_pointer_width="64")]
impl_simd!([isize; 2]: simd_isizex2 | "isize" | x0,x1);

// 256 bits (AVX)
impl_simd!([u8; 32]: simd_u8x32 | "u8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31);
impl_simd!([i8; 32]: simd_i8x32 | "i8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31);
impl_simd!([u16;16]: simd_u16x16 | "u16" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([i16;16]: simd_i16x16 | "i16" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([u32; 8]: simd_u32x8 | "u32" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([i32; 8]: simd_i32x8 | "i32" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([u64; 4]: simd_u64x4 | "u64" | x0,x1,x2,x3);
impl_simd!([i64; 4]: simd_i64x4 | "i64" | x0,x1,x2,x3);
impl_simd!([f32; 8]: simd_f32x8 | "f32" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([f64; 4]: simd_f64x4 | "f64" | x0,x1,x2,x3);
#[cfg(target_pointer_width="32")]
impl_simd!([usize; 8]: simd_usizex8 | "usize" | x0,x1,x2,x3,x4,x5,x6,x7);
#[cfg(target_pointer_width="32")]
impl_simd!([isize; 8]: simd_isizex8 | "isize" | x0,x1,x2,x3,x4,x5,x6,x7);
#[cfg(target_pointer_width="64")]
impl_simd!([usize; 4]: simd_usizex4 | "usize" | x0,x1,x2,x3);
#[cfg(target_pointer_width="64")]
impl_simd!([isize; 4]: simd_isizex4 | "isize" | x0,x1,x2,x3);

// 512 bits (AVX-512)
impl_simd!([u8; 64]: simd_u8x64 | "u8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32,x33,x34,x35,x36,x37,x38,x39,x40,x41,x42,x43,x44,x45,x46,x47,x48,x49,x50,x51,x52,x53,x54,x55,x56,x57,x58,x59,x60,x61,x62,x63);
impl_simd!([i8; 64]: simd_i8x64 | "i8" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32,x33,x34,x35,x36,x37,x38,x39,x40,x41,x42,x43,x44,x45,x46,x47,x48,x49,x50,x51,x52,x53,x54,x55,x56,x57,x58,x59,x60,x61,x62,x63);
impl_simd!([u16;32]: simd_u16x32 | "u16" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31);
impl_simd!([i16;32]: simd_i16x32 | "i16" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31);
impl_simd!([u32; 16]: simd_u32x16 | "u32" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([i32; 16]: simd_i32x16 | "i32" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([u64; 8]: simd_u64x8 | "u64" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([i64; 8]: simd_i64x8 | "i64" | x0,x1,x2,x3,x4,x5,x6,x7);
impl_simd!([f32; 16]: simd_f32x16 | "f32" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([f64; 8]: simd_f64x8 | "f64" | x0,x1,x2,x3,x4,x5,x6,x7);
#[cfg(target_pointer_width="32")]
impl_simd!([usize; 16]: simd_usizex16 | "usize" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
#[cfg(target_pointer_width="32")]
impl_simd!([isize; 16]: simd_isizex16 | "isize" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
#[cfg(target_pointer_width="64")]
impl_simd!([usize; 8]: simd_usizex8 | "usize" | x0,x1,x2,x3,x4,x5,x6,x7);
#[cfg(target_pointer_width="64")]
impl_simd!([isize; 8]: simd_isizex8 | "isize" | x0,x1,x2,x3,x4,x5,x6,x7);

// 1024 bits for specific cases
impl_simd!([u64; 16]: simd_u64x16 | "u64" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([i64; 16]: simd_i64x16 | "i64" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
impl_simd!([f64; 16]: simd_f64x16 | "f64" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
#[cfg(target_pointer_width="64")]
impl_simd!([usize; 16]: simd_usizex16 | "usize" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);
#[cfg(target_pointer_width="64")]
impl_simd!([isize; 16]: simd_isizex16 | "isize" | x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15);

// traits to allow for generic implementations of higher structures, like Vec2<T>, Rect<T>, etc.
pub trait Simd2 { type Type: Copy + Clone + Debug; type T; const N: usize; }
pub trait Simd4 { type Type: Copy + Clone + Debug; type T; const N: usize; }
pub trait Simd8 { type Type: Copy + Clone + Debug; type T; const N: usize; }
pub trait Simd16 { type Type: Copy + Clone + Debug; type T; const N: usize; }

impl Simd2 for u8 { type Type = simd_u8x2; type T = u8; const N: usize = 2; }
impl Simd2 for i8 { type Type = simd_i8x2; type T = i8; const N: usize = 2; }
impl Simd2 for u16 { type Type = simd_u16x2; type T = u16; const N: usize = 2; }
impl Simd2 for i16 { type Type = simd_i16x2; type T = i16; const N: usize = 2; }
impl Simd2 for u32 { type Type = simd_u32x2; type T = u32; const N: usize = 2; }
impl Simd2 for i32 { type Type = simd_i32x2; type T = i32; const N: usize = 2; }
impl Simd2 for u64 { type Type = simd_u64x2; type T = u64; const N: usize = 2; }
impl Simd2 for i64 { type Type = simd_i64x2; type T = i64; const N: usize = 2; }
impl Simd2 for usize { type Type = simd_usizex2; type T = usize; const N: usize = 2; }
impl Simd2 for isize { type Type = simd_isizex2; type T = isize; const N: usize = 2; }
impl Simd2 for f32 { type Type = simd_f32x2; type T = f32; const N: usize = 2; }
impl Simd2 for f64 { type Type = simd_f64x2; type T = f64; const N: usize = 2; }

impl Simd4 for u8 { type Type = simd_u8x4; type T = u8; const N: usize = 4; }
impl Simd4 for i8 { type Type = simd_i8x4; type T = i8; const N: usize = 4; }
impl Simd4 for u16 { type Type = simd_u16x4; type T = u16; const N: usize = 4; }
impl Simd4 for i16 { type Type = simd_i16x4; type T = i16; const N: usize = 4; }
impl Simd4 for u32 { type Type = simd_u32x4; type T = u32; const N: usize = 4; }
impl Simd4 for i32 { type Type = simd_i32x4; type T = i32; const N: usize = 4; }
impl Simd4 for u64 { type Type = simd_u64x4; type T = u64; const N: usize = 4; }
impl Simd4 for i64 { type Type = simd_i64x4; type T = i64; const N: usize = 4; }
impl Simd4 for usize { type Type = simd_usizex4; type T = usize; const N: usize = 4; }
impl Simd4 for isize { type Type = simd_isizex4; type T = isize; const N: usize = 4; }
impl Simd4 for f32 { type Type = simd_f32x4; type T = f32; const N: usize = 4; }
impl Simd4 for f64 { type Type = simd_f64x4; type T = f64; const N: usize = 4; }

impl Simd8 for u8 { type Type = simd_u8x8; type T = u8; const N: usize = 8; }
impl Simd8 for i8 { type Type = simd_i8x8; type T = i8; const N: usize = 8; }
impl Simd8 for u16 { type Type = simd_u16x8; type T = u16; const N: usize = 8; }
impl Simd8 for i16 { type Type = simd_i16x8; type T = i16; const N: usize = 8; }
impl Simd8 for u32 { type Type = simd_u32x8; type T = u32; const N: usize = 8; }
impl Simd8 for i32 { type Type = simd_i32x8; type T = i32; const N: usize = 8; }
impl Simd8 for u64 { type Type = simd_u64x8; type T = u64; const N: usize = 8; }
impl Simd8 for i64 { type Type = simd_i64x8; type T = i64; const N: usize = 8; }
impl Simd8 for usize { type Type = simd_usizex8; type T = usize; const N: usize = 8; }
impl Simd8 for isize { type Type = simd_isizex8; type T = isize; const N: usize = 8; }
impl Simd8 for f32 { type Type = simd_f32x8; type T = f32; const N: usize = 8; }
impl Simd8 for f64 { type Type = simd_f64x8; type T = f64; const N: usize = 8; }

impl Simd16 for u8 { type Type = simd_u8x16; type T = u8; const N: usize = 16; }
impl Simd16 for i8 { type Type = simd_i8x16; type T = i8; const N: usize = 16; }
impl Simd16 for u16 { type Type = simd_u16x16; type T = u16; const N: usize = 16; }
impl Simd16 for i16 { type Type = simd_i16x16; type T = i16; const N: usize = 16; }
impl Simd16 for u32 { type Type = simd_u32x16; type T = u32; const N: usize = 16; }
impl Simd16 for i32 { type Type = simd_i32x16; type T = i32; const N: usize = 16; }
impl Simd16 for u64 { type Type = simd_u64x16; type T = u64; const N: usize = 16; }
impl Simd16 for i64 { type Type = simd_i64x16; type T = i64; const N: usize = 16; }
impl Simd16 for usize { type Type = simd_usizex16; type T = usize; const N: usize = 16; }
impl Simd16 for isize { type Type = simd_isizex16; type T = isize; const N: usize = 16; }
impl Simd16 for f32 { type Type = simd_f32x16; type T = f32; const N: usize = 16; }
impl Simd16 for f64 { type Type = simd_f64x16; type T = f64; const N: usize = 16; }
