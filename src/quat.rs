// E - Quaternion
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

#[derive(Copy,Clone,Debug)]
pub struct Quat<T: Simd4>(pub <T as Simd4>::Type);

macro_rules! impl_quat {
    ($t:ty) => {
        impl Quat<$t> {
            pub fn new(r: $t,i: $t,j: $t,k: $t) -> Quat<$t> {
                Quat(<$t as Simd4>::Type::new(r,i,j,k))
            }

            pub fn r(&self) -> $t {
                self.0.get(0)
            }

            pub fn i(&self) -> $t {
                self.0.get(1)
            }

            pub fn j(&self) -> $t {
                self.0.get(2)
            }

            pub fn k(&self) -> $t {
                self.0.get(3)
            }

            pub fn set_r(&mut self,r: $t) {
                self.0.set(0,r);
            }

            pub fn set_i(&mut self,i: $t) {
                self.0.set(1,i);
            }

            pub fn set_j(&mut self,j: $t) {
                self.0.set(2,j);
            }

            pub fn set_k(&mut self,k: $t) {
                self.0.set(3,k);
            }
        }

        // Quat == Quat
        impl PartialEq for Quat<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd4>::Type::eq(&self.0,&other.0,0xF)
            }
        }

        impl Zero for Quat<$t> {
            fn zero() -> Self {
                Quat(<$t as Simd4>::Type::zero())
            }
        }

        impl Display for Quat<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                let si = if self.i() < <$t>::zero() {
                    format!("{}i",self.i())
                } else {
                    format!("+{}i",self.i())
                };
                let sj = if self.j() < <$t>::zero() {
                    format!("{}j",self.j())
                } else {
                    format!("+{}j",self.j())
                };
                let sk = if self.k() < <$t>::zero() {
                    format!("{}k",self.k())
                } else {
                    format!("+{}k",self.k())
                };
                write!(f,"{}{}{}{}",self.r(),si,sj,sk)
            }
        }

        // Quat + Quat
        impl Add<Quat<$t>> for Quat<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Quat(<$t as Simd4>::Type::add(&self.0,&other.0))
            }
        }

        // Quat += Quat
        impl AddAssign<Quat<$t>> for Quat<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::add(&self.0,&other.0);
            }
        }

        // Quat - Quat
        impl Sub<Quat<$t>> for Quat<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Quat(<$t as Simd4>::Type::sub(&self.0,&other.0))
            }
        }

        // Quat -= Quat
        impl SubAssign<Quat<$t>> for Quat<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::sub(&self.0,&other.0);
            }
        }

        // s * Quat
        impl Mul<Quat<$t>> for $t {
            type Output = Quat<$t>;
            fn mul(self,other: Quat<$t>) -> Quat<$t> {
                Quat(<$t as Simd4>::Type::mul(&<$t as Simd4>::Type::splat(self),&other.0))
            }
        }

        // Quat * s
        impl Mul<$t> for Quat<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Quat(<$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }

        // TODO: Quat * Quat

        // Quat *= s
        impl MulAssign<$t> for Quat<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        } 

        // TODO: Quat *= Quat

        // Quat / s
        impl Div<$t> for Quat<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Quat(<$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }

        // TODO: Quat / Quat

        // Quat /= s
        impl DivAssign<$t> for Quat<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }

        // TODO: Quat /= Quat

        // -Quat
        impl Neg for Quat<$t> {
            type Output = Quat<$t>;
            fn neg(self) -> Quat<$t> {
                Quat(<$t as Simd4>::Type::sub(&<$t as Simd4>::Type::zero(),&self.0))
            }
        }
    }
}

impl_quat!(f32);
impl_quat!(f64);

#[macro_export]
macro_rules! quat {
    ($t:ty: $r:expr,$i:expr,$j:expr,$k:expr) => { Quat::<$t>::new($r,$i,$j,$k) };
}
