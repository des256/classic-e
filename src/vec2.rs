// E - Vector
// Desmond Germans, 2020

// Vec2<T> implements a 2D vector.

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
pub struct Vec2<T: Simd2>(pub <T as Simd2>::Type);

macro_rules! impl_vec2u {
    ($t:ty; $o:expr; $z:expr) => {
        impl Vec2<$t> {
            pub fn new(x: $t,y: $t) -> Self {
                Vec2(<$t as Simd2>::Type::new(x,y))
            }

            pub fn unit_x() -> Self {
                Vec2(<$t as Simd2>::Type::new($o,$z))
            }

            pub fn unit_y() -> Self {
                Vec2(<$t as Simd2>::Type::new($z,$o))
            }

            pub fn x(&self) -> $t {
                self.0.get(0)
            }

            pub fn y(&self) -> $t {
                self.0.get(1)
            }

            pub fn set_x(&mut self,x: $t) {
                self.0.set(0,x);
            }

            pub fn set_y(&mut self,y: $t) {
                self.0.set(1,y);
            }
        }

        impl PartialEq for Vec2<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd2>::Type::eq(&self.0,&other.0,0x3)
            }
        }

        impl Zero for Vec2<$t> {
            fn zero() -> Self {
                Vec2(<$t as Simd2>::Type::zero())
            }
        }

        impl Display for Vec2<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({},{})",self.x(),self.y())
            }
        }

        impl Add<Vec2<$t>> for Vec2<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Vec2(<$t as Simd2>::Type::add(&self.0,&other.0))
            }
        }

        impl AddAssign<Vec2<$t>> for Vec2<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd2>::Type::add(&self.0,&other.0);
            }
        }

        impl Sub<Vec2<$t>> for Vec2<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Vec2(<$t as Simd2>::Type::sub(&self.0,&other.0))
            }
        }

        impl SubAssign<Vec2<$t>> for Vec2<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd2>::Type::sub(&self.0,&other.0);
            }
        }

        impl Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Vec2<$t> {
                Vec2(<$t as Simd2>::Type::mul(&<$t as Simd2>::Type::splat(self),&other.0))
            }
        }

        impl Mul<$t> for Vec2<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Vec2(<$t as Simd2>::Type::mul(&self.0,&<$t as Simd2>::Type::splat(other)))
            }
        }
        
        impl MulAssign<$t> for Vec2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::mul(&self.0,&<$t as Simd2>::Type::splat(other));
            }
        }        

        impl Div<$t> for Vec2<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Vec2(<$t as Simd2>::Type::div(&self.0,&<$t as Simd2>::Type::splat(other)))
            }
        }
        
        impl DivAssign<$t> for Vec2<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::div(&self.0,&<$t as Simd2>::Type::splat(other));
            }
        }
    }
}

macro_rules! impl_vec2i {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec2u!($t; $o; $z);

        impl Neg for Vec2<$t> {
            type Output = Self;
            fn neg(self) -> Self {
                Vec2(<$t as Simd2>::Type::sub(&<$t as Simd2>::Type::zero(),&self.0))
            }
        }
    }
}

macro_rules! impl_vec2f {
    ($t:ty; $o:expr; $z:expr) => {
        impl_vec2i!($t; $o; $z);

        impl Vec2<$t> {
            pub fn dot(_a: &Self,_b: &Self) -> $t {
                // TODO: a.x * b.x + a.y * b.y
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

impl_vec2u!(u8; 1; 0);
impl_vec2i!(i8; 1; 0);
impl_vec2u!(u16; 1; 0);
impl_vec2i!(i16; 1; 0);
impl_vec2u!(u32; 1; 0);
impl_vec2i!(i32; 1; 0);
impl_vec2u!(u64; 1; 0);
impl_vec2i!(i64; 1; 0);
impl_vec2u!(usize; 1; 0);
impl_vec2i!(isize; 1; 0);
impl_vec2f!(f32; 1.0; 0.0);
impl_vec2f!(f64; 1.0; 0.0);

//#[macro_export]
//macro_rules! vec2 {
//    ($x:expr,$y:expr) => { Vec2::new($x,$y) };
//}