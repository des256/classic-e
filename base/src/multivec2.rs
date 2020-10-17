// E - 2D Multivector
// Desmond Germans, 2020

use crate::*;
use std::{
    cmp::PartialEq,
    fmt::{
        Display,
        Formatter,
        Debug,
        Result,
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
        Neg,
    },
};

/// 2D multivector.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec2<T: FloatNumber> {
    pub r: T,
    pub x: T,pub y: T,
    pub xy: T,
}

impl<T: FloatNumber> MultiVec2<T> {
    /// Create new 2D multivector.
    ///
    /// **Arguments**
    ///
    /// * `r` - Scalar component.
    /// * `x` - X-vector component.
    /// * `y` - Y-vector component.
    /// * `xy` - Pseudoscalar component.
    pub fn new(
        r: T,
        x: T,y: T,
        xy: T
    ) -> Self {
        MultiVec2 {
            r: r,
            x: x,y: y,
            xy: xy,
        }
    }

    /// Create new multivector containing a unit scalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_r() -> Self {
        MultiVec2 {
            r: T::one(),
            x: T::zero(),y: T::zero(),
            xy: T::zero(),
        }
    }

    /// Create new multivector containing a unit X-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_x() -> Self {
        MultiVec2 {
            r: T::zero(),
            x: T::one(),y: T::zero(),
            xy: T::zero(),
        }
    }

    /// Create new multivector containing a unit Y-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_y() -> Self {
        MultiVec2 {
            r: T::zero(),
            x: T::zero(),y: T::one(),
            xy: T::zero(),
        }
    }

    /// Create new multivector containing a unit pseudoscalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xy() -> Self {
        MultiVec2 {
            r: T::zero(),
            x: T::zero(),y: T::zero(),
            xy: T::one(),
        }
    }
}

impl<T: FloatNumber> PartialEq for MultiVec2<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.r == other.r) &&
        (self.x == other.x) && (self.y == other.y) &&
        (self.xy == other.xy)
    }
}

impl<T: FloatNumber> Zero for MultiVec2<T> {
    /// Additive identity.
    fn zero() -> Self {
        MultiVec2 {
            r: T::zero(),
            x: T::zero(),y: T::zero(),
            xy: T::zero(),
        }
    }
}

impl<T: FloatNumber> Display for MultiVec2<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let sx = if self.x < T::zero() {
            format!("{}x",self.x)
        } else {
            format!("+{}x",self.x)
        };
        let sy = if self.y < T::zero() {
            format!("{}x",self.y)
        } else {
            format!("+{}x",self.y)
        };
        let sxy = if self.xy < T::zero() {
            format!("{}xy",self.xy)
        } else {
            format!("+{}xy",self.xy)
        };
        write!(f,"{}{}{}{}",self.r,sx,sy,sxy)
    }
}

impl<T: FloatNumber> Add<MultiVec2<T>> for MultiVec2<T> {
    type Output = Self;
    fn add(self,other: MultiVec2<T>) -> Self {
        MultiVec2 {
            r: self.r + other.r,
            x: self.x + other.x,y: self.y + other.y,
            xy: self.xy + other.xy,
        }
    }
}

impl<T: FloatNumber> AddAssign<MultiVec2<T>> for MultiVec2<T> {
    fn add_assign(&mut self,other: Self) {
        self.r += other.r;
        self.x += other.x; self.y += other.y;
        self.xy += other.xy;
    }
}

impl<T: FloatNumber> Sub<MultiVec2<T>> for MultiVec2<T> {
    type Output = Self;
    fn sub(self,other: MultiVec2<T>) -> Self {
        MultiVec2 {
            r: self.r - other.r,
            x: self.x - other.x,y: self.y - other.y,
            xy: self.xy - other.xy,
        }
    }
}

impl<T: FloatNumber> SubAssign<MultiVec2<T>> for MultiVec2<T> {
    fn sub_assign(&mut self,other: Self) {
        self.r -= other.r;
        self.x -= other.x; self.y -= other.y;
        self.xy -= other.xy;
    }
}

macro_rules! scalar_multivec2_mul {
    ($t:ty) => {
        impl Mul<MultiVec2<$t>> for $t {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> MultiVec2<$t> {
                MultiVec2 {
                    r: self * other.r,
                    x: self * other.x,y: self * other.y,
                    xy: self * other.xy,
                }
            }
        }        
    }
}

scalar_multivec2_mul!(f32);
scalar_multivec2_mul!(f64);

impl<T: FloatNumber> Mul<T> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn mul(self,other: T) -> Self {
        MultiVec2 {
            r: self.r * other,
            x: self.x * other,y: self.y * other,
            xy: self.xy * other,
        }
    }
}

impl<T: FloatNumber> MulAssign<T> for MultiVec2<T> {
    fn mul_assign(&mut self,other: T) {
        self.r *= other;
        self.x *= other; self.y *= other;
        self.xy *= other;
    }
}

impl<T: FloatNumber> Mul<MultiVec2<T>> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn mul(self,other: MultiVec2<T>) -> Self {
        MultiVec2 {
            r: self.r * other.r - self.x * other.x - self.y * other.y - self.xy * other.xy,
            x: self.r * other.x + self.x * other.r + self.y * other.xy - self.xy * other.y,
            y: self.r * other.y + self.y * other.r - self.x * other.xy + self.xy * other.x,
            xy: self.r * other.xy + self.xy * other.r + self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: FloatNumber> MulAssign<MultiVec2<T>> for MultiVec2<T> {
    fn mul_assign(&mut self,other: MultiVec2<T>) {
        let nr = self.r * other.r - self.x * other.x - self.y * other.y - self.xy * other.xy;
        let nx = self.r * other.x + self.x * other.r + self.y * other.xy - self.xy * other.y;
        let ny = self.r * other.y + self.y * other.r - self.x * other.xy + self.xy * other.x;
        let nxy = self.r * other.xy + self.xy * other.r + self.x * other.y - self.y * other.x;
        self.r = nr;
        self.x = nx; self.y = ny;
        self.xy = nxy;
    }
}

impl<T: FloatNumber> Div<T> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn div(self,other: T) -> Self {
        MultiVec2 {
            r: self.r / other,
            x: self.x / other,y: self.y / other,
            xy: self.xy / other,
        }
    }
}

impl<T: FloatNumber> DivAssign<T> for MultiVec2<T> {
    fn div_assign(&mut self,other: T) {
        self.r /= other;
        self.x /= other; self.y /= other;
        self.xy /= other;
    }
}

impl<T: FloatNumber + Neg<Output=T>> Neg for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn neg(self) -> MultiVec2<T> {
        MultiVec2 {
            r: -self.r,
            x: -self.x,y: -self.y,
            xy: -self.xy,
        }
    }
}

impl<T: FloatNumber> From<T> for MultiVec2<T> {
    fn from(v: T) -> MultiVec2<T> {
        MultiVec2::<T>::new(v,T::zero(),T::zero(),T::zero())
    }
}

impl<T: FloatNumber> From<Vec2<T>> for MultiVec2<T> {
    fn from(v: Vec2<T>) -> MultiVec2<T> {
        MultiVec2::<T>::new(T::zero(),v.x,v.y,T::zero())
    }
}

impl<T: FloatNumber> From<Complex<T>> for MultiVec2<T> {
    fn from(v: Complex<T>) -> MultiVec2<T> {
        MultiVec2::<T>::new(v.r,T::zero(),T::zero(),v.i)
    }
}
