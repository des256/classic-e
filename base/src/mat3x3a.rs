// E - 3x3 Aligned matrix
// Desmond Germans, 2020

// Mat3x3A<T> implements a 3x3 matrix (aligned).

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

/// 3x3 Aligned matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat3x3A<T: FloatNumber> {
    pub x: Vec3A<T>,
    pub y: Vec3A<T>,
    pub z: Vec3A<T>,
    _w: Vec3A<T>,
}

impl<T: FloatNumber> Mat3x3A<T> {
    /// Create new matrix.
    ///
    /// **Arguments**
    ///
    /// * `xx` - XX-coordinate.
    /// * `xy` - XY-coordinate.
    /// * `xz` - XZ-coordinate.
    /// * `yx` - YX-coordinate.
    /// * `yy` - YY-coordinate.
    /// * `yz` - YZ-coordinate.
    /// * `zx` - ZX-coordinate.
    /// * `zy` - ZY-coordinate.
    /// * `zz` - ZZ-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(xx: T,xy: T,xz: T,yx: T,yy: T,yz: T,zx: T,zy: T,zz: T) -> Self {
        Mat3x3A {
            x: vec3!(xx,xy,xz),
            y: vec3!(yx,yy,yz),
            z: vec3!(zx,zy,zz),
            _w: Vec3A::<T>::zero(),
        }
    }

    /// Create unit matrix.
    /// 
    /// **Returns**
    ///
    /// The unit matrix.
    pub fn unit() -> Self {
        Mat3x3A {
            x: vec3!(T::one(),T::zero(),T::zero()),
            y: vec3!(T::zero(),T::one(),T::zero()),
            z: vec3!(T::zero(),T::zero(),T::one()),
            _w: Vec3A::<T>::zero(),
        }
    }

    /// Create scale matrix.
    ///
    /// **Arguments**
    ///
    /// * `s` - Scale 3D vector.
    ///
    /// **Returns**
    ///
    /// The scale matrix.
    pub fn scale(s: Vec3A<T>) -> Mat3x3A<T> {
        Mat3x3A {
            x: vec3!(s.x,T::zero(),T::zero()),
            y: vec3!(T::zero(),s.y,T::zero()),
            z: vec3!(T::zero(),T::zero(),s.z),
            _w: Vec3A::<T>::zero(),
        }
    }

    /// Create rotation matrix from quaternion.
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
    /// The rotation matrix.
    pub fn rotate(q: Quat<T>) -> Mat3x3A<T> {
        let mut rr = q.r * q.r;
        let mut ii = q.i * q.i;
        let mut jj = q.j * q.j;
        let kk = q.k * q.k;
        let n = rr + ii + jj + kk;
        let s = if n != T::zero() {
            (T::one() + T::one()) / n
        }
        else {
            T::zero()
        };
        let kr = s * q.k * q.r;
        rr *= s;
        ii *= s;
        let ki = s * q.k * q.i;
        let ri = s * q.r * q.i;
        let ij = s * q.i * q.j;
        let kj = s * q.k * q.j;
        let rj = s * q.r * q.j;
        jj *= s;
        Mat3x3A {
            x: vec3!(T::one() - (ii + jj),ri - kj,rj + ki),
            y: vec3!(ri + kj,T::one() - (rr + jj),ij - kr),
            z: vec3!(rj - ki,ij + kr,T::one() - (rr + ii)),
            _w: Vec3A::<T>::zero(),
        }
    }

    /// Create normal transformation matrix corresponding to
    /// homogenous transformation.
    ///
    /// **Arguments**
    ///
    /// * `m` - Homogenous transformation matrix.
    ///
    /// **Returns**
    ///
    /// Normal transformation matrix.
    pub fn normal_from(m: Mat4x4<T>) -> Mat3x3A<T> {
        Mat3x3 {
            x: vec3!(m.x.x,m.x.y,m.x.z),
            y: vec3!(m.y.x,m.y.y,m.y.z),
            z: vec3!(m.z.x,m.z.y,m.z.z),
            _w: Vec3A::<T>::zero(),
        }.inverse().transpose()
    }

    /// Calculate determinant of 3x3-matrix.
    ///
    /// **Returns**
    ///
    /// Determinant of the matrix.
    pub fn det(&self) -> T {
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.x.y;
        let e = self.y.y;
        let f = self.z.y;
        let g = self.x.z;
        let h = self.y.z;
        let i = self.z.z;
        let cofa = e * i - f * h;
        let cofb = f * g - d * i;
        let cofc = d * h - e * g;
        a * cofa - b * cofb + c * cofc
    }

    /// Calculate inverse of 3x3-matrix.
    ///
    /// **Returns**
    ///
    /// Inverse matrix.
    pub fn inverse(&self) -> Mat3x3A<T> {
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.x.y;
        let e = self.y.y;
        let f = self.z.y;
        let g = self.x.z;
        let h = self.y.z;
        let i = self.z.z;
        let ma = e * i - f * h;
        let md = f * g - d * i;
        let mg = d * h - e * g;
        let nd = a * ma + b * md + c * mg;
        if nd != T::zero() {
            let mb = c * h - b * i;
            let mc = b * f - c * e;
            let me = a * i - c * g;
            let mf = c * d - a * f;
            let mh = b * g - a * h;
            let mi = a * e - b * d;
            Mat3x3A {
                x: vec3!(ma,md,mg),
                y: vec3!(mb,me,mh),
                z: vec3!(mc,mf,mi),
                _w: Vec3A::<T>::zero(),
            } / nd
        }
        else {
            Mat3x3A::unit()
        }
    }

    /// Calculate transpose of 3x3-matrix.
    pub fn transpose(&self) -> Mat3x3A<T> {
        Mat3x3A {
            x: vec3!(self.x.x,self.y.x,self.z.x),
            y: vec3!(self.x.y,self.y.y,self.z.y),
            z: vec3!(self.x.z,self.y.z,self.z.z),
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> PartialEq for Mat3x3A<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x.x == other.x.x) && (self.x.y == other.x.y) && (self.x.z == other.x.z) &&
        (self.y.x == other.y.x) && (self.y.y == other.y.y) && (self.y.z == other.y.z) &&
        (self.z.x == other.z.x) && (self.z.y == other.z.y) && (self.z.z == other.z.z)
    }
}

impl<T: FloatNumber> Zero for Mat3x3A<T> {
    fn zero() -> Self {
        Mat3x3A {
            x: Vec3A::<T>::zero(),
            y: Vec3A::<T>::zero(),
            z: Vec3A::<T>::zero(),
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> Display for Mat3x3A<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{})",self.x,self.y,self.z)
    }
}

impl<T: FloatNumber> Add<Mat3x3A<T>> for Mat3x3A<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat3x3A {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> AddAssign<Mat3x3A<T>> for Mat3x3A<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: FloatNumber> Sub<Mat3x3A<T>> for Mat3x3A<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat3x3A {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> SubAssign<Mat3x3A<T>> for Mat3x3A<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: FloatNumber> Mul<T> for Mat3x3A<T> {
    type Output = Mat3x3A<T>;
    fn mul(self,other: T) -> Self::Output {
        Mat3x3A {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            _w: Vec3A::<T>::zero(),
        }
    }
}

macro_rules! scalar_mat3x3a_mul {
    ($t:ty) => {
        impl Mul<Mat3x3A<$t>> for $t {
            type Output = Mat3x3A<$t>;
            fn mul(self,other: Mat3x3A<$t>) -> Self::Output {
                Mat3x3A {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    _w: Vec3A::<T>::zero(),
                }
            }
        }
    }
}

scalar_mat3x3_mul!(f32);
scalar_mat3x3_mul!(f64);

impl<T: FloatNumber> Mul<Vec3A<T>> for Mat3x3A<T> {
    type Output = Vec3A<T>;
    fn mul(self,other: Vec3A<T>) -> Self::Output {
        Vec3A {
            x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z,
            y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z,
            z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z,
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> Mul<Mat3x3A<T>> for Mat3x3A<T> {
    type Output = Mat3x3A<T>;
    fn mul(self,other: Mat3x3A<T>) -> Self::Output {
        Mat3x3A {
            x: vec3!(
                self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z,
                self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z,
                self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z
            ),
            y: vec3!(
                self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z,
                self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z,
                self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z
            ),
            z: vec3!(
                self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z,
                self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z,
                self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z
            ),
            _w: Vec3A::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> Div<T> for Mat3x3A<T> {
    type Output = Mat3x3A<T>;
    fn div(self,other: T) -> Self::Output {
        Mat3x3A {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: FloatNumber> MulAssign<T> for Mat3x3A<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: FloatNumber> MulAssign<Mat3x3A<T>> for Vec3A<T> {
    fn mul_assign(&mut self,other: Mat3x3A<T>) {
        let x = self.x * other.x.x + self.y * other.y.x + self.z * other.z.x;
        let y = self.x * other.x.y + self.y * other.y.y + self.z * other.z.y;
        let z = self.x * other.x.z + self.y * other.y.z + self.z * other.z.z;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl<T: FloatNumber> MulAssign<Mat3x3A<T>> for Mat3x3A<T> {
    fn mul_assign(&mut self,other: Mat3x3A<T>) {
        let x = vec3!(
            self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z,
            self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z,
            self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z
        );
        let y = vec3!(
            self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z,
            self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z,
            self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z
        );
        let z = vec3!(
            self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z,
            self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z,
            self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z
        );
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl<T: FloatNumber> DivAssign<T> for Mat3x3A<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T: FloatNumber> Neg for Mat3x3A<T> {
    type Output = Mat3x3A<T>;
    fn neg(self) -> Self::Output {
        Mat3x3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

macro_rules! mat3x3a_rotations {
    ($t:ty) => {
        impl Mat3x3A<$t> {
            /// Create pitch rotation matrix (rotation in YZ-plane).
            ///
            /// **Arguments**
            ///
            /// * `a` - Angle (in radians).
            ///
            /// **Returns**
            /// 
            /// The rotation matrix.
            pub fn pitch(a: $t) -> Mat3x3A<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3A {
                    x: vec3!(1.0,0.0,0.0),
                    y: vec3!(0.0,ca,sa),
                    z: vec3!(0.0,-sa,ca),
                    _w: Vec3A::<T>::zero(),
                }
            }

            /// Create yaw rotation matrix (rotation in XZ-plane).
            ///
            /// **Arguments**
            ///
            /// * `a` - Angle (in radians).
            ///
            /// **Returns**
            /// 
            /// The rotation matrix.
            pub fn yaw(a: $t) -> Mat3x3A<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3A {
                    x: vec3!(ca,0.0,-sa),
                    y: vec3!(0.0,1.0,0.0),
                    z: vec3!(sa,0.0,ca),
                    _w: Vec3A::<T>::zero(),
                }
            }

            /// Create roll rotation matrix (rotation in XY-plane).
            ///
            /// **Arguments**
            ///
            /// * `a` - Angle (in radians).
            ///
            /// **Returns**
            /// 
            /// The rotation matrix.
            pub fn roll(a: $t) -> Mat3x3A<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3A {
                    x: vec3!(ca,sa,0.0),
                    y: vec3!(-sa,ca,0.0),
                    z: vec3!(0.0,0.0,1.0),
                    _w: Vec3A::<T>::zero(),
                }
            }
        }
    }
}

mat3x3a_rotations!(f32);
mat3x3a_rotations!(f64);
