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
pub struct Vec3<T: Simdable> {
    _x: T,
    _y: T,
    _z: T,
}

impl<T: Simdable> Vec3<T> {
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
        Vec3 { _x: x,_y: y,_z: z, }
    }

    /// Create new X-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_x() -> Self {
        Vec3 { _x: T::one(),_y: T::zero(),_z: T::zero(), }
    }

    /// Create new Y-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_y() -> Self {
        Vec3 { _x: T::zero(),_y: T::one(),_z: T::zero(), }
    }

    /// Create new Z-axis unit vector.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn unit_z() -> Self {
        Vec3 { _x: T::zero(),_y: T::zero(),_z: T::one(), }
    }

    /// Get X-coordinate.
    ///
    /// **Returns**
    ///
    /// The X-coordinate.
    pub fn x(&self) -> T {
        self._x
    }

    /// Get Y-coordinate.
    ///
    /// **Returns**
    ///
    /// The Y-coordinate.
    pub fn y(&self) -> T {
        self._y
    }

    /// Get Z-coordinate.
    ///
    /// **Returns**
    ///
    /// The Z-coordinate.
    pub fn z(&self) -> T {
        self._z
    }

    /// Set X-coordinate.
    ///
    /// **Arguments**
    ///
    /// `x` - New X-coordinate.
    pub fn set_x(&mut self,x: T) {
        self._x = x;
    }

    /// Set Y-coordinate.
    ///
    /// **Arguments**
    ///
    /// `y` - New Y-coordinate.
    pub fn set_y(&mut self,y: T) {
        self._y = y;
    }

    /// Set Z-coordinate.
    ///
    /// **Arguments**
    ///
    /// `z` - New Z-coordinate.
    pub fn set_z(&mut self,z: T) {
        self._z = z;
    }
}

// Vec3 == Vec3
impl<T: Simdable> PartialEq for Vec3<T> {
    fn eq(&self,other: &Self) -> bool {
        (self._x == other._x) &&
        (self._y == other._y) &&
        (self._z == other._z)
    }
}

impl<T: Simdable> Zero for Vec3<T> {
    fn zero() -> Self {
        Vec3 { _x: T::zero(),_y: T::zero(),_z: T::zero(), }
    }
}

impl<T: Simdable> Display for Vec3<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{})",self.x(),self.y(),self.z())
    }
}

// Vec3 + Vec3
impl<T: Simdable> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec3 { _x: self._x + other._x,_y: self._y + other._y,_z: self._z + other._z, }
    }
}

// Vec3 += Vec3
impl<T: Simdable> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self,other: Self) {
        self._x += other._x;
        self._y += other._y;
        self._z += other._z;
    }
}

// Vec3 - Vec3
impl<T: Simdable> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec3 { _x: self._x - other._x,_y: self._y - other._y,_z: self._z - other._z, }
    }
}

// Vec3 -= Vec3
impl<T: Simdable> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self,other: Self) {
        self._x -= other._x;
        self._y -= other._y;
        self._z -= other._z;
    }
}

// s * Vec3
macro_rules! scalar_vec3_mul {
    ($t:ty) => {
        impl Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Vec3<$t> {
                Vec3 { _x: self * other._x,_y: self * other._y,_z: self * other._z, }
            }
        }        
    }
}

scalar_vec3_mul!(f32);
scalar_vec3_mul!(f64);

// Vec3 * s
impl<T: Simdable> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec3 { _x: self._x * other,_y: self._y * other,_z: self._z * other, }
    }
}

// Vec3 *= s
impl<T: Simdable> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self,other: T) {
        self._x *= other;
        self._y *= other;
        self._z *= other;
    }
}        

// Vec3 / s
impl<T: Simdable> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec3 { _x: self._x / other,_y: self._y / other,_z: self._z / other, }
    }
}

// Vec3 /= s
impl<T: Simdable> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self,other: T) {
        self._x /= other;
        self._y /= other;
        self._z /= other;
    }
}

// Vec3 = (Vec2,0)
impl<T: Simdable> From<Vec2<T>> for Vec3<T> {
    fn from(v: Vec2<T>) -> Vec3<T> {
        Vec3 { _x: v.x(),_y: v.y(),_z: T::zero(), }
    }
}

// Vec3 = Vec3A
impl<T: Simdable> From<Vec3A<T>> for Vec3<T> {
    fn from(v: Vec3A<T>) -> Vec3<T> {
        Vec3 { _x: v.x(),_y: v.y(),_z: v.z(), }
    }
}

// -Vec3
impl<T: Simdable + Neg<Output=T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 { _x: -self._x,_y: -self._y,_z: -self._z, }
    }
}

macro_rules! vec3_float {
    ($t:ty) => {
        impl Vec3<$t> {
            pub fn dot(a: Self,b: Self) -> $t {
                a._x * b._x + a._y * b._y
            }
        
            pub fn abs(&self) -> $t {
                (self._x * self._x + self._y * self._y).sqrt()
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
macro_rules! vec3 {
    ($x:expr,$y:expr,$z:expr) => { Vec3::new($x,$y,$z) };
}