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
pub struct Quat<T: FloatNumber> {
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: FloatNumber> Quat<T> {
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
        Quat {
            r: r,
            i: i,
            j: j,
            k: k,
        }
    }
}

impl<T: FloatNumber> PartialEq for Quat<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.r == other.r) &&
        (self.i == other.i) &&
        (self.j == other.j) &&
        (self.k == other.k)
    }
}

impl<T: FloatNumber> Zero for Quat<T> {
    fn zero() -> Self {
        Quat {
            r: T::zero(),
            i: T::zero(),
            j: T::zero(),
            k: T::zero(),
        }
    }
}

impl<T: FloatNumber> Display for Quat<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let si = if self.i < T::zero() {
            format!("{}i",self.i)
        } else {
            format!("+{}i",self.i)
        };
        let sj = if self.j < T::zero() {
            format!("{}j",self.j)
        } else {
            format!("+{}j",self.j)
        };
        let sk = if self.k < T::zero() {
            format!("{}k",self.k)
        } else {
            format!("+{}k",self.k)
        };
        write!(f,"{}{}{}{}",self.r,si,sj,sk)
    }
}

impl<T: FloatNumber> Add<Quat<T>> for Quat<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Quat {
            r: self.r + other.r,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T: FloatNumber> AddAssign<Quat<T>> for Quat<T> {
    fn add_assign(&mut self,other: Self) {
        self.r += other.r;
        self.i += other.i;
        self.j += other.j;
        self.k += other.k;
    }
}

impl<T: FloatNumber> Sub<Quat<T>> for Quat<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Quat {
            r: self.r - other.r,
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

impl<T: FloatNumber> SubAssign<Quat<T>> for Quat<T> {
    fn sub_assign(&mut self,other: Self) {
        self.r -= other.r;
        self.i -= other.i;
        self.j -= other.j;
        self.k -= other.k;
    }
}

macro_rules! scalar_quat_mul {
    ($t:ty) => {
        impl Mul<Quat<$t>> for $t {
            type Output = Quat<$t>;
            fn mul(self,other: Quat<$t>) -> Quat<$t> {
                Quat {
                    r: self * other.r,
                    i: self * other.i,
                    j: self * other.j,
                    k: self * other.k,
                }
            }
        }        
    }
}

scalar_quat_mul!(f32);
scalar_quat_mul!(f64);

impl<T: FloatNumber> Mul<T> for Quat<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Quat {
            r: self.r * other,
            i: self.i * other,
            j: self.j * other,
            k: self.k * other,
        }
    }
}

// TODO: Quat * Quat

impl<T: FloatNumber> MulAssign<T> for Quat<T> {
    fn mul_assign(&mut self,other: T) {
        self.r *= other;
        self.i *= other;
        self.j *= other;
        self.k *= other;
    }
} 

// TODO: Quat *= Quat

impl<T: FloatNumber> Div<T> for Quat<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Quat {
            r: self.r / other,
            i: self.i / other,
            j: self.j / other,
            k: self.k / other,
        }
    }
}

// TODO: Quat / Quat

impl<T: FloatNumber> DivAssign<T> for Quat<T> {
    fn div_assign(&mut self,other: T) {
        self.r /= other;
        self.i /= other;
        self.j /= other;
        self.k /= other;
    }
}

// TODO: Quat /= Quat

impl<T: FloatNumber> Neg for Quat<T> {
    type Output = Quat<T>;
    fn neg(self) -> Quat<T> {
        Quat {
            r: -self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

#[macro_export]
/// Create quaternion.
macro_rules! quat {
    ($r:expr,$i:expr,$j:expr,$k:expr) => { Quat::new($r,$i,$j,$k) };
}
