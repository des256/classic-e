// E - SIMD support
// Desmond Germans, 2020

// This is a very naive and minimal implementation of SIMD access.
// By forcing the data to go through arrays, hopes are that the compiler can
// auto-vectorize this better.

// Refactor when the packed_simd debate in the Rust community is over and
// stable Rust supports proper SIMD access.

use {
    std::{
        fmt::{
            Display,
            Debug,
            Formatter,
            Result
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
        },
        cmp::{
            PartialEq,
            PartialOrd,
        },
    },
};

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

pub trait Simdable: Sized + Copy + Clone + Zero + One + Display + Debug + PartialEq + PartialOrd + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + AddAssign + SubAssign + MulAssign + DivAssign { }

impl Simdable for u8 { }
impl Simdable for i8 { }
impl Simdable for u16 { }
impl Simdable for i16 { }
impl Simdable for u32 { }
impl Simdable for i32 { }
impl Simdable for u64 { }
impl Simdable for i64 { }
impl Simdable for f32 { }
impl Simdable for f64 { }
impl Simdable for usize { }
impl Simdable for isize { }

pub trait SimdableFloat: Simdable { }

impl SimdableFloat for f32 { }
impl SimdableFloat for f64 { }

macro_rules! impl_simd {
    ($name:ident,$n:expr,$s:tt) => {
        #[derive(Copy,Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name<T>([T; $n]);
        impl<T: Simdable> $name<T> {
            pub fn new(v: [T; $n]) -> $name<T> {
                $name(v)
            }

            pub fn splat(p: T) -> $name<T> {
                $name([p; $n])
            }

            pub fn set(&mut self,n: usize,p: T) {
                self.0[n] = p;
            }

            pub fn get(&self,n: usize) -> T {
                self.0[n]
            }

            pub fn eq(&self,other: &Self,mask: u64) -> bool {
                for i in 0..$n {
                    if (mask & (1 << i)) != 0 {
                        if self.0[i] != other.0[i] {
                            return false;
                        }
                    }
                }
                true
            }

            pub fn add(a: Self,b: Self) -> Self {
                let mut v: [T; $n] = a.0;
                for i in 0..$n {
                    v[i] += b.0[i];
                }
                $name(v)
            }

            pub fn sub(a: Self,b: Self) -> Self {
                let mut v: [T; $n] = a.0;
                for i in 0..$n {
                    v[i] -= b.0[i];
                }
                $name(v)
            }

            pub fn mul(a: Self,b: Self) -> Self {
                let mut v: [T; $n] = a.0;
                for i in 0..$n {
                    v[i] *= b.0[i];
                }
                $name(v)
            }

            pub fn div(a: Self,b: Self) -> Self {
                let mut v: [T; $n] = a.0;
                for i in 0..$n {
                    v[i] /= b.0[i];
                }
                $name(v)
            }

            pub fn dot(a: Self,b: Self,mask: u64) -> T {
                let mut v: T = a.0[0] * b.0[0];
                for i in 1..$n {
                    if (mask & (1 << i)) != 0 {
                        v += a.0[i] * b.0[i];
                    }
                }
                v
            }
        }

        impl<T: Simdable> Debug for $name<T> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                let mut result = String::from(format!("{}({}",$s,self.0[0]));
                for i in 1..$n {
                    result.push_str(&format!(",{}",self.0[i]));
                }
                result.push_str(")");
                write!(f,"{}",result)
            }
        }

        impl<T: Simdable> Zero for $name<T> {
            fn zero() -> $name<T> {
                $name([T::zero(); $n])
            }
        }
    }
}

impl_simd!(Simd2,2,"Simd2");
impl_simd!(Simd4,4,"Simd4");
impl_simd!(Simd8,8,"Simd8");
impl_simd!(Simd16,16,"Simd16");
impl_simd!(Simd32,32,"Simd32");
impl_simd!(Simd64,64,"Simd64");

/*// Instantiations

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
*/