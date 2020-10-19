// E - 2x2 Matrix
// Desmond Germans, 2020

// Mat2x2<T> implements a 2x2 matrix.

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

/// 2x2 matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat2x2<T: FloatNumber> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T: FloatNumber> Mat2x2<T> {
    /// Create new matrix.
    ///
    /// **Arguments**
    ///
    /// * `xx` - XX-coordinate.
    /// * `xy` - XY-coordinate.
    /// * `yx` - YX-coordinate.
    /// * `yy` - YY-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(xx: T,xy: T,yx: T,yy: T) -> Self {
        Mat2x2 {
            x: vec2!(xx,xy),
            y: vec2!(yx,yy),
        }
    }

    /// Create unit matrix.
    /// 
    /// **Returns**
    ///
    /// The unit matrix.
    pub fn unit() -> Self {
        Mat2x2 {
            x: vec2!(T::one(),T::zero()),
            y: vec2!(T::zero(),T::one()),
        }
    }
}

impl<T: FloatNumber> PartialEq for Mat2x2<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x.x == other.x.x) && (self.x.y == other.x.y) &&
        (self.y.x == other.y.x) && (self.y.y == other.y.y)
    }
}

impl<T: FloatNumber> Zero for Mat2x2<T> {
    fn zero() -> Self {
        Mat2x2 {
            x: Vec2::<T>::zero(),
            y: Vec2::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> Display for Mat2x2<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{})",self.x,self.y)
    }
}

impl<T: FloatNumber> Add<Mat2x2<T>> for Mat2x2<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat2x2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: FloatNumber> AddAssign<Mat2x2<T>> for Mat2x2<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: FloatNumber> Sub<Mat2x2<T>> for Mat2x2<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat2x2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: FloatNumber> SubAssign<Mat2x2<T>> for Mat2x2<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: FloatNumber> Mul<T> for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn mul(self,other: T) -> Self::Output {
        Mat2x2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

macro_rules! scalar_mat2x2_mul {
    ($t:ty) => {
        impl Mul<Mat2x2<$t>> for $t {
            type Output = Mat2x2<$t>;
            fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: self * other.x,
                    y: self * other.y,
                }
            }
        }
    }
}

scalar_mat2x2_mul!(f32);
scalar_mat2x2_mul!(f64);

impl<T: FloatNumber> Mul<Vec2<T>> for Mat2x2<T> {
    type Output = Vec2<T>;
    fn mul(self,other: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x.x * other.x + self.y.x * other.y,
            y: self.x.y * other.x + self.y.y * other.y,
        }
    }
}

impl<T: FloatNumber> Mul<Mat2x2<T>> for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn mul(self,other: Mat2x2<T>) -> Self::Output {
        Mat2x2 {
            x: vec2!(
                self.x.x * other.x.x + self.y.x * other.x.y,
                self.x.y * other.x.x + self.y.y * other.x.y
            ),
            y: vec2!(
                self.x.x * other.y.x + self.y.x * other.y.y,
                self.x.y * other.y.x + self.y.y * other.y.y
            ),
        }
    }
}

impl<T: FloatNumber> Div<T> for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn div(self,other: T) -> Self::Output {
        Mat2x2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: FloatNumber> MulAssign<T> for Mat2x2<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: FloatNumber> MulAssign<Mat2x2<T>> for Vec2<T> {
    fn mul_assign(&mut self,other: Mat2x2<T>) {
        let x = self.x * other.x.x + self.y * other.y.x;
        let y = self.x * other.x.y + self.y * other.y.y;
        self.x = x;
        self.y = y;
    }
}

impl<T: FloatNumber> MulAssign<Mat2x2<T>> for Mat2x2<T> {
    fn mul_assign(&mut self,other: Mat2x2<T>) {
        let x = vec2!(
            self.x.x * other.x.x + self.y.x * other.x.y,
            self.x.y * other.x.x + self.y.y * other.x.y
        );
        let y = vec2!(
            self.x.x * other.y.x + self.y.x * other.y.y,
            self.x.y * other.y.x + self.y.y * other.y.y
        );
        self.x = x;
        self.y = y;
    }
}

impl<T: FloatNumber> DivAssign<T> for Mat2x2<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T: FloatNumber> Neg for Mat2x2<T> {
    type Output = Mat2x2<T>;
    fn neg(self) -> Self::Output {
        Mat2x2 {
            x: -self.x,
            y: -self.y,
        }
    }
}