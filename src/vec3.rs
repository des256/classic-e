// E - Vector
// Desmond Germans, 2020

// Vec3<T> implements a 3D vector.

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

#[derive(Copy,Clone,Debug)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

macro_rules! impl_vec3u {
    ($t:ty; $o:expr; $z:expr) => {
        impl Vec3<$t> {
            pub fn new(x: $t,y: $t,z: $t) -> Self {
                Vec3 { x: x,y: y,z: z, }
            }

            pub fn unit_x() -> Self {
                Vec3 { x: $o,y: $z,z: $z, }
            }

            pub fn unit_y() -> Self {
                Vec3 { x: $z,y: $o,z: $z, }
            }

            pub fn unit_z() -> Self {
                Vec3 { x: $z,y: $z,z: $o, }
            }

            pub fn x(&self) -> $t {
                self.x
            }

            pub fn y(&self) -> $t {
                self.y
            }

            pub fn z(&self) -> $t {
                self.z
            }

            pub fn set_x(&mut self,x: $t) {
                self.x = x;
            }

            pub fn set_y(&mut self,y: $t) {
                self.y = y;
            }

            pub fn set_z(&mut self,z: $t) {
                self.z = z;
            }
        }

        // Vec3 == Vec3
        impl PartialEq for Vec3<$t> {
            fn eq(&self,other: &Self) -> bool {
                (self.x == other.x) &&
                (self.y == other.y) &&
                (self.z == other.z)
            }
        }

        impl Zero for Vec3<$t> {
            fn zero() -> Self {
                Vec3 { x: $z,y: $z,z: $z, }
            }
        }

        impl Display for Vec3<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x(),self.y(),self.z())
            }
        }

        // Vec3 + Vec3
        impl Add<Vec3<$t>> for Vec3<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Vec3 { x: self.x + other.x,y: self.y + other.y,z: self.z + other.z, }
            }
        }

        // Vec3 += Vec3
        impl AddAssign<Vec3<$t>> for Vec3<$t> {
            fn add_assign(&mut self,other: Self) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        // Vec3 - Vec3
        impl Sub<Vec3<$t>> for Vec3<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Vec3 { x: self.x - other.x,y: self.y - other.y,z: self.z - other.z, }
            }
        }

        // Vec3 -= Vec3
        impl SubAssign<Vec3<$t>> for Vec3<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        // s * Vec3
        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Vec3<$t> {
                Vec3 { x: self * other.x,y: self * other.y,z: self * other.z, }
            }
        }

        // Vec3 * s
        impl Mul<$t> for Vec3<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Vec3 { x: self.x * other,y: self.y * other,z: self.z * other, }
            }
        }
        
        // Vec3 *= s
        impl MulAssign<$t> for Vec3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }        

        // Vec3 / s
        impl Div<$t> for Vec3<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Vec3 { x: self.x / other,y: self.y / other,z: self.z / other, }
            }
        }
        
        // Vec3 /= s
        impl DivAssign<$t> for Vec3<$t> {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
            }
        }

        // Vec3 = (Vec2,0)
        impl From<Vec2<$t>> for Vec3<$t> {
            fn from(v: Vec2<$t>) -> Vec3<$t> {
                Vec3::<$t>::new(v.x(),v.y(),$z)
            }
        }

        // Vec3 = Vec3A
        impl From<Vec3A<$t>> for Vec3<$t> {
            fn from(v: Vec3A<$t>) -> Vec3<$t> {
                Vec3::<$t>::new(v.x(),v.y(),v.z())
            }
        }
    }
}

macro_rules! impl_vec3i {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec3u!($t; $o; $z);

        // -Vec3
        impl Neg for Vec3<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Vec3 { x: -self.x,y: -self.y,z: -self.z, }
            }
        }
    }
}

macro_rules! impl_vec3f {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec3i!($t; $o; $z);

        impl Vec3<$t> {
            pub fn dot(_a: &Self,_b: &Self) -> $t {
                // TODO: a.x * b.x + a.y * b.y + a.z * b.z
                $z
            }

            // TODO: cross()

            pub fn abs(&self) -> $t {
                // TODO: (self.x * self.x + self.y * self.y).sqrt()
                $z
            }

            pub fn norm(&self) -> Self {
                // TODO:
                /*
                let d = self.abs();
                if d != <$t>::zero() {
                    *self / d
                }
                else {
                    *self
                }
                */
                Self::zero()
            }
        }
    }
}

impl_vec3u!(u8; 1; 0);
impl_vec3i!(i8; 1; 0);
impl_vec3u!(u16; 1; 0);
impl_vec3i!(i16; 1; 0);
impl_vec3u!(u32; 1; 0);
impl_vec3i!(i32; 1; 0);
impl_vec3u!(u64; 1; 0);
impl_vec3i!(i64; 1; 0);
impl_vec3u!(usize; 1; 0);
impl_vec3i!(isize; 1; 0);
impl_vec3f!(f32; 1.0; 0.0);
impl_vec3f!(f64; 1.0; 0.0);

#[macro_export]
macro_rules! vec3 {
    ($t:ty: $x:expr,$y:expr,$z:expr) => { Vec3::<$t>::new($x,$y,$z) };
}