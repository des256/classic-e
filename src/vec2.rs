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

#[cfg(target_arch="x86")]
use core::arch::x86::*;
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::*;

macro_rules! impl_vec2u (
    ($v:ident,$t:ty) => (

        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone)]
        pub struct $v {
            _x: $t,
            _y: $t,
        }

        impl $v {
            pub fn from_xy(x: $t,y: $t) -> $v {
                $v { _x: x,_y: y, }
            }

            pub fn unit_x() -> $v {
                $v { _x: <$t>::one(),_y: <$t>::zero(), }
            }

            pub fn unit_y() -> $v {
                $v { _x: <$t>::zero(),_y: <$t>::one(), }
            }

            pub fn x(&mut self) -> &mut $t {
                &mut self._x
            }

            pub fn y(&mut self) -> &mut $t {
                &mut self._y
            }
        }

        impl PartialEq for $v {
            fn eq(&self,other: &$v) -> bool {
                (self._x == other._x) &&
                (self._y == other._y)
            }
        }

        impl Display for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{})",self._x,self._y)
            }
        }

        impl Debug for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{})",self._x,self._y)
            }
        }
        
        impl Zero for $v {
            fn zero() -> Self {
                $v { _x: <$t>::zero(),_y: <$t>::zero(), }
            }
        }        

        impl Add<$v> for $v {
            type Output = $v;
            fn add(self,other: $v) -> Self::Output {
                $v {
                    _x: self._x + other._x,
                    _y: self._y + other._y,
                }
            }
        }
        
        impl AddAssign<$v> for $v {
            fn add_assign(&mut self,other: $v) {
                self._x += other._x;
                self._y += other._y;
            }
        }

        impl Sub<$v> for $v {
            type Output = $v;
            fn sub(self,other: $v) -> Self::Output {
                $v {
                    _x: self._x - other._x,
                    _y: self._y - other._y,
                }
            }
        }
        
        impl SubAssign<$v> for $v {
            fn sub_assign(&mut self,other: $v) {
                self._x -= other._x;
                self._y -= other._y;
            }
        }
        
        impl Mul<$v> for $t {
            type Output = $v;
            fn mul(self,other: $v) -> Self::Output {
                $v {
                    _x: self * other._x,
                    _y: self * other._y,
                }
            }
        }        
        
        impl Mul<$t> for $v {
            type Output = $v;
            fn mul(self,other: $t) -> Self::Output {
                $v {
                    _x: self._x * other,
                    _y: self._y * other,
                }
            }
        }
        
        impl MulAssign<$t> for $v {
            fn mul_assign(&mut self,other: $t) {
                self._x *= other;
                self._y *= other;
            }
        }        

        impl Div<$t> for $v {
            type Output = $v;
            fn div(self,other: $t) -> Self::Output {
                $v {
                    _x: self._x / other,
                    _y: self._y / other,
                }
            }
        }
        
        impl DivAssign<$t> for $v {
            fn div_assign(&mut self,other: $t) {
                self._x /= other;
                self._y /= other;
            }
        }        
    );
);

macro_rules! impl_vec2i (
    ($v:ident,$t:ty) => (

        impl_vec2u!($v,$t);

        impl Neg for $v {
            type Output = Self;
            fn neg(self) -> $v {
                $v {
                    _x: -self._x,
                    _y: -self._y,
                }
            }
        }
    );
);

macro_rules! impl_vec2f (
    ($v:ident,$t:ty) => (

        impl_vec2i!($v,$t);

        impl $v {
            pub fn dot(a: $v,b: $v) -> $t {
                a._x * b._x + a._y * b._y
            }

            pub fn abs(&self) -> $t {
                (self._x * self._x + self._y * self._y).sqrt()
            }

            pub fn norm(&self) -> $v {
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

impl_vec2u!(u8x2_cpu,u8);
impl_vec2i!(i8x2_cpu,i8);
impl_vec2u!(u16x2_cpu,u16);
impl_vec2i!(i16x2_cpu,i16);
impl_vec2u!(u32x2_cpu,u32);
impl_vec2i!(i32x2_cpu,i32);
impl_vec2u!(u64x2_cpu,u64);
impl_vec2i!(i64x2_cpu,i64);
impl_vec2u!(usizex2_cpu,usize);
impl_vec2i!(isizex2_cpu,isize);
impl_vec2f!(f32x2_cpu,f32);
impl_vec2f!(f64x2_cpu,f64);

#[cfg(target_feature="sse")]
mod sse {
    use crate::vec2::*;

    macro_rules! impl_u64x2_sse (
        ($v:ident,$t:ty) => (

            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 2],
                i: __m128i,
            }

            impl $v {
                pub fn from_xy(x: $t,y: $t) -> $v {
                    unsafe { $v { i: _mm_set_epi64x(x as i64,y as i64), } }
                }

                pub fn unit_x() -> $v {
                    unsafe { $v { i: _mm_set_epi64x(1,0), } }
                }

                pub fn unit_y() -> $v {
                    unsafe { $v { i: _mm_set_epi64x(0,1), } }
                }

                pub fn x(&mut self) -> &mut $t {
                    unsafe { &mut self.v[0] }
                }

                pub fn y(&mut self) -> &mut $t {
                    unsafe { &mut self.v[1] }
                }
            }

            impl PartialEq for $v {
                fn eq(&self,other: &$v) -> bool {
                    unsafe { _mm_movemask_epi8(_mm_cmpeq_epi64(self.i,other.i)) == 0xFFFF }
                }
            }

            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",*self.x(),*self.y())
                }
            }

            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",*self.x(),*self.y())
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    unsafe { $v { i: _mm_set1_epi64x(0), } }
                }
            }        

            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    unsafe { $v { i: _mm_add_epi64(self.i,other.i), } }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    unsafe { self.i = _mm_add_epi64(self.i,other.i) }
                }
            }

            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    unsafe { $v { i: _mm_sub_epi64(self.i,other.i), } }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    unsafe { self.i = _mm_sub_epi64(self.i,other.i) }
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    unsafe { $v { v: [ self * *other.x(),self * *other.y(), ] } }
                }
            }        
            
            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    unsafe { $v { v: [ *self.x() * other,*self.y() * other, ] } }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    unsafe { self.v = [ *self.x() * other,*self.y() * other, ] }
                }
            }        

            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    unsafe { $v { v: [ *self.x() / other,*self.y() / other, ] } }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    unsafe { self.v = [ *self.x() / other,*self.y() / other, ] }
                }
            }        
        );
    );

    impl_u64x2_sse!(u64x2_sse,u64);

    macro_rules! impl_i64x2_sse (
        ($v:ident,$t:ty) => (
            impl_u64x2_sse!($v,$t);

            impl Neg for $v {
                type Output = Self;
                fn neg(self) -> $v {
                    unsafe { $v { i: _mm_sub_epi64(_mm_set1_epi64x(0),self.i), } }
                }
            }
        );
    );

    impl_i64x2_sse!(i64x2_sse,i64);

    /*macro_rules! impl_vec2f64_sse (
        ($v:ident,$t:ty) => (

            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 2],
                d: __m128d,
            }

            impl $v {
                pub fn from_xy(x: $t,y: $t) -> $v {
                    $v { d: _mm_set_pd(x,y), }
                }

                pub fn unit_x() -> $v {
                    $v { d: _mm_set_pd(1.0,0.0), }
                }

                pub fn unit_y() -> $v {
                    $v { d: _mm_set_pd(0.0,1.0), }
                }

                pub fn x(&self) -> $t {
                    self.v[0]
                }

                pub fn y(&self) -> $t {
                    self.v[1]
                }

                pub fn set_x(&mut self,x: $t) {
                    self.v[0] = x;
                }

                pub fn set_y(&mut self,y: $t) {
                    self.v[1] = y;
                }

                pub fn set_xy(&mut self,x: $t,y: $t) {
                    self.d = _mm_set_pd(x,y);
                }

                pub fn dot(a: $v,b: $v) -> $t {
                    $v { d: _mm_dp_pd(a.d,b.d,0xFF), }.v[0]
                }

                pub fn abs(&self) -> $t {
                    $v { d: _mm_dp_pd(self.d,self.d,0xFF), }.v[0].sqrt()
                }

                pub fn norm(&self) -> $v {
                    let d = self.abs();
                    if d != <$t>::zero() {
                        *self / d
                    }
                    else {
                        *self
                    }
                }
            }

            impl PartialEq for $v {
                fn eq(&self,other: &$v) -> bool {
                    (self.v[0] == other.v[0]) &&
                    (self.v[1] == other.v[1])
                }
            }

            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",self.v[0],self.v[1])
                }
            }

            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{})",self.v[0],self.v[1])
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    $v { d: _mm_set1_pd(0.0), }
                }
            }        

            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    $v { d: _mm_add_pd(self.d,other.d), }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    self.d = _mm_add_pd(self.d,other.d);
                }
            }

            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    $v { d: _mm_sub_pd(self.d,other.d), }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    self.d = _mm_sub_pd(self.d,other.d);
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    $v { d: _mm_mul_pd(_mm_set1_pd(self),other.d), }
                }
            }

            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    $v { d: _mm_mul_pd(self.d,_mm_set1_pd(other)), }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    self.d = _mm_mul_pd(self.d,_mm_set1_pd(other));
                }
            }

            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    $v { d: _mm_div_pd(self.d,_mm_set1_pd(other)), }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    self.d = _mm_div_pd(self.d,_mm_set1_pd(other));
                }
            }

            impl Neg for $v {
                type Output = Self;
                fn neg(self) -> $v {
                    $v { d: _mm_sub_pd(_mm_set1_pd(0.0),self.d), }
                }
            }
        );
    );

    impl_vec2f64_sse!(f64x2_sse,f64);*/
}

#[cfg(target_feature="sse")]
pub use sse::*;

/// packed 2-part vector of u8.
#[allow(non_camel_case_types)]
pub type u8x2 = u8x2_cpu;

/// packed 2-part vector of i8.
#[allow(non_camel_case_types)]
pub type i8x2 = i8x2_cpu;

/// packed 2-part vector of u16.
#[allow(non_camel_case_types)]
pub type u16x2 = u16x2_cpu;

/// packed 2-part vector of i16.
#[allow(non_camel_case_types)]
pub type i16x2 = i16x2_cpu;

/// packed 2-part vector of u32.
#[allow(non_camel_case_types)]
pub type u32x2 = u32x2_cpu;

/// packed 2-part vector of i32.
#[allow(non_camel_case_types)]
pub type i32x2 = i32x2_cpu;

/// packed 2-part vector of u64.
#[allow(non_camel_case_types)]
#[cfg(not(target_feature="sse"))]
pub type u64x2 = u64x2_cpu;
#[allow(non_camel_case_types)]
#[cfg(target_feature="sse")]
pub type u64x2 = u64x2_sse;

/// packed 2-part vector of i64.
#[allow(non_camel_case_types)]
#[cfg(not(target_feature="sse"))]
pub type i64x2 = i64x2_cpu;
#[allow(non_camel_case_types)]
#[cfg(target_feature="sse")]
pub type i64x2 = i64x2_sse;

/// packed 2-part vector of usize.
#[allow(non_camel_case_types)]
pub type usizex2 = usizex2_cpu;

/// packed 2-part vector of isize.
#[allow(non_camel_case_types)]
pub type isizex2 = isizex2_cpu;

/// packed 2-part vector of f32.
#[allow(non_camel_case_types)]
pub type f32x2 = f32x2_cpu;

/// packed 2-part vector of f64.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="sse"))]
pub type f64x2 = f64x2_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="sse")]
//pub type f64x2 = f64x2_sse;
