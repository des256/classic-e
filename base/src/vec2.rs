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
pub struct Vec2<T: Simdable>(Simd2<T>);

impl<T: Simdable> Vec2<T> {
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
        Vec2(Simd2::new([x,y]))
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec2(Simd2::new([T::one(),T::zero()]))
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec2(Simd2::new([T::zero(),T::one()]))
    }

    /// Get X-coordinate.
    ///
    /// **Returns**
    ///
    /// The X-coordinate.
    pub fn x(&self) -> T {
        self.0.get(0)
    }

    /// Get Y-coordinate.
    ///
    /// **Returns**
    ///
    /// The Y-coordinate.
    pub fn y(&self) -> T {
        self.0.get(1)
    }

    /// Set X-coordinate.
    ///
    /// **Arguments**
    ///
    /// `x` - New X-coordinate.
    pub fn set_x(&mut self,x: T) {
        self.0.set(0,x);
    }

    /// Set Y-coordinate.
    ///
    /// **Arguments**
    ///
    /// `y` - New Y-coordinate.
    pub fn set_y(&mut self,y: T) {
        self.0.set(1,y);
    }
}

impl<T: Simdable> PartialEq for Vec2<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd2::<T>::eq(&self.0,&other.0,0x3)
    }
}

impl<T: Simdable> Zero for Vec2<T> {
    fn zero() -> Self {
        Vec2(Simd2::<T>::zero())
    }
}

impl<T: Simdable> Display for Vec2<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{})",self.x(),self.y())
    }
}

impl<T: Simdable> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec2(Simd2::<T>::add(self.0,other.0))
    }
}

impl<T: Simdable> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd2::<T>::add(self.0,other.0);
    }
}

impl<T: Simdable> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec2(Simd2::<T>::sub(self.0,other.0))
    }
}

impl<T: Simdable> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd2::<T>::sub(self.0,other.0);
    }
}

macro_rules! scalar_vec2_mul {
    ($t:ty) => {
        impl Mul<Vec2<$t>> for $t {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Vec2<$t> {
                Vec2(Simd2::<$t>::mul(Simd2::<$t>::splat(self),other.0))
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

impl<T: Simdable> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec2(Simd2::<T>::mul(self.0,Simd2::splat(other)))
    }
}
    
impl<T: Simdable> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd2::<T>::mul(self.0,Simd2::splat(other));
    }
}        

impl<T: Simdable> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec2(Simd2::<T>::div(self.0,Simd2::splat(other)))
    }
}
    
impl<T: Simdable> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd2::<T>::div(self.0,Simd2::splat(other));
    }
}

impl<T: Simdable> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec2(Simd2::<T>::sub(Simd2::zero(),self.0))
    }
}

macro_rules! vec2_float {
    ($t:ty) => {
        impl Vec2<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                Simd2::<$t>::dot(a.0,b.0,0x3)
            }
        
            pub fn abs(&self) -> $t {
                Simd2::dot(self.0,self.0,0x3).sqrt()
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