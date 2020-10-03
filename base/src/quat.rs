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

/// Quaternion.
#[derive(Copy,Clone,Debug)]
pub struct Quat<T: SimdableFloat>(Simd4<T>);

impl<T: SimdableFloat> Quat<T> {
    /// Create new quaternion.
    ///
    /// **Arguments**
    ///
    /// * `r` - Real component.
    /// * `i` - I-component.
    /// * `j` - J-component.
    /// * `k` - K-component.
    ///
    /// **Returns**
    ///
    /// New quaternion.
    pub fn new(r: T,i: T,j: T,k: T) -> Quat<T> {
        Quat(Simd4::new([r,i,j,k]))
    }

    /// Get real component.
    ///
    /// **Returns**
    ///
    /// The real component of the quaternion.
    pub fn r(&self) -> T {
        self.0.get(0)
    }

    /// Get I-component.
    ///
    /// **Returns**
    ///
    /// The I-component of the quaternion.
    pub fn i(&self) -> T {
        self.0.get(1)
    }

    /// Get J-component.
    ///
    /// **Returns**
    ///
    /// The J-component of the quaternion.
    pub fn j(&self) -> T {
        self.0.get(2)
    }

    /// Get K-component.
    ///
    /// **Returns**
    ///
    /// The K-component of the quaternion.
    pub fn k(&self) -> T {
        self.0.get(3)
    }

    /// Set real component.
    ///
    /// **Arguments**
    ///
    /// * `r` - New real component.
    pub fn set_r(&mut self,r: T) {
        self.0.set(0,r);
    }

    /// Set I-component.
    ///
    /// **Arguments**
    ///
    /// * `i` - New I-component.
    pub fn set_i(&mut self,i: T) {
        self.0.set(1,i);
    }

    /// Set J-component.
    ///
    /// **Arguments**
    ///
    /// * `j` - New J-component.
    pub fn set_j(&mut self,j: T) {
        self.0.set(2,j);
    }

    /// Set K-component.
    ///
    /// **Arguments**
    ///
    /// * `k` - New K-component.
    pub fn set_k(&mut self,k: T) {
        self.0.set(3,k);
    }
}

// Quat == Quat
impl<T: SimdableFloat> PartialEq for Quat<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd4::eq(&self.0,&other.0,0xF)
    }
}

impl<T: SimdableFloat> Zero for Quat<T> {
    fn zero() -> Self {
        Quat(Simd4::zero())
    }
}

impl<T: SimdableFloat> Display for Quat<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i() < T::zero() {
            format!("{}i",self.i())
        } else {
            format!("+{}i",self.i())
        };
        let sj = if self.j() < T::zero() {
            format!("{}j",self.j())
        } else {
            format!("+{}j",self.j())
        };
        let sk = if self.k() < T::zero() {
            format!("{}k",self.k())
        } else {
            format!("+{}k",self.k())
        };
        write!(f,"{}{}{}{}",self.r(),si,sj,sk)
    }
}

// Quat + Quat
impl<T: SimdableFloat> Add<Quat<T>> for Quat<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Quat(Simd4::add(self.0,other.0))
    }
}

// Quat += Quat
impl<T: SimdableFloat> AddAssign<Quat<T>> for Quat<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd4::add(self.0,other.0);
    }
}

// Quat - Quat
impl<T: SimdableFloat> Sub<Quat<T>> for Quat<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Quat(Simd4::sub(self.0,other.0))
    }
}

// Quat -= Quat
impl<T: SimdableFloat> SubAssign<Quat<T>> for Quat<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd4::sub(self.0,other.0);
    }
}

// s * Quat
macro_rules! scalar_quat_mul {
    ($t:ty) => {
        impl Mul<Quat<$t>> for $t {
            type Output = Quat<$t>;
            fn mul(self,other: Quat<$t>) -> Quat<$t> {
                Quat(Simd4::mul(Simd4::splat(self),other.0))
            }
        }        
    }
}

scalar_quat_mul!(f32);
scalar_quat_mul!(f64);

// Quat * s
impl<T: SimdableFloat> Mul<T> for Quat<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Quat(Simd4::mul(self.0,Simd4::splat(other)))
    }
}

// TODO: Quat * Quat

// Quat *= s
impl<T: SimdableFloat> MulAssign<T> for Quat<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd4::mul(self.0,Simd4::splat(other));
    }
} 

// TODO: Quat *= Quat

// Quat / s
impl<T: SimdableFloat> Div<T> for Quat<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Quat(Simd4::div(self.0,Simd4::splat(other)))
    }
}

// TODO: Quat / Quat

// Quat /= s
impl<T: SimdableFloat> DivAssign<T> for Quat<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd4::div(self.0,Simd4::splat(other));
    }
}

// TODO: Quat /= Quat

// -Quat
impl<T: SimdableFloat> Neg for Quat<T> {
    type Output = Quat<T>;
    fn neg(self) -> Quat<T> {
        Quat(Simd4::sub(Simd4::zero(),self.0))
    }
}

#[macro_export]
macro_rules! quat {
    ($r:expr,$i:expr,$j:expr,$k:expr) => { Quat::new($r,$i,$j,$k) };
}
