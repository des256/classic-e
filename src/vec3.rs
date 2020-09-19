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

macro_rules! impl_vec3u (
    ($v:ident,$t:ty) => (

        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone)]
        pub struct $v {
            pub x: $t,
            pub y: $t,
            pub z: $t,
        }

        impl $v {
            pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                $v {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            pub fn zero() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }

            pub fn unit_x() -> $v {
                $v {
                    x: <$t>::one(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }

            pub fn unit_y() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::one(),
                    z: <$t>::zero(),
                }
            }

            pub fn unit_z() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::one(),
                }
            }
        }

        impl PartialEq for $v {
            fn eq(&self,other: &$v) -> bool {
                (self.x == other.x) &&
                (self.y == other.y) &&
                (self.z == other.z)
            }
        }

        impl Display for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Debug for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }
        
        impl Zero for $v {
            fn zero() -> Self {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
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
                }
            }
        }
        
        impl AddAssign<$v> for $v {
            fn add_assign(&mut self,other: $v) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl Sub<$v> for $v {
            type Output = $v;
            fn sub(self,other: $v) -> Self::Output {
                $v {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }
        
        impl SubAssign<$v> for $v {
            fn sub_assign(&mut self,other: $v) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }
        
        impl Mul<$v> for $t {
            type Output = $v;
            fn mul(self,other: $v) -> Self::Output {
                $v {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
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
                }
            }
        }
        
        impl MulAssign<$t> for $v {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }        

        impl Div<$t> for $v {
            type Output = $v;
            fn div(self,other: $t) -> Self::Output {
                $v {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }
        
        impl DivAssign<$t> for $v {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
            }
        }        
    );
);

macro_rules! impl_vec3i (
    ($v:ident,$t:ty) => (

        impl_vec3u!($v,$t);

        impl Neg for $v {
            type Output = Self;
            fn neg(self) -> $v {
                $v {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }
    );
);

macro_rules! impl_vec3f (
    ($v:ident,$t:ty) => (

        impl_vec3i!($v,$t);

        impl $v {
            pub unsafe fn cross(a: $v,b: $v) -> $v {
                $v {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                }
            }

            pub unsafe fn dot(a: $v,b: $v) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            pub unsafe fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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

impl_vec3u!(u8x3_cpu,u8);
impl_vec3i!(i8x3_cpu,i8);
impl_vec3u!(u16x3_cpu,u16);
impl_vec3i!(i16x3_cpu,i16);
impl_vec3u!(u32x3_cpu,u32);
impl_vec3i!(i32x3_cpu,i32);
impl_vec3u!(u64x3_cpu,u64);
impl_vec3i!(i64x3_cpu,i64);
impl_vec3u!(usizex3_cpu,usize);
impl_vec3i!(isizex3_cpu,isize);
impl_vec3f!(f32x3_cpu,f32);
impl_vec3f!(f64x3_cpu,f64);

/// packed 3-part vector of u8.
#[allow(non_camel_case_types)]
pub type u8x3 = u8x3_cpu;

/// packed 3-part vector of i8.
#[allow(non_camel_case_types)]
pub type i8x3 = i8x3_cpu;

/// packed 3-part vector of u16.
#[allow(non_camel_case_types)]
pub type u16x3 = u16x3_cpu;

/// packed 3-part vector of i16.
#[allow(non_camel_case_types)]
pub type i16x3 = i16x3_cpu;

/// packed 3-part vector of u32.
#[allow(non_camel_case_types)]
pub type u32x3 = u32x3_cpu;

/// packed 3-part vector of i32.
#[allow(non_camel_case_types)]
pub type i32x3 = i32x3_cpu;

/// packed 3-part vector of u64.
#[allow(non_camel_case_types)]
pub type u64x3 = u64x3_cpu;

/// packed 3-part vector of i64.
#[allow(non_camel_case_types)]
pub type i64x3 = i64x3_cpu;

/// packed 3-part vector of usize.
#[allow(non_camel_case_types)]
pub type usizex3 = usizex3_cpu;

/// packed 3-part vector of isize.
#[allow(non_camel_case_types)]
pub type isizex3 = isizex3_cpu;

/// packed 3-part vector of f32.
#[allow(non_camel_case_types)]
pub type f32x3 = f32x3_cpu;

/// packed 3-part vector of f64.
#[allow(non_camel_case_types)]
pub type f64x3 = f64x3_cpu;

macro_rules! impl_vec3au (
    ($v:ident,$t:ty) => (

        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone)]
        pub struct $v {
            pub x: $t,
            pub y: $t,
            pub z: $t,
            _padding: $t,
        }

        impl $v {
            pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                $v {
                    x: x,
                    y: y,
                    z: z,
                    _padding: <$t>::zero(),
                }
            }

            pub fn zero() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    _padding: <$t>::zero(),
                }
            }

            pub fn unit_x() -> $v {
                $v {
                    x: <$t>::one(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    _padding: <$t>::zero(),
                }
            }

            pub fn unit_y() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::one(),
                    z: <$t>::zero(),
                    _padding: <$t>::zero(),
                }
            }

            pub fn unit_z() -> $v {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::one(),
                    _padding: <$t>::zero(),
                }
            }
        }

        impl PartialEq for $v {
            fn eq(&self,other: &$v) -> bool {
                (self.x == other.x) &&
                (self.y == other.y) &&
                (self.z == other.z)
            }
        }

        impl Display for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Debug for $v {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }
        
        impl Zero for $v {
            fn zero() -> Self {
                $v {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    _padding: <$t>::zero(),
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
                    _padding: <$t>::zero(),
                }
            }
        }
        
        impl AddAssign<$v> for $v {
            fn add_assign(&mut self,other: $v) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl Sub<$v> for $v {
            type Output = $v;
            fn sub(self,other: $v) -> Self::Output {
                $v {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    _padding: <$t>::zero(),
                }
            }
        }
        
        impl SubAssign<$v> for $v {
            fn sub_assign(&mut self,other: $v) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }
        
        impl Mul<$v> for $t {
            type Output = $v;
            fn mul(self,other: $v) -> Self::Output {
                $v {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    _padding: <$t>::zero(),
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
                    _padding: <$t>::zero(),
                }
            }
        }
        
        impl MulAssign<$t> for $v {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }        

        impl Div<$t> for $v {
            type Output = $v;
            fn div(self,other: $t) -> Self::Output {
                $v {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                    _padding: <$t>::zero(),
                }
            }
        }
        
        impl DivAssign<$t> for $v {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
            }
        }        
    );
);

macro_rules! impl_vec3ai (
    ($v:ident,$t:ty) => (

        impl_vec3au!($v,$t);

        impl Neg for $v {
            type Output = Self;
            fn neg(self) -> $v {
                $v {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    _padding: <$t>::zero(),
                }
            }
        }
    );
);

macro_rules! impl_vec3af (
    ($v:ident,$t:ty) => (

        impl_vec3ai!($v,$t);

        impl $v {
            pub unsafe fn cross(a: $v,b: $v) -> $v {
                $v {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                    _padding: <$t>::zero(),
                }
            }

            pub unsafe fn dot(a: $v,b: $v) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            pub unsafe fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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

impl_vec3au!(u8x3a_cpu,u8);
impl_vec3ai!(i8x3a_cpu,i8);
impl_vec3au!(u16x3a_cpu,u16);
impl_vec3ai!(i16x3a_cpu,i16);
impl_vec3au!(u32x3a_cpu,u32);
impl_vec3ai!(i32x3a_cpu,i32);
impl_vec3au!(u64x3a_cpu,u64);
impl_vec3ai!(i64x3a_cpu,i64);
impl_vec3au!(usizex3a_cpu,usize);
impl_vec3ai!(isizex3a_cpu,isize);
impl_vec3af!(f32x3a_cpu,f32);
impl_vec3af!(f64x3a_cpu,f64);

/*#[cfg(target_feature="sse")]
mod sse {
    use crate::vec3::*;

    macro_rules! impl_vec3au32_sse (
        ($v:ident,$t:ty) => (
    
            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 4],
                i: __m128i,
            }
    
            impl $v {
                pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                    $v { i: _mm_set_epi32(x as i32,y as i32,z as i32,0), }
                }
    
                pub fn unit_x() -> $v {
                    $v { i: _mm_set_epi32(1,0,0,0), }
                }
    
                pub fn unit_y() -> $v {
                    $v { i: _mm_set_epi32(0,1,0,0), }
                }
    
                pub fn unit_z() -> $v {
                    $v { i: _mm_set_epi32(0,0,1,0), }
                }
    
                pub fn x(&self) -> $t {
                    self.v[0]
                }
    
                pub fn y(&self) -> $t {
                    self.v[1]
                }
    
                pub fn z(&self) -> $t {
                    self.v[2]
                }
    
                pub fn set_x(&mut self,x: $t) {
                    self.v[0] = x;
                }
    
                pub fn set_y(&mut self,y: $t) {
                    self.v[1] = y;
                }
    
                pub fn set_z(&mut self,z: $t) {
                    self.v[2] = z;
                }
    
                pub fn set_xyz(&mut self,x: $t,y: $t,z: $t) {
                    self.i = _mm_set_epi32(x as i32,y as i32,z as i32,0);
                }
            }
    
            impl PartialEq for $v {
                fn eq(&self,other: &$v) -> bool {
                    _mm_movemask_epi8(_mm_cmpeq_epi32(self.i,other.i)) == 0xFFFF
                }
            }
    
            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
    
            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    $v { i: _mm_set1_epi32(0), }
                }
            }
    
            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    $v { i: _mm_add_epi32(self.i,other.i), }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    self.i = _mm_add_epi32(self.i,other.i);
                }
            }
    
            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    $v { i: _mm_sub_epi32(self.i,other.i), }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    self.i = _mm_sub_epi32(self.i,other.i);
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    $v { i: _mm_mul_epi32(_mm_set1_epi32(self as i32),other.i), }
                }
            }
            
            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    $v { i: _mm_mul_epi32(self.i,_mm_set1_epi32(other as i32)), }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    self.i = _mm_mul_epi32(self.i,_mm_set1_epi32(other as i32));
                }
            }
    
            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    $v { v: [
                        self.v[0] / other,
                        self.v[1] / other,
                        self.v[2] / other,
                        <$t>::zero()
                    ], }
                    //$v { i: _mm_div_epi32(self.i,_mm_set1_epi32(other)), }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    self.v = [
                        self.v[0] / other,
                        self.v[1] / other,
                        self.v[2] / other,
                        <$t>::zero()
                    ];
                    //self.i = _mm_div_epi32(self.i,_mm_set1_epi32(other));
                }
            }
        );
    );

    impl_vec3au32_sse!(u32x3a_sse,u32);
    
    macro_rules! impl_vec3ai32_sse (
        ($v:ident,$t:ty) => (
    
            impl_vec3au32_sse!($v,$t);
    
            impl Neg for $v {
                type Output = Self;
                fn neg(self) -> $v {
                    $v { i: _mm_sub_epi32(_mm_set1_epi32(0),self.i), }
                }
            }
        );
    );

    impl_vec3ai32_sse!(i32x3a_sse,i32);
    
    macro_rules! impl_vec3af32_sse (
        ($v:ident,$t:ty) => (

            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 4],
                f: __m128,
            }
    
            impl $v {
                pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                    $v { f: _mm_set_ps(x,y,z,0.0), }
                }
    
                pub fn unit_x() -> $v {
                    $v { f: _mm_set_ps(1.0,0.0,0.0,0.0), }
                }
    
                pub fn unit_y() -> $v {
                    $v { f: _mm_set_ps(0.0,1.0,0.0,0.0), }
                }
    
                pub fn unit_z() -> $v {
                    $v { f: _mm_set_ps(0.0,0.0,1.0,0.0), }
                }
    
                pub fn x(&self) -> $t {
                    self.v[0]
                }
    
                pub fn y(&self) -> $t {
                    self.v[1]
                }
    
                pub fn z(&self) -> $t {
                    self.v[2]
                }
    
                pub fn set_x(&mut self,x: $t) {
                    self.v[0] = x;
                }
    
                pub fn set_y(&mut self,y: $t) {
                    self.v[1] = y;
                }
    
                pub fn set_z(&mut self,z: $t) {
                    self.v[2] = z;
                }
    
                pub fn set_xyz(&mut self,x: $t,y: $t,z: $t) {
                    self.f = _mm_set_ps(x,y,z,0.0);
                }

                pub fn cross(a: $v,b: $v) -> $v {
                    $v {
                        v: [
                            a.v[1] * b.v[2] - a.v[2] * b.v[1],
                            a.v[2] * b.v[0] - a.v[0] * b.v[2],
                            a.v[0] * b.v[1] - a.v[1] * b.v[0],
                            <$t>::zero()
                        ]
                    }
                }
    
                pub fn dot(a: $v,b: $v) -> $t {
                    $v { f: _mm_dp_ps(a.f,b.f,0xFF), }.v[0]
                }
    
                pub fn abs(&self) -> $t {
                    $v { f: _mm_dp_ps(self.f,self.f,0xFF), }.v[0].sqrt()
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
                    _mm_movemask_ps(_mm_cmpeq_ps(self.f,other.f)) == 0xF
                }
            }
    
            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
    
            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    $v { f: _mm_set1_ps(0.0), }
                }
            }        
    
            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    $v { f: _mm_add_ps(self.f,other.f), }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    self.f = _mm_add_ps(self.f,other.f);
                }
            }
    
            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    $v { f: _mm_sub_ps(self.f,other.f), }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    self.f = _mm_sub_ps(self.f,other.f);
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    $v { f: _mm_mul_ps(_mm_set1_ps(self),other.f), }
                }
            }        
            
            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    $v { f: _mm_mul_ps(self.f,_mm_set1_ps(other)), }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    self.f = _mm_mul_ps(self.f,_mm_set1_ps(other));
                }
            }        
    
            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    $v { f: _mm_div_ps(self.f,_mm_set1_ps(other)), }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    self.f = _mm_div_ps(self.f,_mm_set1_ps(other));
                }
            }        

            impl Neg for $v {
                type Output = Self;
                fn neg(self) -> $v {
                    $v { f: _mm_sub_ps(_mm_set1_ps(0.0),self.f), }
                }
            }
        );
    );
    
    impl_vec3af32_sse!(f32x3a_sse,f32);
}

#[cfg(target_feature="sse")]
pub use sse::*;*/

/*#[cfg(target_feature="avx")]
mod avx {
    use crate::vec3::*;

    macro_rules! impl_vec3au64_avx (
        ($v:ident,$t:ty) => (
    
            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 4],
                i: __m256i,
            }
    
            impl $v {
                pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                    $v { i: _mm256_set_epi64(x as i64,y as i64,z as i64,0), }
                }
    
                pub fn unit_x() -> $v {
                    $v { i: _mm256_set_epi64(1,0,0,0), }
                }
    
                pub fn unit_y() -> $v {
                    $v { i: _mm256_set_epi64(0,1,0,0), }
                }
    
                pub fn unit_z() -> $v {
                    $v { i: _mm256_set_epi64(0,0,1,0), }
                }
    
                pub fn x(&self) -> $t {
                    self.v[0]
                }
    
                pub fn y(&self) -> $t {
                    self.v[1]
                }
    
                pub fn z(&self) -> $t {
                    self.v[2]
                }
    
                pub fn set_x(&mut self,x: $t) {
                    self.v[0] = x;
                }
    
                pub fn set_y(&mut self,y: $t) {
                    self.v[1] = y;
                }
    
                pub fn set_z(&mut self,z: $t) {
                    self.v[2] = z;
                }
    
                pub fn set_xyz(&mut self,x: $t,y: $t,z: $t) {
                    self.i = _mm256_set_epi64(x as i64,y as i64,z as i64,0);
                }
            }
    
            impl PartialEq for $v {
                fn eq(&self,other: &$v) -> bool {
                    _mm256_movemask_epi8(_mm256_cmpeq_epi64(self.i,other.i)) == 0xFFFFFFFF
                }
            }
    
            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
    
            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    $v { i: _mm256_set1_epi64(0), }
                }
            }
    
            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    $v { i: _mm256_add_epi64(self.i,other.i), }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    self.i = _mm256_add_epi64(self.i,other.i);
                }
            }
    
            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    $v { i: _mm256_sub_epi64(self.i,other.i), }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    self.i = _mm256_sub_epi64(self.i,other.i);
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    $v { i: _mm256_mul_epi64(_mm256_set1_epi64(self as i64),other.i), }
                }
            }
            
            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    $v { i: _mm256_mul_epi64(self.i,_mm256_set1_epi64(other as i64)), }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    self.i = _mm256_mul_epi64(self.i,_mm256_set1_epi64(other as i32));
                }
            }
    
            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    $v { v: [
                        self.v[0] / other,
                        self.v[1] / other,
                        self.v[2] / other,
                        <$t>::zero()
                    ], }
                    //$v { i: _mm_div_epi32(self.i,_mm_set1_epi32(other)), }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    self.v = [
                        self.v[0] / other,
                        self.v[1] / other,
                        self.v[2] / other,
                        <$t>::zero()
                    ];
                    //self.i = _mm_div_epi32(self.i,_mm_set1_epi32(other));
                }
            }
        );
    );

    impl_vec3au64_avx!(u64x3a_avx,u64);
    
    macro_rules! impl_vec3ai64_avx (
        ($v:ident,$t:ty) => (
    
            impl_vec3au64_avx!($v,$t);
    
            impl Neg for $v {
                fn neg(self) -> $v {
                    $v { i: _mm256_sub_epi64(_mm256_set1_epi64(0),self.i), }
                }
            }
        );
    );

    impl_vec3ai64_sse!(i64x3a_avx,i64);
    
    macro_rules! impl_vec3af64_avx (
        ($v:ident,$t:ty) => (

            #[allow(non_camel_case_types)]
            #[derive(Copy,Clone)]
            pub union $v {
                v: [$t; 4],
                d: __m256,
            }
    
            impl $v {
                pub fn from_xyz(x: $t,y: $t,z: $t) -> $v {
                    $v { d: _mm256_set_pd(x,y,z,0.0), }
                }
    
                pub fn unit_x() -> $v {
                    $v { d: _mm256_set_pd(1.0,0.0,0.0,0.0), }
                }
    
                pub fn unit_y() -> $v {
                    $v { d: _mm256_set_pd(0.0,1.0,0.0,0.0), }
                }
    
                pub fn unit_z() -> $v {
                    $v { d: _mm256_set_pd(0.0,0.0,1.0,0.0), }
                }
    
                pub fn x(&self) -> $t {
                    self.v[0]
                }
    
                pub fn y(&self) -> $t {
                    self.v[1]
                }
    
                pub fn z(&self) -> $t {
                    self.v[2]
                }
    
                pub fn set_x(&mut self,x: $t) {
                    self.v[0] = x;
                }
    
                pub fn set_y(&mut self,y: $t) {
                    self.v[1] = y;
                }
    
                pub fn set_z(&mut self,z: $t) {
                    self.v[2] = z;
                }
    
                pub fn set_xyz(&mut self,x: $t,y: $t,z: $t) {
                    self.d = _mm256_set_pd(x,y,z,0.0);
                }

                pub fn cross(a: $v,b: $v) -> $v {
                    $v {
                        v: [
                            a.v[1] * b.v[2] - a.v[2] * b.v[1],
                            a.v[2] * b.v[0] - a.v[0] * b.v[2],
                            a.v[0] * b.v[1] - a.v[1] * b.v[0]
                        ]
                    }
                }
    
                pub fn dot(a: $v,b: $v) -> $t {
                    $v { d: _mm256_dp_pd(a.d,b.d,0xFF), }.v[0]
                }
    
                pub fn abs(&self) -> $t {
                    $v { d: _mm256_dp_pd(self.d,self.d,0xFF), }.v[0].sqrt()
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
                    _mm256_movemask_pd(_mm256_cmpeq_pd(self.d,other.d)) == 0xF
                }
            }
    
            impl Display for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
    
            impl Debug for $v {
                fn fmt(&self,f: &mut Formatter) -> Result {
                    write!(f,"({},{},{})",self.v[0],self.v[1],self.v[2])
                }
            }
            
            impl Zero for $v {
                fn zero() -> Self {
                    $v { d: _mm256_set1_pd(0.0), }
                }
            }        
    
            impl Add<$v> for $v {
                type Output = $v;
                fn add(self,other: $v) -> Self::Output {
                    $v { d: _mm256_add_pd(self.d,other.d), }
                }
            }
            
            impl AddAssign<$v> for $v {
                fn add_assign(&mut self,other: $v) {
                    self.d = _mm256_add_pd(self.d,other.d);
                }
            }
    
            impl Sub<$v> for $v {
                type Output = $v;
                fn sub(self,other: $v) -> Self::Output {
                    $v { d: _mm256_sub_pd(self.d,other.d), }
                }
            }
            
            impl SubAssign<$v> for $v {
                fn sub_assign(&mut self,other: $v) {
                    self.d = _mm256_sub_pd(self.d,other.d);
                }
            }
            
            impl Mul<$v> for $t {
                type Output = $v;
                fn mul(self,other: $v) -> Self::Output {
                    $v { d: _mm256_mul_pd(_mm256_set1_pd(self),other.d), }
                }
            }        
            
            impl Mul<$t> for $v {
                type Output = $v;
                fn mul(self,other: $t) -> Self::Output {
                    $v { f: _mm256_mul_pd(self.d,_mm256_set1_pd(other)), }
                }
            }
            
            impl MulAssign<$t> for $v {
                fn mul_assign(&mut self,other: $t) {
                    self.d = _mm256_mul_pd(self.d,_mm256_set1_pd(other));
                }
            }        
    
            impl Div<$t> for $v {
                type Output = $v;
                fn div(self,other: $t) -> Self::Output {
                    $v { d: _mm256_div_pd(self.d,_mm256_set1_pd(other)), }
                }
            }
            
            impl DivAssign<$t> for $v {
                fn div_assign(&mut self,other: $t) {
                    self.d = _mm256_div_pd(self.d,_mm256_set1_pd(other));
                }
            }        

            impl Neg for $v {
                fn neg(self) -> $v {
                    $v { d: _mm256_sub_pd(_mm256_set1_pd(0.0),self.f), }
                }
            }
        );
    );
    
    impl_vec3af64_avx!(f64x3a_avx,f64);
}*/

/// aligned 3-part vector of u8.
#[allow(non_camel_case_types)]
pub type u8x3a = u8x3a_cpu;

/// aligned 3-part vector of i8.
#[allow(non_camel_case_types)]
pub type i8x3a = i8x3a_cpu;

/// aligned 3-part vector of u16.
#[allow(non_camel_case_types)]
pub type u16x3a = u16x3a_cpu;

/// aligned 3-part vector of i16.
#[allow(non_camel_case_types)]
pub type i16x3a = i16x3a_cpu;

/// aligned 3-part vector of u32.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="sse"))]
pub type u32x3a = u32x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="sse"))]
//pub type u32x3a = u32x3a_sse;

/// aligned 3-part vector of i32.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="sse"))]
pub type i32x3a = i32x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="sse")]
//pub type i32x3a = i32x3a_sse;

/// aligned 3-part vector of u64.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="avx"))]
pub type u64x3a = u64x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="avx")]
//pub type u64x3a = u64x3a_avx;

/// aligned 3-part vector of i64.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="avx"))]
pub type i64x3a = i64x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="avx")]
//pub type i64x3a = i64x3a_sse;

/// aligned 3-part vector of usize.
#[allow(non_camel_case_types)]
pub type usizex3a = usizex3a_cpu;

/// aligned 3-part vector of isize.
#[allow(non_camel_case_types)]
pub type isizex3a = isizex3a_cpu;

/// aligned 3-part vector of f32.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="sse"))]
pub type f32x3a = f32x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="sse")]
//pub type f32x3a = f32x3a_sse;

/// packed 3-part vector of f64.
#[allow(non_camel_case_types)]
//#[cfg(not(target_feature="avx"))]
pub type f64x3a = f64x3a_cpu;
//#[allow(non_camel_case_types)]
//#[cfg(target_feature="avx")]
//pub type f64x3a = f64x3a_avx;
