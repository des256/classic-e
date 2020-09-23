// E - Complex
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
pub struct Complex<T: Simd2>(pub <T as Simd2>::Type);

macro_rules! impl_complex {
    ($t:ty) => {
        impl Complex<$t> {
            pub fn new(r: $t,i: $t) -> Complex<$t> {
                Complex(<$t as Simd2>::Type::new(r,i))
            }

            pub fn r(&self) -> $t {
                self.0.get(0)
            }

            pub fn i(&self) -> $t {
                self.0.get(1)
            }

            pub fn set_r(&mut self,r: $t) {
                self.0.set(0,r);
            }

            pub fn set_i(&mut self,i: $t) {
                self.0.set(1,i);
            }
        }

        // Complex == Complex
        impl PartialEq for Complex<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd2>::Type::eq(&self.0,&other.0,0x3)
            }
        }

        impl Zero for Complex<$t> {
            fn zero() -> Self {
                Complex(<$t as Simd2>::Type::zero())
            }
        }

        impl Display for Complex<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                let si = if self.i() < <$t>::zero() {
                    format!("{}i",self.i())
                } else {
                    format!("+{}i",self.i())
                };
                write!(f,"{}{}",self.r(),si)
            }
        }

        // s + Complex
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Complex<$t> {
                Complex(<$t as Simd2>::Type::new(self + other.r(),other.i()))
            }
        }

        // Complex + s
        impl Add<$t> for Complex<$t> {
            type Output = Self;
            fn add(self,other: $t) -> Self {
                Complex(<$t as Simd2>::Type::new(self.r() + other,self.i()))
            }
        }

        // Complex + Complex
        impl Add<Complex<$t>> for Complex<$t> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                Complex(<$t as Simd2>::Type::add(&self.0,&other.0))
            }
        }

        // Complex += s
        impl AddAssign<$t> for Complex<$t> {
            fn add_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::new(self.r() + other,self.i());
            }
        }

        // Complex += Complex
        impl AddAssign<Complex<$t>> for Complex<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd2>::Type::add(&self.0,&other.0);
            }
        }

        // s - Complex
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Complex<$t> {
                Complex(<$t as Simd2>::Type::new(self - other.r(),-other.i()))
            }
        }

        // Complex - s
        impl Sub<$t> for Complex<$t> {
            type Output = Self;
            fn sub(self,other: $t) -> Self {
                Complex(<$t as Simd2>::Type::new(self.r() - other,self.i()))
            }
        }

        // Complex - Complex
        impl Sub<Complex<$t>> for Complex<$t> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                Complex(<$t as Simd2>::Type::sub(&self.0,&other.0))
            }
        }

        // Complex -= s
        impl SubAssign<$t> for Complex<$t> {
            fn sub_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::new(self.r() - other,self.i())
            }
        }

        // Complex -= Complex
        impl SubAssign<Complex<$t>> for Complex<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd2>::Type::sub(&self.0,&other.0);
            }
        }

        // s * Complex
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Complex<$t> {
                Complex(<$t as Simd2>::Type::mul(&<$t as Simd2>::Type::splat(self),&other.0))
            }
        }

        // Complex * s
        impl Mul<$t> for Complex<$t> {
            type Output = Self;
            fn mul(self,other: $t) -> Self {
                Complex(<$t as Simd2>::Type::mul(&self.0,&<$t as Simd2>::Type::splat(other)))
            }
        }

        // Complex * Complex
        impl Mul<Complex<$t>> for Complex<$t> {
            type Output = Self;
            fn mul(self,other: Self) -> Self {
                Complex(<$t as Simd2>::Type::new(
                    self.r() * other.r() - self.i() * other.i(),
                    self.r() * other.i() + self.i() * other.r()
                ))
            }
        }

        // Complex *= s
        impl MulAssign<$t> for Complex<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::mul(&self.0,&<$t as Simd2>::Type::splat(other));
            }
        } 

        // Complex *= Complex
        impl MulAssign<Complex<$t>> for Complex<$t> {
            fn mul_assign(&mut self,other: Complex<$t>) {
                self.0 = <$t as Simd2>::Type::new(
                    self.r() * other.r() - self.i() * other.i(),
                    self.r() * other.i() + self.i() * other.r()
                );
            }
        }

        // Complex / s
        impl Div<$t> for Complex<$t> {
            type Output = Self;
            fn div(self,other: $t) -> Self {
                Complex(<$t as Simd2>::Type::div(&self.0,&<$t as Simd2>::Type::splat(other)))
            }
        }

        // TODO: Complex / Complex

        // Complex /= s
        impl DivAssign<$t> for Complex<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd2>::Type::div(&self.0,&<$t as Simd2>::Type::splat(other));
            }
        }

        // TODO: Complex /= Complex

        // -Complex
        impl Neg for Complex<$t> {
            type Output = Complex<$t>;
            fn neg(self) -> Complex<$t> {
                Complex(<$t as Simd2>::Type::sub(&<$t as Simd2>::Type::zero(),&self.0))
            }
        }
    }
}

impl_complex!(f32);
impl_complex!(f64);

#[macro_export]
macro_rules! complex {
    ($t:ty: $r:expr,$i:expr) => { Complex::<$t>::new($r,$i) };
}
