// E - Matrix
// Desmond Germans, 2020

use crate::*;
use std::cmp::PartialEq;
use std::fmt::{Display,Formatter,Result,Debug};
use std::ops::{Add,Sub,Neg,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};

/// Elementary 2x2 matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat2x2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

macro_rules! impl_mat2x2 (
    ($t:ty) => (
        impl Mat2x2<$t> {
            /// Create 2x2-matrix.
            /// # Arguments
            /// * `x` - First column 2-vector.
            /// * `y` - Last column 2-vector.
            pub fn new(x: &Vec2<$t>,y: &Vec2<$t>) -> Mat2x2<$t> {
                Mat2x2 {
                    x: *x,
                    y: *y,
                }
            }
        }

        impl PartialEq for Mat2x2<$t> {
            fn eq(&self,other: &Mat2x2<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
            }
        }
        
        impl Display for Mat2x2<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({}; {};)",self.x,self.y)
            }
        }
        
        impl Zero for Mat2x2<$t> {
            /// Return empty 2x2-matrix.
            fn zero() -> Mat2x2<$t> {
                Mat2x2 {
                    x: Vec2::zero(),
                    y: Vec2::zero(),
                }
            }
        }

        impl One for Mat2x2<$t> {
            /// Return unit 2x2-matrix.
            fn one() -> Mat2x2<$t> {
                Mat2x2 {
                    x: vec2!(1.0,0.0),
                    y: vec2!(0.0,1.0),
                }
            }
        }

        impl Neg for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn neg(self) -> Self::Output {
                Mat2x2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl Add<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn add(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl Sub<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn sub(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl AddAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn add_assign(&mut self,other: Mat2x2<$t>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl SubAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn sub_assign(&mut self,other: Mat2x2<$t>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl Mul<$t> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat2x2 {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl Mul<Vec2<$t>> for Mat2x2<$t> {
            type Output = Vec2<$t>;
            fn mul(self,other: Vec2<$t>) -> Self::Output {
                Vec2 {
                    x: self.x.x * other.x + self.y.x * other.y,
                    y: self.x.y * other.x + self.y.y * other.y,
                }
            }
        }

        impl Mul<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn mul(self,other: Mat2x2<$t>) -> Self::Output {
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

        impl Div<$t> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat2x2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl MulAssign<$t> for Mat2x2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }

        impl MulAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn mul_assign(&mut self,other: Mat2x2<$t>) {
                let nx = vec2!(
                    self.x.x * other.x.x + self.y.x * other.x.y,
                    self.x.y * other.x.x + self.y.y * other.x.y
                );
                let ny = vec2!(
                    self.x.x * other.y.x + self.y.x * other.y.y,
                    self.x.y * other.y.x + self.y.y * other.y.y
                );
                self.x = nx;
                self.y = ny;
            }
        }

        impl DivAssign<$t> for Mat2x2<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                }
            }
        }
    );
);

impl_mat2x2!(f32);
impl_mat2x2!(f64);

/// Elementary 3x3 matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat3x3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

macro_rules! impl_mat3x3 (
    ($t:ty) => (
        impl Mat3x3<$t> {
            /// Create 3x3-matrix.
            /// # Arguments
            /// * `x` - First column 3-vector.
            /// * `y` - Middle column 3-vector.
            /// * `z` - Last column 3-vector.
            pub fn new(x: Vec3<$t>,y: Vec3<$t>,z: Vec3<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            /// Create scale matrix.
            /// # Arguments
            /// * `s` - Scale 3-vector.
            pub fn scale(s: Vec3<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: vec3!(s.x,0.0,0.0),
                    y: vec3!(0.0,s.y,0.0),
                    z: vec3!(0.0,0.0,s.z),
                }
            }

            /// Create rotation matrix from quaternion.
            /// # Arguments
            /// * `r` - Real component.
            /// * `i` - I-component.
            /// * `j` - J-component.
            /// * `k` - K-component.
            pub fn rotate(r: $t,i: $t,j: $t,k: $t) -> Mat3x3<$t> {
                let mut rr = r * r;
                let mut ii = i * i;
                let mut jj = j * j;
                let kk = k * k;
                let n = rr + ii + jj + kk;
                let s = if n != 0.0 {
                    2.0 / n
                }
                else {
                    0.0
                };
                let kr = s * k * r;
                rr *= s;
                ii *= s;
                let ki = s * k * i;
                let ri = s * r * i;
                let ij = s * i * j;
                let kj = s * k * j;
                let rj = s * r * j;
                jj *= s;
                Mat3x3 {
                    x: vec3!(1.0 - (ii + jj),ri - kj,rj + ki),
                    y: vec3!(ri + kj,1.0 - (rr + jj),ij - kr),
                    z: vec3!(rj - ki,ij + kr,1.0 - (rr + ii)),
                }
            }

            /// Create pitch rotation matrix (rotation in YZ-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
            pub fn pitch(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: vec3!(1.0,0.0,0.0),
                    y: vec3!(0.0,ca,sa),
                    z: vec3!(0.0,-sa,ca),
                }
            }

            /// Create yaw rotation matrix (rotation in XZ-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
            pub fn yaw(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: vec3!(ca,0.0,-sa),
                    y: vec3!(0.0,1.0,0.0),
                    z: vec3!(sa,0.0,ca),
                }
            }

            /// Create roll rotation matrix (rotation in XY-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
            pub fn roll(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: vec3!(ca,sa,0.0),
                    y: vec3!(-sa,ca,0.0),
                    z: vec3!(0.0,0.0,1.0),
                }
            }

            /// Create normal transformation matrix corresponding to
            /// homogenous transformation.
            pub fn normal_from(m: Mat4x4<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: vec3!(m.x.x,m.x.y,m.x.z),
                    y: vec3!(m.y.x,m.y.y,m.y.z),
                    z: vec3!(m.z.x,m.z.y,m.z.z),
                }.inverse().transpose()
            }

            /// Calculate determinant of 3x3-matrix.
            pub fn det(&self) -> $t {
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
            pub fn inverse(&self) -> Mat3x3<$t> {
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
                if nd != 0.0 {
                    let mb = c * h - b * i;
                    let mc = b * f - c * e;
                    let me = a * i - c * g;
                    let mf = c * d - a * f;
                    let mh = b * g - a * h;
                    let mi = a * e - b * d;
                    Mat3x3 {
                        x: vec3!(ma,md,mg),
                        y: vec3!(mb,me,mh),
                        z: vec3!(mc,mf,mi),
                    } / nd
                }
                else {
                    Mat3x3::one()
                }
            }

            /// Calculate transpose of 3x3-matrix.
            pub fn transpose(&self) -> Mat3x3<$t> {
                Mat3x3 {
                    x: vec3!(self.x.x,self.y.x,self.z.x),
                    y: vec3!(self.x.y,self.y.y,self.z.y),
                    z: vec3!(self.x.z,self.y.z,self.z.z),
                }
            }
        }

        impl PartialEq for Mat3x3<$t> {
            fn eq(&self,other: &Mat3x3<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
            }
        }
        
        impl Display for Mat3x3<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({}; {}; {};)",self.x,self.y,self.z)
            }
        }
        
        impl Zero for Mat3x3<$t> {
            /// Return empty 3x3-matrix.
            fn zero() -> Mat3x3<$t> {
                Mat3x3 {
                    x: Vec3::zero(),
                    y: Vec3::zero(),
                    z: Vec3::zero(),
                }
            }
        }

        impl One for Mat3x3<$t> {
            /// Return unit 3x3-matrix.
            fn one() -> Mat3x3<$t> {
                Mat3x3 {
                    x: vec3!(1.0,0.0,0.0),
                    y: vec3!(0.0,1.0,0.0),
                    z: vec3!(0.0,0.0,1.0),
                }
            }
        }

        impl Neg for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn neg(self) -> Self::Output {
                Mat3x3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl Add<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn add(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl Sub<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn sub(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl AddAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn add_assign(&mut self,other: Mat3x3<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl SubAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn sub_assign(&mut self,other: Mat3x3<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl Mul<$t> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat3x3 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }

        impl Mul<Vec3<$t>> for Mat3x3<$t> {
            type Output = Vec3<$t>;
            fn mul(self,other: Vec3<$t>) -> Self::Output {
                vec3!(
                    self.x.x * other.x + self.y.x * other.y + self.z.x * other.z,
                    self.x.y * other.x + self.y.y * other.y + self.z.y * other.z,
                    self.x.z * other.x + self.y.z * other.y + self.z.z * other.z
                )
            }
        }

        impl Mul<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn mul(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
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
                }
            }
        }

        impl Div<$t> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat3x3 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl MulAssign<$t> for Mat3x3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl MulAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn mul_assign(&mut self,other: Mat3x3<$t>) {
                let nx = vec3!(
                    self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z,
                    self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z,
                    self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z
                );
                let ny = vec3!(
                    self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z,
                    self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z,
                    self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z
                );
                let nz = vec3!(
                    self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z,
                    self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z,
                    self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z
                );
                self.x = nx;
                self.y = ny;
                self.z = nz;
            }
        }

        impl DivAssign<$t> for Mat3x3<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }
        }
    );
);

impl_mat3x3!(f32);
impl_mat3x3!(f64);

/// Elementary 4x4 matrix.
#[derive(Copy,Clone,Debug)]
pub struct Mat4x4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

macro_rules! impl_mat4x4 (
    ($t:ty) => (
        impl Mat4x4<$t> {
            /// Create 4x4-matrix.
            /// # Arguments
            /// * `x` - First column 4-vector.
            /// * `y` - Second column 4-vector.
            /// * `z` - Third column 4-vector.
            /// * `w` - Last column 4-vector.
            pub fn new(x: Vec4<$t>,y: Vec4<$t>,z: Vec4<$t>,w: Vec4<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }
 
            /// Calculate determinant of 4x4-matrix.
            pub fn det(&self) -> $t {
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
            pub fn inverse(&self) -> Mat4x4<$t> {
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
                if(nd != 0.0)
                {
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
                    Mat4x4::one()
                }
            }

            /// Calculate transpose of 4x4-matrix.
            pub fn transpose(&self) -> Mat4x4<$t> {
                Mat4x4 {
                    x: vec4!(self.x.x,self.y.x,self.z.x,self.w.x),
                    y: vec4!(self.x.y,self.y.y,self.z.y,self.w.y),
                    z: vec4!(self.x.z,self.y.z,self.z.z,self.w.z),
                    w: vec4!(self.x.w,self.y.w,self.z.w,self.w.w),
                }
            }

            /// Create homogenous orthogonal projection matrix.
            /// # Arguments
            /// * `l` - Left (minimum X).
            /// * `r` - Right (maximum X).
            /// * `b` - Bottom (minimum Y).
            /// * `t` - Top (maximum Y).
            /// * `n` - Near (minimum Z).
            /// * `f` - Far (maximum Z).
            pub fn ortho(l: $t,r: $t,b: $t,t: $t,n: $t,f: $t) -> Mat4x4<$t> {
                let dx = r - l;
                let dy = t - b;
                let dz = f - n;
                let rx = -(r + l) / dx;
                let ry = -(t + b) / dy;
                let rz = -(f + n) / dz;
                Mat4x4 {
                    x: vec4!(2.0 / dx,0.0,0.0,0.0),
                    y: vec4!(0.0,2.0 / dy,0.0,0.0),
                    z: vec4!(0.0,0.0,-2.0 / dz,0.0),
                    w: vec4!(rx,ry,rz,1.0),
                }
            }

            /// Create homogenous perspective projection matrix.
            /// # Arguments
            /// * `fovy` - Vertical field-of-view angle (in radians).
            /// * `aspect` - Horizontal/vertical aspect ratio.
            /// * `n` - Near (minimum Z).
            /// * `f` - Far (maximum Z).
            pub fn perspective(fovy: $t,aspect: $t,n: $t,f: $t) -> Mat4x4<$t> {
                let q = 1.0 / (fovy.to_radians() / 2.0).tan();
                Mat4x4 {
                    x: vec4!(q / aspect,0.0,0.0,0.0),
                    y: vec4!(0.0,q,0.0,0.0),
                    z: vec4!(0.0,0.0,(f + n) / (n - f),-1.0),
                    w: vec4!(0.0,0.0,2.0 * f * n / (n - f),0.0),
                }
            }

            /// Create homogenous translation matrix.
            /// # Arguments
            /// * `t` - Translation 3-vector.
            pub fn translate(t: Vec3<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: vec4!(1.0,0.0,0.0,0.0),
                    y: vec4!(0.0,1.0,0.0,0.0),
                    z: vec4!(0.0,0.0,1.0,0.0),
                    w: vec4!(t.x,t.y,t.z,1.0),
                }
            }

            /// Create homogenous scaling matrix.
            /// # Arguments
            /// * `s` - Scaling 3-vector.
            pub fn scale(s: Vec3<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: vec4!(s.x,0.0,0.0,0.0),
                    y: vec4!(0.0,s.y,0.0,0.0),
                    z: vec4!(0.0,0.0,s.z,0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
                }
            }

            /// Create homogenous rotation matrix from quaternion.
            /// # Arguments
            /// * `r` - Real component.
            /// * `i` - I-component.
            /// * `j` - J-component.
            /// * `k` - K-component.
            pub fn rotate(r: $t,i: $t,j: $t,k: $t) -> Mat4x4<$t> {
                let mut rr = r * r;
                let mut ii = i * i;
                let mut jj = j * j;
                let kk = k * k;
                let n = rr + ii + jj + kk;
                let s = if n != 0.0 {
                    2.0 / n
                }
                else {
                    0.0
                };
                let kr = s * k * r;
                rr *= s;
                ii *= s;
                let ki = s * k * i;
                let ri = s * r * i;
                let ij = s * i * j;
                let kj = s * k * j;
                let rj = s * r * j;
                jj *= s;
                Mat4x4 {
                    x: vec4!(1.0 - (ii + jj),ri - kj,rj + ki,0.0),
                    y: vec4!(ri + kj,1.0 - (rr + jj),ij - kr,0.0),
                    z: vec4!(rj - ki,ij + kr,1.0 - (rr + ii),0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
                }
            }

            /// Create homogenous pitch rotation matrix (rotation in YZ-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
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

            /// Create homogenous yaw rotation matrix (rotation in XZ-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
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

            /// Create homogenous roll rotation matrix (rotation in XY-plane).
            /// # Arguments
            /// * `a` - Angle (in radians).
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
        }

        impl PartialEq for Mat4x4<$t> {
            fn eq(&self,other: &Mat4x4<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
            }
        }
        
        impl Display for Mat4x4<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                write!(f,"({}; {}; {}; {};)",self.x,self.y,self.z,self.w)
            }
        }
        
        impl Zero for Mat4x4<$t> {
            /// Return empty 4x4-matrix.
            fn zero() -> Mat4x4<$t> {
                Mat4x4 {
                    x: Vec4::zero(),
                    y: Vec4::zero(),
                    z: Vec4::zero(),
                    w: Vec4::zero(),
                }
            }
        }

        impl One for Mat4x4<$t> {
            /// Return unit 4x4-matrix.
            fn one() -> Mat4x4<$t> {
                Mat4x4 {
                    x: vec4!(1.0,0.0,0.0,0.0),
                    y: vec4!(0.0,1.0,0.0,0.0),
                    z: vec4!(0.0,0.0,1.0,0.0),
                    w: vec4!(0.0,0.0,0.0,1.0),
                }
            }
        }

        impl Neg for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn neg(self) -> Self::Output {
                Mat4x4 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }

        impl Add<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn add(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }

        impl Sub<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn sub(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }

        impl AddAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn add_assign(&mut self,other: Mat4x4<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl SubAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn sub_assign(&mut self,other: Mat4x4<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }

        impl Mul<$t> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat4x4 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }

        impl Mul<Vec4<$t>> for Mat4x4<$t> {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Self::Output {
                Vec4 {
                    x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z + self.w.x * other.w,
                    y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z + self.w.y * other.w,
                    z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z + self.w.z * other.w,
                    w: self.x.w * other.x + self.y.w * other.y + self.z.w * other.z + self.w.w * other.w,
                }
            }
        }

        impl Mul<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn mul(self,other: Mat4x4<$t>) -> Self::Output {
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

        impl Div<$t> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat4x4 {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl MulAssign<$t> for Mat4x4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }

        impl MulAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn mul_assign(&mut self,other: Mat4x4<$t>) {
                let nx = vec4!(
                    self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z + self.w.x * other.x.w,
                    self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z + self.w.y * other.x.w,
                    self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z + self.w.z * other.x.w,
                    self.x.w * other.x.x + self.y.w * other.x.y + self.z.w * other.x.z + self.w.w * other.x.w
                );
                let ny = vec4!(
                    self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z + self.w.x * other.y.w,
                    self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z + self.w.y * other.y.w,
                    self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z + self.w.z * other.y.w,
                    self.x.w * other.y.x + self.y.w * other.y.y + self.z.w * other.y.z + self.w.w * other.y.w
                );
                let nz = vec4!(
                    self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z + self.w.x * other.z.w,
                    self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z + self.w.y * other.z.w,
                    self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z + self.w.z * other.z.w,
                    self.x.w * other.z.x + self.y.w * other.z.y + self.z.w * other.z.z + self.w.w * other.z.w
                );
                let nw = vec4!(
                    self.x.x * other.w.x + self.y.x * other.w.y + self.z.x * other.w.z + self.w.x * other.w.w,
                    self.x.y * other.w.x + self.y.y * other.w.y + self.z.y * other.w.z + self.w.y * other.w.w,
                    self.x.z * other.w.x + self.y.z * other.w.y + self.z.z * other.w.z + self.w.z * other.w.w,
                    self.x.w * other.w.x + self.y.w * other.w.y + self.z.w * other.w.z + self.w.w * other.w.w
                );
                self.x = nx;
                self.y = ny;
                self.z = nz;
                self.w = nw;
            }
        }

        impl DivAssign<$t> for Mat4x4<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }
        }
    );
);

impl_mat4x4!(f32);
impl_mat4x4!(f64);