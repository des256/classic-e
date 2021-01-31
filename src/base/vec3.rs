// E - Vector
// Desmond Germans, 2020

// Vec3<T> implements a 3D vector (packed).

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

/// Packed 3D vector.
///
/// The vector is exactly as big as the 3 components packed together, and not
/// based on underlying SIMD access. Use this for storage, not for
/// performance.
#[derive(Copy,Clone,Debug)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Number> Vec3<T> {
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
        Vec3 { x: x,y: y,z: z, }
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec3 { x: T::one(),y: T::zero(),z: T::zero(), }
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec3 { x: T::zero(),y: T::one(),z: T::zero(), }
    }

    /// Create new Z-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_z() -> Self {
        Vec3 { x: T::zero(),y: T::zero(),z: T::one(), }
    }
}

// Vec3 == Vec3
impl<T: Number> PartialEq for Vec3<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z)
    }
}

impl<T: Number> Zero for Vec3<T> {
    fn zero() -> Self {
        Vec3 { x: T::zero(),y: T::zero(),z: T::zero(), }
    }
}

impl<T: Number> Display for Vec3<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{})",self.x,self.y,self.z)
    }
}

// Vec3 + Vec3
impl<T: Number> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec3 { x: self.x + other.x,y: self.y + other.y,z: self.z + other.z, }
    }
}

// Vec3 += Vec3
impl<T: Number> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vec3 - Vec3
impl<T: Number> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec3 { x: self.x - other.x,y: self.y - other.y,z: self.z - other.z, }
    }
}

// Vec3 -= Vec3
impl<T: Number> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// s * Vec3
macro_rules! scalar_vec3_mul {
    ($t:ty) => {
        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Vec3<$t> {
                Vec3 { x: self * other.x,y: self * other.y,z: self * other.z, }
            }
        }        
    }
}

scalar_vec3_mul!(f32);
scalar_vec3_mul!(f64);

// Vec3 * s
impl<T: Number> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec3 { x: self.x * other,y: self.y * other,z: self.z * other, }
    }
}

// Vec3 *= s
impl<T: Number> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}        

// Vec3 / s
impl<T: Number> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec3 { x: self.x / other,y: self.y / other,z: self.z / other, }
    }
}

// Vec3 /= s
impl<T: Number> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

// Vec3 = (Vec2,0)
impl<T: Number> From<Vec2<T>> for Vec3<T> {
    fn from(v: Vec2<T>) -> Vec3<T> {
        Vec3 { x: v.x,y: v.y,z: T::zero(), }
    }
}

// Vec3 = Vec3A
impl<T: Number> From<Vec3A<T>> for Vec3<T> {
    fn from(v: Vec3A<T>) -> Vec3<T> {
        Vec3 { x: v.x,y: v.y,z: v.z, }
    }
}

// -Vec3
impl<T: Number + Neg<Output=T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 { x: -self.x,y: -self.y,z: -self.z, }
    }
}

macro_rules! vec3_float {
    ($t:ty) => {
        impl Vec3<$t> {
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

vec3_float!(f32);
vec3_float!(f64);

#[macro_export]
/// Create packed 3D vector.
macro_rules! vec3 {
    ($x:expr,$y:expr,$z:expr) => { Vec3::new($x,$y,$z) };
}