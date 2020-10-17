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
pub struct Vec3A<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    _w: T,
}

impl<T: Number> Vec3A<T> {
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
        Vec3A { x: x,y: y,z: z,_w: T::zero(), }
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec3A { x: T::one(),y: T::zero(),z: T::zero(),_w: T::zero(), }
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec3A { x: T::zero(),y: T::one(),z: T::zero(),_w: T::zero(), }
    }

    /// Create new Z-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_z() -> Self {
        Vec3A { x: T::zero(),y: T::zero(),z: T::one(),_w: T::zero(), }
    }
}

impl<T: Number> PartialEq for Vec3A<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z)
    }
}

impl<T: Number> Zero for Vec3A<T> {
    fn zero() -> Self {
        Vec3A { x: T::zero(),y: T::zero(),z: T::zero(),_w: T::zero(), }
    }
}

impl<T: Number> Display for Vec3A<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{})",self.x,self.y,self.z)
    }
}

impl<T: Number> Add<Vec3A<T>> for Vec3A<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec3A { x: self.x + other.x,y: self.y + other.y,z: self.z + other.z,_w: T::zero(), }
    }
}

impl<T: Number> AddAssign<Vec3A<T>> for Vec3A<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Number> Sub<Vec3A<T>> for Vec3A<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec3A { x: self.x - other.x,y: self.y - other.y,z: self.z - other.z,_w: T::zero(), }
    }
}

impl<T: Number> SubAssign<Vec3A<T>> for Vec3A<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

macro_rules! scalar_vec3a_mul {
    ($t:ty) => {
        impl Mul<Vec3A<$t>> for $t {
            type Output = Vec3A<$t>;
            fn mul(self,other: Vec3A<$t>) -> Vec3A<$t> {
                Vec3A { x: self * other.x,y: self * other.y,z: self * other.z,_w: <$t>::zero(), }
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

impl<T: Number> Mul<T> for Vec3A<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec3A { x: self.x * other,y: self.y * other,z: self.z * other,_w: T::zero(), }
    }
}
    
impl<T: Number> MulAssign<T> for Vec3A<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}        

impl<T: Number> Div<T> for Vec3A<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec3A { x: self.x / other,y: self.y / other,z: self.z / other,_w: T::zero(), }
    }
}
    
impl<T: Number> DivAssign<T> for Vec3A<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T: Number + Neg<Output=T>> Neg for Vec3A<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3A { x: -self.x,y: -self.y,z: -self.z,_w: T::zero(), }
    }
}

macro_rules! vec3a_float {
    ($t:ty) => {
        impl Vec3A<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }
        
            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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
