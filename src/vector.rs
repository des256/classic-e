// E - Vector
// Desmond Germans, 2020

use crate::*;
use std::cmp::PartialEq;
use std::fmt::{Display,Formatter,Debug,Result};
use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg};

/// Elementary 2-vector.
#[derive(Copy,Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

macro_rules! impl_vec2 (
    ($t:ty) => (
        impl PartialEq for Vec2<$t> {
            fn eq(&self,other: &Vec2<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
            }
        }

        impl Display for Vec2<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl Debug for Vec2<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl Zero for Vec2<$t> {
            /// Return 2-vector at the origin.
            fn zero() -> Vec2<$t> {
                Vec2 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                }
            }
        }

        impl Add<Vec2<$t>> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn add(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl Sub<Vec2<$t>> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn sub(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl AddAssign<Vec2<$t>> for Vec2<$t> {
            fn add_assign(&mut self,other: Vec2<$t>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl SubAssign<Vec2<$t>> for Vec2<$t> {
            fn sub_assign(&mut self,other: Vec2<$t>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self * other.x,
                    y: self * other.y,
                }
            }
        }

        impl Mul<$t> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec2 {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl MulAssign<$t> for Vec2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }

        impl Div<$t> for Vec2<$t> {
            type Output = Vec2<$t>;
            fn div(self,other: $t) -> Self::Output {
                Vec2 {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }

        impl DivAssign<$t> for Vec2<$t> {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
            }
        }
    );
);

macro_rules! impl_vec2_neg (
    ($t:ty) => (
        impl Neg for Vec2<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Vec2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
    );
);

macro_rules! impl_vec2_flt (
    ($t:ty) => (
        impl Vec2<$t> {
            /// Calculate 2-vector dot-product.
            /// # Arguments
            /// * `a` - First 2-vector.
            /// * `b` - Second 2-vector.
            /// # Returns
            /// The dot-product of `a` and `b`.
            pub fn dot(a: Vec2<$t>,b: Vec2<$t>) -> $t {
                a.x * b.x + a.y * b.y
            }

            /// Calculate length or absolute value of a 2-vector.
            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y).sqrt()
            }

            /// Calculate norm of 2-vector (scaled to unit length).
            pub fn norm(&self) -> Vec2<$t> {
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


#[doc(hidden)]
pub fn init_vec2<T>(x: T,y: T) -> crate::Vec2<T> {
    crate::Vec2 {
        x: x,
        y: y,
    }
}

/// Create 2-vector.
#[macro_export]
macro_rules! vec2 (
    ($x:expr,$y:expr) => (
        crate::Vec2 { x: $x,y: $y, }
        //init_vec2($x,$y)
    );
);

impl_vec2!(u8);
impl_vec2!(i8);
impl_vec2_neg!(i8);
impl_vec2!(u16);
impl_vec2!(i16);
impl_vec2_neg!(i16);
impl_vec2!(u32);
impl_vec2!(i32);
impl_vec2_neg!(i32);
impl_vec2!(u64);
impl_vec2!(i64);
impl_vec2_neg!(i64);
impl_vec2!(usize);
impl_vec2!(isize);
impl_vec2_neg!(isize);
impl_vec2!(f32);
impl_vec2_neg!(f32);
impl_vec2_flt!(f32);
impl_vec2!(f64);
impl_vec2_neg!(f64);
impl_vec2_flt!(f64);

/// Elementary 3-vector.
#[derive(Copy,Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_vec3 (
    ($t:ty) => (
        impl PartialEq for Vec3<$t> {
            fn eq(&self,other: &Vec3<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
            }
        }

        impl Display for Vec3<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Debug for Vec3<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Zero for Vec3<$t> {
            /// Return 3-vector at the origin.
            fn zero() -> Vec3<$t> {
                Vec3 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }
        }

        impl Add<Vec3<$t>> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn add(self,other: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl Sub<Vec3<$t>> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn sub(self,other: Vec3<$t>) -> Vec3<$t> {
                Vec3 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl AddAssign<Vec3<$t>> for Vec3<$t> {
            fn add_assign(&mut self,other: Vec3<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl SubAssign<Vec3<$t>> for Vec3<$t> {
            fn sub_assign(&mut self,other: Vec3<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl Mul<$t> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec3 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }
                    

        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                }
            }
        }

        impl MulAssign<$t> for Vec3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl Div<$t> for Vec3<$t> {
            type Output = Vec3<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != <$t>::zero() {
                    Vec3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl DivAssign<$t> for Vec3<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != <$t>::zero() {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }
        }
    );
);

macro_rules! impl_vec3_neg (
    ($t:ty) => (
        impl Neg for Vec3<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }
    );
);

macro_rules! impl_vec3_flt (
    ($t:ty) => (
        impl Vec3<$t> {
            /// Calculate 3-vector cross-product.
            /// # Arguments
            /// * `a` - First 3-vector.
            /// * `b` - Second 3-vector.
            /// # Returns
            /// Cross-product of `a` and `b`.
            pub fn cross(a: Vec3<$t>,b: Vec3<$t>) -> Vec3<$t> {
                Vec3 {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                }
            }

            /// Calculate 3-vector dot-product.
            /// # Arguments
            /// * `a` - First 3-vector.
            /// * `b` - Second 3-vector.
            /// # Returns
            /// Dot-product of `a` and `b`.
            pub fn dot(a: Vec3<$t>,b: Vec3<$t>) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            /// Calculate length or absolute value of a 3-vector.
            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }

            /// Calculate norm of 3-vector (scaled to unit length).
            pub fn norm(&self) -> Vec3<$t> {
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

#[doc(hidden)]
pub fn init_vec3<T>(x: T,y: T,z: T) -> crate::Vec3<T> {
    crate::Vec3 {
        x: x,
        y: y,
        z: z,
    }
}

/// Create 3-vector.
#[macro_export]
macro_rules! vec3 (
    ($x:expr,$y:expr,$z:expr) => (
        crate::Vec3 { x: $x,y: $y,z: $z, }
        //init_vec3($x,$y,$z)
    );
);

impl_vec3!(u8);
impl_vec3!(i8);
impl_vec3_neg!(i8);
impl_vec3!(u16);
impl_vec3!(i16);
impl_vec3_neg!(i16);
impl_vec3!(u32);
impl_vec3!(i32);
impl_vec3_neg!(i32);
impl_vec3!(u64);
impl_vec3!(i64);
impl_vec3_neg!(i64);
impl_vec3!(usize);
impl_vec3!(isize);
impl_vec3_neg!(isize);
impl_vec3!(f32);
impl_vec3_neg!(f32);
impl_vec3_flt!(f32);
impl_vec3!(f64);
impl_vec3_neg!(f64);
impl_vec3_flt!(f64);

/// Elementary 4-vector.
#[derive(Copy,Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_vec4 (
    ($t:ty) => (
        impl PartialEq for Vec4<$t> {
            fn eq(&self,other: &Vec4<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
            }
        }

        impl Display for Vec4<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl Debug for Vec4<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl Zero for Vec4<$t> {
            /// Return 4-vector at the origin. 
            fn zero() -> Vec4<$t> {
                Vec4 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }
        }

        impl Add<Vec4<$t>> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn add(self,other: Vec4<$t>) -> Self::Output {
                Vec4 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }

        impl Sub<Vec4<$t>> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn sub(self,other: Vec4<$t>) -> Vec4<$t> {
                Vec4 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }

        impl AddAssign<Vec4<$t>> for Vec4<$t> {
            fn add_assign(&mut self,other: Vec4<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl SubAssign<Vec4<$t>> for Vec4<$t> {
            fn sub_assign(&mut self,other: Vec4<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }

        impl Mul<$t> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Vec4 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }

        impl Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Self::Output {
                Vec4 {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                }
            }
        }

        impl MulAssign<$t> for Vec4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }

        impl Div<$t> for Vec4<$t> {
            type Output = Vec4<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != <$t>::zero() {
                    Vec4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl DivAssign<$t> for Vec4<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != <$t>::zero() {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }
        }
    );
);

macro_rules! impl_vec4_neg (
    ($t:ty) => (
        impl Neg for Vec4<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Vec4 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }
    );
);

macro_rules! impl_vec4_flt (
    ($t:ty) => (
        impl Vec4<$t> {
            /// Calculate 4-vector dot-product.
            /// # Arguments
            /// * `a` - First 4-vector.
            /// * `b` - Second 4-vector.
            /// # Returns
            /// Dot-product of `a` and `b`.
            pub fn dot(a: Vec4<$t>,b: Vec4<$t>) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
            }

            /// Calculate length or absolute value of 4-vector.
            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
            }

            /// Calculate norm of 4-vector (scaled to unit length).
            pub fn norm(&self) -> Vec4<$t> {
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

#[doc(hidden)]
pub fn init_vec4<T>(x: T,y: T,z: T,w: T) -> crate::Vec4<T> {
    crate::Vec4 {
        x: x,
        y: y,
        z: z,
        w: w,
    }
}

/// Create 4-vector.
#[macro_export]
macro_rules! vec4 (
    ($x:expr,$y:expr,$z:expr,$w:expr) => (
        crate::Vec4 { x: $x,y: $y,z: $z,w: $w, }
        //init_vec4($x,$y,$z,$w)
    );
);

impl_vec4!(u8);
impl_vec4!(i8);
impl_vec4_neg!(i8);
impl_vec4!(u16);
impl_vec4!(i16);
impl_vec4_neg!(i16);
impl_vec4!(u32);
impl_vec4!(i32);
impl_vec4_neg!(i32);
impl_vec4!(u64);
impl_vec4!(i64);
impl_vec4_neg!(i64);
impl_vec4!(usize);
impl_vec4!(isize);
impl_vec4_neg!(isize);
impl_vec4!(f32);
impl_vec4_neg!(f32);
impl_vec4_flt!(f32);
impl_vec4!(f64);
impl_vec4_neg!(f64);
impl_vec4_flt!(f64);
