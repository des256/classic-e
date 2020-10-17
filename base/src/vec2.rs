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

/// 2D Vector.
#[derive(Copy,Clone,Debug)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Number> Vec2<T> {
    /// Create new vector.
    ///
    /// **Arguments**
    ///
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(x: T,y: T) -> Self {
        Vec2 { x: x,y: y, }
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec2 { x: T::one(),y: T::zero(), }
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec2 { x: T::zero(),y: T::one(), }
    }
}

impl<T: Number> PartialEq for Vec2<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y)
    }
}

impl<T: Number> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2 { x: T::zero(),y: T::zero(), }
    }
}

impl<T: Number> Display for Vec2<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{})",self.x,self.y)
    }
}

impl<T: Number> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec2 { x: self.x + other.x,y: self.y + other.y, }
    }
}

impl<T: Number> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Number> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec2 { x: self.x - other.x,y: self.y - other.y, }
    }
}

impl<T: Number> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

macro_rules! scalar_vec2_mul {
    ($t:ty) => {
        impl Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Vec2<$t> {
                Vec2 { x: self * other.x,y: self * other.y, }
            }
        }
    }
}

scalar_vec2_mul!(u8);
scalar_vec2_mul!(i8);
scalar_vec2_mul!(u16);
scalar_vec2_mul!(i16);
scalar_vec2_mul!(u32);
scalar_vec2_mul!(i32);
scalar_vec2_mul!(u64);
scalar_vec2_mul!(i64);
scalar_vec2_mul!(f32);
scalar_vec2_mul!(f64);
scalar_vec2_mul!(usize);
scalar_vec2_mul!(isize);

impl<T: Number> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec2 { x: self.x * other,y: self.y * other, }
    }
}
    
impl<T: Number> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
    }
}        

impl<T: Number> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec2 { x: self.x / other,y: self.y / other, }
    }
}
    
impl<T: Number> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T: Number + Neg<Output=T>> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 { x: -self.x,y: -self.y, }
    }
}

macro_rules! vec2_float {
    ($t:ty) => {
        impl Vec2<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                a.x * b.x + a.y * b.y
            }
        
            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        
            pub fn norm(&self) -> Self {
                let d = self.abs();
                if d != <$t>::zero() {
                    *self / d
                }
                else {
                    *self
                }
            }
        }        
    }
}

vec2_float!(f32);
vec2_float!(f64);

#[macro_export]
macro_rules! vec2 {
    ($x:expr,$y:expr) => { Vec2::new($x,$y) };
}
