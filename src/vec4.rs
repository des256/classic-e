// E - Vector
// Desmond Germans, 2020

use crate::*;
use std::{
    cmp::PartialEq,
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
        Neg
    },
};

//#[cfg(target_arch="x86")]
//use core::arch::x86::*;
//#[cfg(target_arch="x86_64")]
//use core::arch::x86_64::*;

macro_rules! impl_vec4u (
    ($v:ident,$t:ty) => (

        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone)]
        pub struct $v {
            pub x: $t,
            pub y: $t,
            pub z: $t,
            pub w: $t,
        }

        impl $v {
            pub fn from_xyzw(x: $t,y: $t,z: $t,w: $t) -> $v {
                $v {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }

            pub fn zero() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }

            pub fn unit_x() -> $v {
                $v {
                    x: <$t>::one(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }

            pub fn unit_y() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::one(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }

            pub fn unit_z() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::one(),
                    w: <$t>::zero(),
                }
            }
        }

        impl PartialEq for $v {
            fn eq(&self,other: &$v) -> bool {
                (self.x == other.x) &&
                (self.y == other.y) &&
                (self.z == other.z) &&
                (self.w == other.w)
            }
        }

        impl Display for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl Debug for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }
        
        impl Zero for $v {
            fn zero() -> Self {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }
        }        

        impl Add<$v> for $v {
            type Output = $v;
            fn add(self,other: $v) -> Self::Output {
                $v {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }
        
        impl AddAssign<$v> for $v {
            fn add_assign(&mut self,other: $v) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl Sub<$v> for $v {
            type Output = $v;
            fn sub(self,other: $v) -> Self::Output {
                $v {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }
        
        impl SubAssign<$v> for $v {
            fn sub_assign(&mut self,other: $v) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }
        
        impl Mul<$v> for $t {
            type Output = $v;
            fn mul(self,other: $v) -> Self::Output {
                $v {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                }
            }
        }        
        
        impl Mul<$t> for $v {
            type Output = $v;
            fn mul(self,other: $t) -> Self::Output {
                $v {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }
        
        impl MulAssign<$t> for $v {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }        

        impl Div<$t> for $v {
            type Output = $v;
            fn div(self,other: $t) -> Self::Output {
                $v {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                    w: self.w / other,
                }
            }
        }
        
        impl DivAssign<$t> for $v {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
                self.w /= other;
            }
        }        
    );
);

macro_rules! impl_vec4i (
    ($v:ident,$t:ty) => (

        impl_vec4u!($v,$t);

        impl Neg for $v {
            type Output = Self;
            fn neg(self) -> $v {
                $v {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }
    );
);

macro_rules! impl_vec4f (
    ($v:ident,$t:ty) => (

        impl_vec4i!($v,$t);

        impl $v {
            pub unsafe fn dot(a: $v,b: $v) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
            }

            pub unsafe fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
            }

            pub unsafe fn norm(&self) -> $v {
                let d = self.abs();
                if d != <$t>::zero() {
                    *self / d
                }
                else {
                    *self
                }
            }
        }
    );
);

impl_vec4u!(u8x4_cpu,u8);
impl_vec4i!(i8x4_cpu,i8);
impl_vec4u!(u16x4_cpu,u16);
impl_vec4i!(i16x4_cpu,i16);
impl_vec4u!(u32x4_cpu,u32);
impl_vec4i!(i32x4_cpu,i32);
impl_vec4u!(u64x4_cpu,u64);
impl_vec4i!(i64x4_cpu,i64);
impl_vec4u!(usizex4_cpu,usize);
impl_vec4i!(isizex4_cpu,isize);
impl_vec4f!(f32x4_cpu,f32);
impl_vec4f!(f64x4_cpu,f64);

/// packed 4-part vector of u8.
#[allow(non_camel_case_types)]
pub type u8x4 = u8x4_cpu;

/// packed 4-part vector of i8.
#[allow(non_camel_case_types)]
pub type i8x4 = i8x4_cpu;

/// packed 4-part vector of u16.
#[allow(non_camel_case_types)]
pub type u16x4 = u16x4_cpu;

/// packed 4-part vector of i16.
#[allow(non_camel_case_types)]
pub type i16x4 = i16x4_cpu;

/// packed 4-part vector of u32.
#[allow(non_camel_case_types)]
pub type u32x4 = u32x4_cpu;

/// packed 4-part vector of i32.
#[allow(non_camel_case_types)]
pub type i32x4 = i32x4_cpu;

/// packed 4-part vector of u64.
#[allow(non_camel_case_types)]
pub type u64x4 = u64x4_cpu;

/// packed 4-part vector of i64.
#[allow(non_camel_case_types)]
pub type i64x4 = i64x4_cpu;

/// packed 4-part vector of usize.
#[allow(non_camel_case_types)]
pub type usizex4 = usizex4_cpu;

/// packed 4-part vector of isize.
#[allow(non_camel_case_types)]
pub type isizex4 = isizex4_cpu;

/// packed 4-part vector of f32.
#[allow(non_camel_case_types)]
pub type f32x4 = f32x4_cpu;

/// packed 4-part vector of f64.
#[allow(non_camel_case_types)]
pub type f64x4 = f64x4_cpu;
