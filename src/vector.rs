// E - Vector
// Desmond Germans, 2020

use std::cmp::PartialEq;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Result;
use crate::Zero;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::Neg;

#[derive(Copy,Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

macro_rules! impl_vec2 (
    ($t:ty) => (
        impl Vec2<$t> {
            pub fn new(x: $t,y: $t) -> Vec2<$t> {
                Vec2 {
                    x: x,
                    y: y,
                }
            }
        }

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

#[allow(non_camel_case_types)]
pub type u8_2 = Vec2<u8>;

impl_vec2!(u8);

#[allow(non_camel_case_types)]
pub type i8_2 = Vec2<i8>;

impl_vec2!(i8);
impl_vec2_neg!(i8);

#[allow(non_camel_case_types)]
pub type u16_2 = Vec2<u16>;

impl_vec2!(u16);

#[allow(non_camel_case_types)]
pub type i16_2 = Vec2<i16>;

impl_vec2!(i16);
impl_vec2_neg!(i16);

#[allow(non_camel_case_types)]
pub type u32_2 = Vec2<u32>;

impl_vec2!(u32);

#[allow(non_camel_case_types)]
pub type i32_2 = Vec2<i32>;

impl_vec2!(i32);
impl_vec2_neg!(i32);

#[allow(non_camel_case_types)]
pub type u64_2 = Vec2<u64>;

impl_vec2!(u64);

#[allow(non_camel_case_types)]
pub type i64_2 = Vec2<i64>;

impl_vec2!(i64);
impl_vec2_neg!(i64);

#[allow(non_camel_case_types)]
pub type usize_2 = Vec2<usize>;

impl_vec2!(usize);

#[allow(non_camel_case_types)]
pub type isize_2 = Vec2<isize>;

impl_vec2!(isize);
impl_vec2_neg!(isize);

#[allow(non_camel_case_types)]
pub type f32_2 = Vec2<f32>;

impl_vec2!(f32);
impl_vec2_neg!(f32);

#[allow(non_camel_case_types)]
pub type f64_2 = Vec2<f64>;

impl_vec2!(f64);
impl_vec2_neg!(f64);

#[derive(Copy,Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_vec3 (
    ($t:ty) => (
        impl Vec3<$t> {
            pub fn new(x: $t,y: $t,z: $t) -> Vec3<$t> {
                Vec3 {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            pub fn cross(a: Vec3<$t>,b: Vec3<$t>) -> Vec3<$t> {
                Vec3 {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                }
            }

            pub fn dot(a: Vec3<$t>,b: Vec3<$t>) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }

            pub fn norm(self) -> Vec3<$t> {
                let d = self.abs();
                if d != 0.0 {
                    self / d
                }
                else {
                    self
                }
            }
        }

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
            fn zero() -> Vec3<$t> {
                Vec3 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }
        }

        impl Neg for Vec3<$t> {
            type Output = Vec3<$t>;
            fn neg(self) -> Self::Output {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
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
                if other != 0.0 {
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
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32_3 = Vec3<f32>;

impl_vec3!(f32);

#[allow(non_camel_case_types)]
pub type f64_3 = Vec3<f64>;

impl_vec3!(f64);

#[derive(Copy,Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_vec4 (
    ($t:ty) => (
        impl Vec4<$t> {
            pub fn new(x: $t,y: $t,z: $t,w: $t) -> Vec4<$t> {
                Vec4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }
        }

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
            fn zero() -> Vec4<$t> {
                Vec4 {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }
        }

        impl Neg for Vec4<$t> {
            type Output = Vec4<$t>;
            fn neg(self) -> Self::Output {
                Vec4 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
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
                if other != 0.0 {
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
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32_4 = Vec4<f32>;

impl_vec4!(f32);

#[allow(non_camel_case_types)]
pub type f64_4 = Vec4<f64>;

impl_vec4!(f64);