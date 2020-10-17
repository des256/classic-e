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
pub struct Complex<T: FloatNumber> {
    pub r: T,
    pub i: T,
}

impl<T: FloatNumber> Complex<T> {
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
        Complex {
            r: r,
            i: i,
        }
    }
}

// Complex == Complex
impl<T: FloatNumber> PartialEq for Complex<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.r == other.r) &&
        (self.i == other.i)
    }
}

impl<T: FloatNumber> Zero for Complex<T> {
    fn zero() -> Self {
        Complex {
            r: T::zero(),
            i: T::zero(),
        }
    }
}

impl<T: FloatNumber> Display for Complex<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i < T::zero() {
            format!("{}i",self.i)
        } else {
            format!("+{}i",self.i)
        };
        write!(f,"{}{}",self.r,si)
    }
}

// s + Complex
macro_rules! scalar_complex_mul {
    ($t:ty) => {
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self + other.r,
                    i: other.i,
                }
            }
        }
    }
}

scalar_complex_mul!(f32);
scalar_complex_mul!(f64);

// Complex + s
impl<T: FloatNumber> Add<T> for Complex<T> {
    type Output = Self;
    fn add(self,other: T) -> Self {
        Complex::new(self.r + other,self.i)
    }
}

// Complex + Complex
impl<T: FloatNumber> Add<Complex<T>> for Complex<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

// Complex += s
impl<T: FloatNumber> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self,other: T) {
        self.r += other;
    }
}

// Complex += Complex
impl<T: FloatNumber> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self,other: Self) {
        self.r += other.r;
        self.i += other.i;
    }
}

// s - Complex
macro_rules! scalar_complex_sub {
    ($t:ty) => {
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self - other.r,
                    i: -other.i,
                }
            }
        }        
    }
}

scalar_complex_sub!(f32);
scalar_complex_sub!(f64);

// Complex - s
impl<T: FloatNumber> Sub<T> for Complex<T> {
    type Output = Self;
    fn sub(self,other: T) -> Self {
        Complex {
            r: self.r - other,
            i: self.i,
        }
    }
}

// Complex - Complex
impl<T: FloatNumber> Sub<Complex<T>> for Complex<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}

// Complex -= s
impl<T: FloatNumber> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self,other: T) {
        self.r -= other;
    }
}

// Complex -= Complex
impl<T: FloatNumber> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self,other: Self) {
        self.r -= other.r;
        self.i -= other.i;
    }
}

// s * Complex
macro_rules! scalar_complex_mul {
    ($t:ty) => {
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self,other: Complex<$t>) -> Complex<$t> {
                Complex {
                    r: self * other.r,
                    i: self * other.i,
                }
            }
        }        
    }
}

scalar_complex_mul!(f32);
scalar_complex_mul!(f64);

// Complex * s
impl<T: FloatNumber> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Complex {
            r: self.r * other,
            i: self.i * other,
        }
    }
}

// Complex * Complex
impl<T: FloatNumber> Mul<Complex<T>> for Complex<T> {
    type Output = Self;
    fn mul(self,other: Self) -> Self {
        Complex {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        }
    }
}

// Complex *= s
impl<T: FloatNumber> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self,other: T) {
        self.r *= other;
        self.i *= other;
    }
} 

// Complex *= Complex
impl<T: FloatNumber> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self,other: Complex<T>) {
        let r = self.r * other.r - self.i * other.i;
        let i = self.r * other.i + self.i * other.r;
        self.r = r;
        self.i = i;
    }
}

// Complex / s
impl<T: FloatNumber> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Complex {
            r: self.r / other,
            i: self.i / other,
        }
    }
}

// TODO: Complex / Complex

// Complex /= s
impl<T: FloatNumber> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self,other: T) {
        self.r /= other;
        self.i /= other;
    }
}

// TODO: Complex /= Complex

// -Complex
impl<T: FloatNumber> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

#[macro_export]
macro_rules! complex {
    ($r:expr,$i:expr) => { Complex::new($r,$i) };
}
