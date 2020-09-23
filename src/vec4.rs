// E - Vector
// Desmond Germans, 2020

// Vec4<T> implements a 4D vector.

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
pub struct Vec4<T: Simd4>(pub <T as Simd4>::Type);

macro_rules! impl_vec4u {
    ($t:ty; $o:expr; $z:expr) => {
        impl Vec4<$t> {
            pub fn new(x: $t,y: $t,z: $t,w: $t) -> Self {
                Vec4(<$t as Simd4>::Type::new(x,y,z,w))
            }

            pub fn unit_x() -> Self {
                Vec4(<$t as Simd4>::Type::new($o,$z,$z,$z))
            }

            pub fn unit_y() -> Self {
                Vec4(<$t as Simd4>::Type::new($z,$o,$z,$z))
            }

            pub fn unit_z() -> Self {
                Vec4(<$t as Simd4>::Type::new($z,$z,$o,$z))
            }

            pub fn unit_w() -> Self {
                Vec4(<$t as Simd4>::Type::new($z,$z,$z,$o))
            }

            pub fn x(&self) -> $t {
                self.0.get(0)
            }

            pub fn y(&self) -> $t {
                self.0.get(1)
            }

            pub fn z(&self) -> $t {
                self.0.get(2)
            }

            pub fn w(&self) -> $t {
                self.0.get(3)
            }

            pub fn set_x(&mut self,x: $t) {
                self.0.set(0,x);
            }

            pub fn set_y(&mut self,y: $t) {
                self.0.set(1,y);
            }

            pub fn set_z(&mut self,z: $t) {
                self.0.set(2,z);
            }

            pub fn set_w(&mut self,w: $t) {
                self.0.set(3,w);
            }
        }

        // Vec4 == Vec4
        impl PartialEq for Vec4<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd4>::Type::eq(&self.0,&other.0,0xF)
            }
        }

        impl Zero for Vec4<$t> {
            fn zero() -> Self {
                Vec4(<$t as Simd4>::Type::zero())
            }
        }

        impl Display for Vec4<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{},{})",self.x(),self.y(),self.z(),self.w())
            }
        }

        // Vec4 + Vec4
        impl Add<Vec4<$t>> for Vec4<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Vec4(<$t as Simd4>::Type::add(&self.0,&other.0))
            }
        }

        // Vec4 += Vec4
        impl AddAssign<Vec4<$t>> for Vec4<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::add(&self.0,&other.0);
            }
        }

        // Vec4 - Vec4
        impl Sub<Vec4<$t>> for Vec4<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Vec4(<$t as Simd4>::Type::sub(&self.0,&other.0))
            }
        }

        // Vec4 -= Vec4
        impl SubAssign<Vec4<$t>> for Vec4<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::sub(&self.0,&other.0);
            }
        }

        // s * Vec4
        impl Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Vec4<$t> {
                Vec4(<$t as Simd4>::Type::mul(&<$t as Simd4>::Type::splat(self),&other.0))
            }
        }

        // Vec4 * s
        impl Mul<$t> for Vec4<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Vec4(<$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }
        
        // Vec4 *= s
        impl MulAssign<$t> for Vec4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }        

        // Vec4 / s
        impl Div<$t> for Vec4<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Vec4(<$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }
        
        // Vec4 /= s
        impl DivAssign<$t> for Vec4<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }

        // Vec4 = (Vec2,0,1)
        impl From<Vec2<$t>> for Vec4<$t> {
            fn from(v: Vec2<$t>) -> Vec4<$t> {
                Vec4::<$t>::new(v.x(),v.y(),$z,$o)
            }
        }

        // Vec4 = (Vec3,1)
        impl From<Vec3<$t>> for Vec4<$t> {
            fn from(v: Vec3<$t>) -> Vec4<$t> {
                Vec4::<$t>::new(v.x(),v.y(),v.z(),$o)
            }
        }

        // Vec4 = (Vec3A,1)
        impl From<Vec3A<$t>> for Vec4<$t> {
            fn from(v: Vec3A<$t>) -> Vec4<$t> {
                Vec4::<$t>::new(v.x(),v.y(),v.z(),$o)
            }
        }
    }
}

macro_rules! impl_vec4i {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec4u!($t; $o; $z);

        impl Neg for Vec4<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Vec4(<$t as Simd4>::Type::sub(&<$t as Simd4>::Type::zero(),&self.0))
            }
        }
    }
}

macro_rules! impl_vec4f {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec4i!($t; $o; $z);

        impl Vec4<$t> {
            pub fn dot(_a: &Self,_b: &Self) -> $t {
                // TODO: a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
                $z
            }

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

impl_vec4u!(u8; 1; 0);
impl_vec4i!(i8; 1; 0);
impl_vec4u!(u16; 1; 0);
impl_vec4i!(i16; 1; 0);
impl_vec4u!(u32; 1; 0);
impl_vec4i!(i32; 1; 0);
impl_vec4u!(u64; 1; 0);
impl_vec4i!(i64; 1; 0);
impl_vec4u!(usize; 1; 0);
impl_vec4i!(isize; 1; 0);
impl_vec4f!(f32; 1.0; 0.0);
impl_vec4f!(f64; 1.0; 0.0);

#[macro_export]
macro_rules! vec4 {
    ($t:ty: $x:expr,$y:expr,$z:expr,$w:expr) => { Vec4::<$t>::new($x,$y,$z,$w) };
}