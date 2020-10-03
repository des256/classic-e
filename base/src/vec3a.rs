// E - Vector
// Desmond Germans, 2020

// Vec3A<T> implements a 3D vector (aligned to 4 elements).

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

/// Aligned 3D vector.
///
/// The vector is based on underlying 4-component SIMD access, and the last
/// coordinate is not used. Use this for performance.
#[derive(Copy,Clone,Debug)]
pub struct Vec3A<T: Simdable>(Simd4<T>);

impl<T: Simdable> Vec3A<T> {
    /// Create new vector.
    ///
    /// **Arguments**
    ///
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    /// * `z` - Z-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(x: T,y: T,z: T) -> Self {
        Vec3A(Simd4::new([x,y,z,T::zero()]))
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec3A(Simd4::new([T::one(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec3A(Simd4::new([T::zero(),T::one(),T::zero(),T::zero()]))
    }

    /// Create new Z-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_z() -> Self {
        Vec3A(Simd4::new([T::zero(),T::zero(),T::one(),T::zero()]))
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
}

impl<T: Simdable> PartialEq for Vec3A<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd4::<T>::eq(&self.0,&other.0,0x7)
    }
}

impl<T: Simdable> Zero for Vec3A<T> {
    fn zero() -> Self {
        Vec3A(Simd4::<T>::zero())
    }
}

impl<T: Simdable> Display for Vec3A<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{})",self.x(),self.y(),self.z())
    }
}

impl<T: Simdable> Add<Vec3A<T>> for Vec3A<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec3A(Simd4::<T>::add(self.0,other.0))
    }
}

impl<T: Simdable> AddAssign<Vec3A<T>> for Vec3A<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::add(self.0,other.0);
    }
}

impl<T: Simdable> Sub<Vec3A<T>> for Vec3A<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec3A(Simd4::<T>::sub(self.0,other.0))
    }
}

impl<T: Simdable> SubAssign<Vec3A<T>> for Vec3A<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::sub(self.0,other.0);
    }
}

macro_rules! scalar_vec3a_mul {
    ($t:ty) => {
        impl Mul<Vec3A<$t>> for $t {
            type Output = Vec3A<$t>;
            fn mul(self,other: Vec3A<$t>) -> Vec3A<$t> {
                Vec3A(Simd4::<$t>::mul(Simd4::<$t>::splat(self),other.0))
            }
        }
    }
}

scalar_vec3a_mul!(u8);
scalar_vec3a_mul!(i8);
scalar_vec3a_mul!(u16);
scalar_vec3a_mul!(i16);
scalar_vec3a_mul!(u32);
scalar_vec3a_mul!(i32);
scalar_vec3a_mul!(u64);
scalar_vec3a_mul!(i64);
scalar_vec3a_mul!(f32);
scalar_vec3a_mul!(f64);
scalar_vec3a_mul!(usize);
scalar_vec3a_mul!(isize);

impl<T: Simdable> Mul<T> for Vec3A<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec3A(Simd4::<T>::mul(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> MulAssign<T> for Vec3A<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::mul(self.0,Simd4::splat(other));
    }
}        

impl<T: Simdable> Div<T> for Vec3A<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec3A(Simd4::<T>::div(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> DivAssign<T> for Vec3A<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::div(self.0,Simd4::splat(other));
    }
}

impl<T: Simdable> Neg for Vec3A<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3A(Simd4::<T>::sub(Simd4::zero(),self.0))
    }
}

macro_rules! vec3a_float {
    ($t:ty) => {
        impl Vec3A<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                Simd4::<$t>::dot(a.0,b.0,0x7)
            }
        
            pub fn abs(&self) -> $t {
                Simd4::dot(self.0,self.0,0x7).sqrt()
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

vec3a_float!(f32);
vec3a_float!(f64);

#[macro_export]
macro_rules! vec3a {
    ($x:expr,$y:expr,$z:expr) => { Vec3A::new($x,$y,$z) };
}
