// E - Vector
// Desmond Germans, 2020

// Vec3A<T> implements a 3D vector.

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
pub struct Vec3A<T: Simd4>(pub <T as Simd4>::Type);

macro_rules! impl_vec3au {
    ($t:ty; $o:expr; $z:expr) => {
        impl Vec3A<$t> {
            pub fn new(x: $t,y: $t,z: $t) -> Self {
                Vec3A(<$t as Simd4>::Type::new(x,y,z,$z))
            }

            pub fn unit_x() -> Self {
                Vec3A(<$t as Simd4>::Type::new($o,$z,$z,$z))
            }

            pub fn unit_y() -> Self {
                Vec3A(<$t as Simd4>::Type::new($z,$o,$z,$z))
            }

            pub fn unit_z() -> Self {
                Vec3A(<$t as Simd4>::Type::new($z,$z,$o,$z))
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

            pub fn set_x(&mut self,x: $t) {
                self.0.set(0,x);
            }

            pub fn set_y(&mut self,y: $t) {
                self.0.set(1,y);
            }

            pub fn set_z(&mut self,z: $t) {
                self.0.set(2,z);
            }
        }

        // Vec3A == Vec3A
        impl PartialEq for Vec3A<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd4>::Type::eq(&self.0,&other.0,0x7)
            }
        }

        impl Zero for Vec3A<$t> {
            fn zero() -> Self {
                Vec3A(<$t as Simd4>::Type::zero())
            }
        }

        impl Display for Vec3A<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{},{})",self.x(),self.y(),self.z())
            }
        }

        // Vec3A + Vec3A
        impl Add<Vec3A<$t>> for Vec3A<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Vec3A(<$t as Simd4>::Type::add(&self.0,&other.0))
            }
        }

        // Vec3A += Vec3A
        impl AddAssign<Vec3A<$t>> for Vec3A<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::add(&self.0,&other.0);
            }
        }

        // Vec3A - Vec3A
        impl Sub<Vec3A<$t>> for Vec3A<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Vec3A(<$t as Simd4>::Type::sub(&self.0,&other.0))
            }
        }

        // Vec3A -= Vec3A
        impl SubAssign<Vec3A<$t>> for Vec3A<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::sub(&self.0,&other.0);
            }
        }

        // s * Vec3A
        impl Mul<Vec3A<$t>> for $t {
            type Output = Vec3A<$t>;
            fn mul(self,other: Vec3A<$t>) -> Vec3A<$t> {
                Vec3A(<$t as Simd4>::Type::mul(&<$t as Simd4>::Type::splat(self),&other.0))
            }
        }

        // Vec3A * s
        impl Mul<$t> for Vec3A<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Vec3A(<$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }
        
        // Vec3A *= s
        impl MulAssign<$t> for Vec3A<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }        

        // Vec3A / s
        impl Div<$t> for Vec3A<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Vec3A(<$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }
        
        // Vec3A /= s
        impl DivAssign<$t> for Vec3A<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }

        // Vec3A = (Vec2,0)
        impl From<Vec2<$t>> for Vec3A<$t> {
            fn from(v: Vec2<$t>) -> Vec3A<$t> {
                Vec3A::<$t>::new(v.x(),v.y(),$z)
            }
        }

        // Vec3A = Vec3
        impl From<Vec3<$t>> for Vec3A<$t> {
            fn from(v: Vec3<$t>) -> Vec3A<$t> {
                Vec3A::<$t>::new(v.x(),v.y(),v.z())
            }
        }
    }
}

macro_rules! impl_vec3ai {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec3au!($t; $o; $z);

        impl Neg for Vec3A<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Vec3A(<$t as Simd4>::Type::sub(&<$t as Simd4>::Type::zero(),&self.0))
            }
        }
    }
}

macro_rules! impl_vec3af {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec3ai!($t; $o; $z);

        impl Vec3A<$t> {
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

impl_vec3au!(u8; 1; 0);
impl_vec3ai!(i8; 1; 0);
impl_vec3au!(u16; 1; 0);
impl_vec3ai!(i16; 1; 0);
impl_vec3au!(u32; 1; 0);
impl_vec3ai!(i32; 1; 0);
impl_vec3au!(u64; 1; 0);
impl_vec3ai!(i64; 1; 0);
impl_vec3au!(usize; 1; 0);
impl_vec3ai!(isize; 1; 0);
impl_vec3af!(f32; 1.0; 0.0);
impl_vec3af!(f64; 1.0; 0.0);

#[macro_export]
macro_rules! vec3a {
   ($t:ty: $x:expr,$y:expr,$z:expr) => { Vec3A::<$t>::new($x,$y,$z) };
}
