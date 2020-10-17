// E - 4x4 Matrix
// Desmond Germans, 2020

// Mat4x4<T> implements a 4x4 matrix.

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

/// 4x4 Matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat4x4<T: FloatNumber> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T: FloatNumber> Mat4x4<T> {
    /// Create new 4D matrix.
    ///
    /// **Arguments**
    ///
    /// * `xx` - XX-coordinate.
    /// * `xy` - XY-coordinate.
    /// * `xz` - XZ-coordinate.
    /// * `xw` - XW-coordinate.
    /// * `yx` - YX-coordinate.
    /// * `yy` - YY-coordinate.
    /// * `yz` - YZ-coordinate.
    /// * `yw` - YW-coordinate.
    /// * `zx` - ZX-coordinate.
    /// * `zy` - ZY-coordinate.
    /// * `zz` - ZZ-coordinate.
    /// * `zw` - ZW-coordinate.
    /// * `wx` - WX-coordinate.
    /// * `wy` - WY-coordinate.
    /// * `wz` - WZ-coordinate.
    /// * `ww` - WW-coordinate.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new(
        xx: T,xy: T,xz: T,xw: T,
        yx: T,yy: T,yz: T,yw: T,
        zx: T,zy: T,zz: T,zw: T,
        wx: T,wy: T,wz: T,ww: T,
    ) -> Self {
        Mat4x4 {
            x: vec4!(xx,xy,xz,xw),
            y: vec4!(yx,yy,yz,yw),
            z: vec4!(zx,zy,zz,zw),
            w: vec4!(wx,wy,wz,ww),
        }
    }

    /// Create new 4D matrix from vectors.
    ///
    /// **Arguments**
    ///
    /// * `x` - X-column.
    /// * `y` - Y-column.
    /// * `z` - Z-column.
    /// * `w` - W-column.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new_from_vecs(x: Vec4<T>,y: Vec4<T>,z: Vec4<T>,w: Vec4<T>) -> Self {
        Mat4x4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// Extend 2x2 matrix into 4x4 matrix.
    ///
    /// **Arguments**
    ///
    /// * `m` - 2x2 matrix.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new_from_mat2x2(m: Mat2x2<T>) -> Self {
        Mat4x4 {
            x: vec4!(m.x.x,m.x.y,T::zero(),T::zero()),
            y: vec4!(m.y.x,m.y.y,T::zero(),T::zero()),
            z: vec4!(T::zero(),T::zero(),T::one(),T::zero()),
            w: vec4!(T::zero(),T::zero(),T::zero(),T::one()),
        }
    }

    /// Extend 3x3 matrix into 4x4 matrix.
    ///
    /// **Arguments**
    ///
    /// * `m` - 3x3 matrix.
    ///
    /// **Returns**
    ///
    /// The new vector.
    pub fn new_from_mat3x3(m: Mat3x3<T>) -> Self {
        Mat4x4 {
            x: vec4!(m.x.x,m.x.y,m.x.z,T::zero()),
            y: vec4!(m.y.x,m.y.y,m.y.z,T::zero()),
            z: vec4!(m.z.x,m.z.y,m.z.z,T::zero()),
            w: vec4!(T::zero(),T::zero(),T::zero(),T::one()),
        }
    }

    /// Create unit matrix.
    /// 
    /// **Returns**
    ///
    /// The unit matrix.
    pub fn unit() -> Self {
        Mat4x4 {
            x: vec4!(T::one(),T::zero(),T::zero(),T::zero()),
            y: vec4!(T::zero(),T::one(),T::zero(),T::zero()),
            z: vec4!(T::zero(),T::zero(),T::one(),T::zero()),
            w: vec4!(T::zero(),T::zero(),T::zero(),T::one()),
        }
    }

    /// Calculate determinant of 4x4-matrix.
    ///
    /// **Returns**
    ///
    /// Determinant of the matrix.
    pub fn det(&self) -> T {
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.w.x;
        let e = self.x.y;
        let f = self.y.y;
        let g = self.z.y;
        let h = self.w.y;
        let i = self.x.z;
        let j = self.y.z;
        let k = self.z.z;
        let l = self.w.z;
        let m = self.x.w;
        let n = self.y.w;
        let o = self.z.w;
        let p = self.w.w;
        let kpol = k * p - o * l;
        let jpnl = j * p - n * l;
        let jonk = j * o - n * k;
        let ipml = i * p - m * l;
        let iomk = i * o - m * k;
        let inmj = i * n - m * j;
        let cofa = f * kpol - g * jpnl + h * jonk;
        let cofb = e * kpol - g * ipml + h * iomk;
        let cofc = e * jpnl - f * ipml + h * inmj;
        let cofd = e * jonk - f * iomk + g * inmj;
        a * cofa - b * cofb + c * cofc - d * cofd
}

    /// Calculate inverse of 4x4-matrix.
    ///
    /// **Returns**
    ///
    /// Inverse matrix.
    pub fn inverse(&self) -> Mat4x4<T> {
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.w.x;
        let e = self.x.y;
        let f = self.y.y;
        let g = self.z.y;
        let h = self.w.y;
        let i = self.x.z;
        let j = self.y.z;
        let k = self.z.z;
        let l = self.w.z;
        let m = self.x.w;
        let n = self.y.w;
        let o = self.z.w;
        let p = self.w.w;
        let kpol = k * p - o * l;
        let jpnl = j * p - n * l;
        let jonk = j * o - n * k;
        let ipml = i * p - m * l;
        let iomk = i * o - m * k;
        let inmj = i * n - m * j;
        let cofa = f * kpol - g * jpnl + h * jonk;
        let cofb = e * kpol - g * ipml + h * iomk;
        let cofc = e * jpnl - f * ipml + h * inmj;
        let cofd = e * jonk - f * iomk + g * inmj;
        let nd = a * cofa - b * cofb + c * cofc - d * cofd;
        if nd != T::zero() {
            let chgd = c * h - g * d;
            let bhfd = b * h - f * d;
            let bgfc = b * g - f * c;
            let ahed = a * h - e * d;
            let agec = a * g - e * c;
            let afeb = a * f - e * b;
            let cofe = b * kpol - c * jpnl + d * jonk;
            let coff = a * kpol - c * ipml + d * iomk;
            let cofg = a * jpnl - b * ipml + d * inmj;
            let cofh = a * jonk - b * iomk + c * inmj;
            let cofi = n * chgd - o * bhfd + p * bgfc;
            let cofj = m * chgd - o * ahed + p * agec;
            let cofk = m * bhfd - n * ahed + p * afeb;
            let cofl = m * bgfc - n * agec + o * afeb;
            let cofm = j * chgd - k * bhfd + l * bgfc;
            let cofn = i * chgd - k * ahed + l * agec;
            let cofo = i * bhfd - j * ahed + l * afeb;
            let cofp = i * bgfc - j * agec + k * afeb;
            Mat4x4 {
                x: vec4!(cofa,-cofb,cofc,-cofd),
                y: vec4!(-cofe,coff,-cofg,cofh),
                z: vec4!(cofi,-cofj,cofk,-cofl),
                w: vec4!(-cofm,cofn,-cofo,cofp),
            } / nd
        }
        else {
            Mat4x4::unit()
        }
    }

    /// Calculate transpose of 4x4-matrix.
    ///
    /// **Returns**
    ///
    /// Transpose matrix.
    pub fn transpose(&self) -> Mat4x4<T> {
        Mat4x4 {
            x: vec4!(self.x.x,self.y.x,self.z.x,self.w.x),
            y: vec4!(self.x.y,self.y.y,self.z.y,self.w.y),
            z: vec4!(self.x.z,self.y.z,self.z.z,self.w.z),
            w: vec4!(self.x.w,self.y.w,self.z.w,self.w.w),
        }
    }

    /// Create homogenous orthogonal projection matrix.
    ///
    /// **Arguments**
    /// * `l` - Left (minimum X).
    /// * `r` - Right (maximum X).
    /// * `b` - Bottom (minimum Y).
    /// * `t` - Top (maximum Y).
    /// * `n` - Near (minimum Z).
    /// * `f` - Far (maximum Z).
    ///
    /// **Returns**
    ///
    /// Homogenous orthogonal projection matrix.
    pub fn ortho(l: T,r: T,b: T,t: T,n: T,f: T) -> Mat4x4<T> {
        let dx = r - l;
        let dy = t - b;
        let dz = f - n;
        let rx = -(r + l) / dx;
        let ry = -(t + b) / dy;
        let rz = -(f + n) / dz;
        Mat4x4 {
            x: vec4!((T::one() + T::one()) / dx,T::zero(),T::zero(),T::zero()),
            y: vec4!(T::zero(),(T::one() + T::one()) / dy,T::zero(),T::zero()),
            z: vec4!(T::zero(),T::zero(),-(T::one() + T::one()) / dz,T::zero()),
            w: vec4!(rx,ry,rz,T::one()),
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
    pub fn scale(s: Vec4<T>) -> Mat4x4<T> {
        Mat4x4 {
            x: vec4!(s.x,T::zero(),T::zero(),T::zero()),
            y: vec4!(T::zero(),s.y,T::zero(),T::zero()),
            z: vec4!(T::zero(),T::zero(),s.z,T::zero()),
            w: vec4!(T::zero(),T::zero(),T::zero(),s.w),
        }
    }

    /// Create homogenous rotation matrix.
    ///
    /// **Arguments**
    /// * `q` - Quaternion.
    ///
    /// **Returns**
    ///
    /// Homogenous rotation matrix.
    pub fn rotate(q: Quat<T>) -> Mat4x4<T> {
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
        Mat4x4 {
            x: vec4!(T::one() - (ii + jj),ri - kj,rj + ki,T::zero()),
            y: vec4!(ri + kj,T::one() - (rr + jj),ij - kr,T::zero()),
            z: vec4!(rj - ki,ij + kr,T::one() - (rr + ii),T::zero()),
            w: vec4!(T::zero(),T::zero(),T::zero(),T::one())
        }
    }
}

impl<T: FloatNumber> PartialEq for Mat4x4<T> {
    fn eq(&self,other: &Self) -> bool {
        (self.x.x == other.x.x) && (self.x.y == other.x.y) && (self.x.z == other.x.z) && (self.x.w == other.x.w) &&
        (self.y.x == other.y.x) && (self.y.y == other.y.y) && (self.y.z == other.y.z) && (self.y.w == other.y.w) &&
        (self.z.x == other.z.x) && (self.z.y == other.z.y) && (self.z.z == other.z.z) && (self.z.w == other.z.w) &&
        (self.w.x == other.w.x) && (self.w.y == other.w.y) && (self.w.z == other.w.z) && (self.w.w == other.w.w)
    }
}

impl<T: FloatNumber> Zero for Mat4x4<T> {
    fn zero() -> Self {
        Mat4x4 {
            x: Vec4::<T>::zero(),
            y: Vec4::<T>::zero(),
            z: Vec4::<T>::zero(),
            w: Vec4::<T>::zero(),
        }
    }
}

impl<T: FloatNumber> Display for Mat4x4<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
    }
}

impl<T: FloatNumber> Add<Mat4x4<T>> for Mat4x4<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Mat4x4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: FloatNumber> AddAssign<Mat4x4<T>> for Mat4x4<T> {
    fn add_assign(&mut self,other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T: FloatNumber> Sub<Mat4x4<T>> for Mat4x4<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Mat4x4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: FloatNumber> SubAssign<Mat4x4<T>> for Mat4x4<T> {
    fn sub_assign(&mut self,other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T: FloatNumber> Mul<T> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn mul(self,other: T) -> Self::Output {
        Mat4x4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

macro_rules! scalar_mat4x4_mul {
    ($t:ty) => {
        impl Mul<Mat4x4<$t>> for $t {
            type Output = Mat4x4<$t>;
            fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                }
            }
        }
    }
}

scalar_mat4x4_mul!(f32);
scalar_mat4x4_mul!(f64);

impl<T: FloatNumber> Mul<Vec4<T>> for Mat4x4<T> {
    type Output = Vec4<T>;
    fn mul(self,other: Vec4<T>) -> Self::Output {
        Vec4 {
            x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z + self.w.x * other.w,
            y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z + self.w.y * other.w,
            z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z + self.w.z * other.w,
            w: self.x.w * other.x + self.y.w * other.y + self.z.w * other.z + self.w.w * other.w,
        }
    }
}

impl<T: FloatNumber> Mul<Mat4x4<T>> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn mul(self,other: Mat4x4<T>) -> Self::Output {
        Mat4x4 {
            x: vec4!(
                self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z + self.w.x * other.x.w,
                self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z + self.w.y * other.x.w,
                self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z + self.w.z * other.x.w,
                self.x.w * other.x.x + self.y.w * other.x.y + self.z.w * other.x.z + self.w.w * other.x.w
            ),
            y: vec4!(
                self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z + self.w.x * other.y.w,
                self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z + self.w.y * other.y.w,
                self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z + self.w.z * other.y.w,
                self.x.w * other.y.x + self.y.w * other.y.y + self.z.w * other.y.z + self.w.w * other.y.w
            ),
            z: vec4!(
                self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z + self.w.x * other.z.w,
                self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z + self.w.y * other.z.w,
                self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z + self.w.z * other.z.w,
                self.x.w * other.z.x + self.y.w * other.z.y + self.z.w * other.z.z + self.w.w * other.z.w
            ),
            w: vec4!(
                self.x.x * other.w.x + self.y.x * other.w.y + self.z.x * other.w.z + self.w.x * other.w.w,
                self.x.y * other.w.x + self.y.y * other.w.y + self.z.y * other.w.z + self.w.y * other.w.w,
                self.x.z * other.w.x + self.y.z * other.w.y + self.z.z * other.w.z + self.w.z * other.w.w,
                self.x.w * other.w.x + self.y.w * other.w.y + self.z.w * other.w.z + self.w.w * other.w.w
            ),
        }
    }
}

impl<T: FloatNumber> Div<T> for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn div(self,other: T) -> Self::Output {
        Mat4x4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: FloatNumber> MulAssign<T> for Mat4x4<T> {
    fn mul_assign(&mut self,other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl<T: FloatNumber> MulAssign<Mat4x4<T>> for Vec4<T> {
    fn mul_assign(&mut self,other: Mat4x4<T>) {
        let x = self.x * other.x.x + self.y * other.y.x + self.z * other.z.x + self.w * other.w.x;
        let y = self.x * other.x.y + self.y * other.y.y + self.z * other.z.y + self.w * other.w.y;
        let z = self.x * other.x.z + self.y * other.y.z + self.z * other.z.z + self.w * other.w.z;
        let w = self.x * other.x.w + self.y * other.y.w + self.z * other.z.w + self.w * other.w.w;
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}

impl<T: FloatNumber> MulAssign<Mat4x4<T>> for Mat4x4<T> {
    fn mul_assign(&mut self,other: Mat4x4<T>) {
        let x = vec4!(
            self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z + self.w.x * other.x.w,
            self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z + self.w.y * other.x.w,
            self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z + self.w.z * other.x.w,
            self.x.w * other.x.x + self.y.w * other.x.y + self.z.w * other.x.z + self.w.w * other.x.w
        );
        let y = vec4!(
            self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z + self.w.x * other.y.w,
            self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z + self.w.y * other.y.w,
            self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z + self.w.z * other.y.w,
            self.x.w * other.y.x + self.y.w * other.y.y + self.z.w * other.y.z + self.w.w * other.y.w
        );
        let z = vec4!(
            self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z + self.w.x * other.z.w,
            self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z + self.w.y * other.z.w,
            self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z + self.w.z * other.z.w,
            self.x.w * other.z.x + self.y.w * other.z.y + self.z.w * other.z.z + self.w.w * other.z.w
        );
        let w = vec4!(
            self.x.x * other.w.x + self.y.x * other.w.y + self.z.x * other.w.z + self.w.x * other.w.w,
            self.x.y * other.w.x + self.y.y * other.w.y + self.z.y * other.w.z + self.w.y * other.w.w,
            self.x.z * other.w.x + self.y.z * other.w.y + self.z.z * other.w.z + self.w.z * other.w.w,
            self.x.w * other.w.x + self.y.w * other.w.y + self.z.w * other.w.z + self.w.w * other.w.w
        );
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}

impl<T: FloatNumber> DivAssign<T> for Mat4x4<T> {
    fn div_assign(&mut self,other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl<T: FloatNumber> Neg for Mat4x4<T> {
    type Output = Mat4x4<T>;
    fn neg(self) -> Self::Output {
        Mat4x4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

macro_rules! mat4x4_rotations {
    ($t:ty) => {
        impl Mat4x4<$t> {
            /// Create pitch rotation matrix (rotation in YZ-plane).
            ///
            /// **Arguments**
            ///
            /// * `a` - Angle (in radians).
            ///
            /// **Returns**
            /// 
            /// The rotation matrix.
            pub fn pitch(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: vec4!(1.0,0.0,0.0,0.0),
                    y: vec4!(0.0,ca,sa,0.0),
                    z: vec4!(0.0,-sa,ca,0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
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
            pub fn yaw(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: vec4!(ca,0.0,-sa,0.0),
                    y: vec4!(0.0,1.0,0.0,0.0),
                    z: vec4!(sa,0.0,ca,0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
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
            pub fn roll(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: vec4!(ca,sa,0.0,0.0),
                    y: vec4!(-sa,ca,0.0,0.0),
                    z: vec4!(0.0,0.0,1.0,0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
                }
            }

            /// Create homogenous perspective projection matrix.
            ///
            /// **Arguments**
            /// * `fovy` - Vertical field-of-view angle (in radians).
            /// * `aspect` - Horizontal/vertical aspect ratio.
            /// * `n` - Near (minimum Z).
            /// * `f` - Far (maximum Z).
            ///
            /// **Returns**
            ///
            /// Homogenous perspective projection matrix.
            pub fn perspective(fovy: $t,aspect: $t,n: $t,f: $t) -> Mat4x4<$t> {
                let q = 1.0 / (fovy.to_radians() / 2.0).tan();
                Mat4x4 {
                    x: vec4!(q / aspect,0.0,0.0,0.0),
                    y: vec4!(0.0,q,0.0,0.0),
                    z: vec4!(0.0,0.0,(f + n) / (n - f),-1.0),
                    w: vec4!(0.0,0.0,2.0 * f * n / (n - f),0.0),
                }
            }
        }
    }
}

mat4x4_rotations!(f32);
mat4x4_rotations!(f64);
