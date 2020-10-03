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

/// Complex number.
#[derive(Copy,Clone,Debug)]
pub struct Complex<T: SimdableFloat>(Simd2<T>);

impl<T: SimdableFloat> Complex<T> {
    /// Create new complex number.
    ///
    /// **Arguments**
    ///
    /// * `r` - Real component.
    /// * `i` - Imaginary component.
    ///
    /// **Returns**
    ///
    /// New complex number.
    pub fn new(r: T,i: T) -> Complex<T> {
        Complex(Simd2::new([r,i]))
    }

    /// Get real component.
    ///
    /// **Returns**
    ///
    /// The real component of the complex number.
    pub fn r(&self) -> T {
        self.0.get(0)
    }

    /// Get imaginary component.
    ///
    /// **Returns**
    ///
    /// The imaginary component of the complex number.
    pub fn i(&self) -> T {
        self.0.get(1)
    }

    /// Set real component.
    ///
    /// **Arguments**
    ///
    /// * `r` - New real component.
    pub fn set_r(&mut self,r: T) {
        self.0.set(0,r);
    }

    /// Set imaginary component.
    ///
    /// **Arguments**
    ///
    /// * `i` - New imaginary component.
    pub fn set_i(&mut self,i: T) {
        self.0.set(1,i);
    }
}

// Complex == Complex
impl<T: SimdableFloat> PartialEq for Complex<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd2::eq(&self.0,&other.0,0x3)
    }
}

impl<T: SimdableFloat> Zero for Complex<T> {
    fn zero() -> Self {
        Complex(Simd2::zero())
    }
}

impl<T: SimdableFloat> Display for Complex<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i() < T::zero() {
            format!("{}i",self.i())
        } else {
            format!("+{}i",self.i())
        };
        write!(f,"{}{}",self.r(),si)
    }
}

// s + Complex
macro_rules! scalar_complex_mul {
    ($t:ty) => {
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Complex<$t> {
                Complex::new(self + other.r(),other.i())
            }
        }
    }
}

scalar_complex_mul!(f32);
scalar_complex_mul!(f64);

// Complex + s
impl<T: SimdableFloat> Add<T> for Complex<T> {
    type Output = Self;
    fn add(self,other: T) -> Self {
        Complex::new(self.r() + other,self.i())
    }
}

// Complex + Complex
impl<T: SimdableFloat> Add<Complex<T>> for Complex<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Complex(Simd2::add(self.0,other.0))
    }
}

// Complex += s
impl<T: SimdableFloat> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self,other: T) {
        self.0 = Simd2::new([self.r() + other,self.i()]);
    }
}

// Complex += Complex
impl<T: SimdableFloat> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd2::add(self.0,other.0);
    }
}

// s - Complex
macro_rules! scalar_complex_sub {
    ($t:ty) => {
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Complex<$t> {
                Complex::new(self - other.r(),-other.i())
            }
        }        
    }
}

scalar_complex_sub!(f32);
scalar_complex_sub!(f64);

// Complex - s
impl<T: SimdableFloat> Sub<T> for Complex<T> {
    type Output = Self;
    fn sub(self,other: T) -> Self {
        Complex::new(self.r() - other,self.i())
    }
}

// Complex - Complex
impl<T: SimdableFloat> Sub<Complex<T>> for Complex<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Complex(Simd2::sub(self.0,other.0))
    }
}

// Complex -= s
impl<T: SimdableFloat> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self,other: T) {
        self.0 = Simd2::new([self.r() - other,self.i()])
    }
}

// Complex -= Complex
impl<T: SimdableFloat> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd2::sub(self.0,other.0);
    }
}

// s * Complex
macro_rules! scalar_complex_mul {
    ($t:ty) => {
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Complex<$t> {
                Complex(Simd2::mul(Simd2::splat(self),other.0))
            }
        }        
    }
}

scalar_complex_mul!(f32);
scalar_complex_mul!(f64);

// Complex * s
impl<T: SimdableFloat> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Complex(Simd2::mul(self.0,Simd2::splat(other)))
    }
}

// Complex * Complex
impl<T: SimdableFloat> Mul<Complex<T>> for Complex<T> {
    type Output = Self;
    fn mul(self,other: Self) -> Self {
        Complex::new(
            self.r() * other.r() - self.i() * other.i(),
            self.r() * other.i() + self.i() * other.r()
        )
    }
}

// Complex *= s
impl<T: SimdableFloat> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd2::mul(self.0,Simd2::splat(other));
    }
} 

// Complex *= Complex
impl<T: SimdableFloat> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self,other: Complex<T>) {
        self.0 = Simd2::new([
            self.r() * other.r() - self.i() * other.i(),
            self.r() * other.i() + self.i() * other.r()
        ]);
    }
}

// Complex / s
impl<T: SimdableFloat> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Complex(Simd2::div(self.0,Simd2::splat(other)))
    }
}

// TODO: Complex / Complex

// Complex /= s
impl<T: SimdableFloat> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd2::div(self.0,Simd2::splat(other));
    }
}

// TODO: Complex /= Complex

// -Complex
impl<T: SimdableFloat> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex(Simd2::sub(Simd2::zero(),self.0))
    }
}

#[macro_export]
macro_rules! complex {
    ($r:expr,$i:expr) => { Complex::new($r,$i) };
}
