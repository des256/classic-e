// E - Vector
// Desmond Germans, 2020

// Vec4<T> implements a 4D vector.

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
pub struct Vec4<T: Simdable>(Simd4<T>);

impl<T: Simdable> Vec4<T> {
    /// Create new vector.
    ///
    /// **Arguments**
    ///
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    /// * `z` - Z-coordinate.
    /// * `w` - W-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(x: T,y: T,z: T,w: T) -> Self {
        Vec4(Simd4::new([x,y,z,w]))
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec4(Simd4::new([T::one(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec4(Simd4::new([T::zero(),T::one(),T::zero(),T::zero()]))
    }

    /// Create new Z-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_z() -> Self {
        Vec4(Simd4::new([T::zero(),T::zero(),T::one(),T::zero()]))
    }

    /// Create new W-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_w() -> Self {
        Vec4(Simd4::new([T::zero(),T::zero(),T::zero(),T::one()]))
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

    /// Get Z-coordinate.
    ///
    /// **Returns**
    ///
    /// The Z-coordinate.
    pub fn z(&self) -> T {
        self.0.get(2)
    }

    /// Get W-coordinate.
    ///
    /// **Returns**
    ///
    /// The W-coordinate.
    pub fn w(&self) -> T {
        self.0.get(3)
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

    /// Set Z-coordinate.
    ///
    /// **Arguments**
    ///
    /// `z` - New Z-coordinate.
    pub fn set_z(&mut self,z: T) {
        self.0.set(2,z);
    }

    /// Set W-coordinate.
    ///
    /// **Arguments**
    ///
    /// `w` - New W-coordinate.
    pub fn set_w(&mut self,w: T) {
        self.0.set(3,w);
    }
}

impl<T: Simdable> PartialEq for Vec4<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd4::<T>::eq(&self.0,&other.0,0xF)
    }
}

impl<T: Simdable> Zero for Vec4<T> {
    fn zero() -> Self {
        Vec4(Simd4::<T>::zero())
    }
}

impl<T: Simdable> Display for Vec4<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{},{})",self.x(),self.y(),self.z(),self.w())
    }
}

impl<T: Simdable> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec4(Simd4::<T>::add(self.0,other.0))
    }
}

impl<T: Simdable> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::add(self.0,other.0);
    }
}

impl<T: Simdable> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec4(Simd4::<T>::sub(self.0,other.0))
    }
}

impl<T: Simdable> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::sub(self.0,other.0);
    }
}

macro_rules! scalar_vec4_mul {
    ($t:ty) => {
        impl Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Vec4<$t> {
                Vec4(Simd4::<$t>::mul(Simd4::<$t>::splat(self),other.0))
            }
        }
    }
}

scalar_vec4_mul!(u8);
scalar_vec4_mul!(i8);
scalar_vec4_mul!(u16);
scalar_vec4_mul!(i16);
scalar_vec4_mul!(u32);
scalar_vec4_mul!(i32);
scalar_vec4_mul!(u64);
scalar_vec4_mul!(i64);
scalar_vec4_mul!(f32);
scalar_vec4_mul!(f64);
scalar_vec4_mul!(usize);
scalar_vec4_mul!(isize);

impl<T: Simdable> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec4(Simd4::<T>::mul(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::mul(self.0,Simd4::splat(other));
    }
}        

impl<T: Simdable> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec4(Simd4::<T>::div(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::div(self.0,Simd4::splat(other));
    }
}

impl<T: Simdable> Neg for Vec4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec4(Simd4::<T>::sub(Simd4::zero(),self.0))
    }
}

macro_rules! vec4_float {
    ($t:ty) => {
        impl Vec4<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                Simd4::<$t>::dot(a.0,b.0,0xF)
            }
        
            pub fn abs(&self) -> $t {
                Simd4::dot(self.0,self.0,0xF).sqrt()
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

vec4_float!(f32);
vec4_float!(f64);

#[macro_export]
macro_rules! vec4 {
    ($x:expr,$y:expr,$z:expr,$w:expr) => { Vec4::new($x,$y,$z,$w) };
}
