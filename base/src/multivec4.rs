// E - Multivector
// Desmond Germans, 2020

/*
use crate::*;
use std::{
    cmp::PartialEq,
    fmt::{
        Display,
        Formatter,
        Debug,
        Result,
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
        Neg,
    },
};

/// 4D multivector.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec4<T: SimdableFloat>(Simd16<T>);

impl<T: SimdableFloat> MultiVec4<T> {
    /// Create new 4D multivector.
    ///
    /// **Arguments**
    ///
    /// * `r` - Scalar component.
    /// * `x` - X-vector component.
    /// * `y` - Y-vector component.
    /// * `z` - Z-vector component.
    /// * `w` - W-vector component.
    /// * `xy` - XY-bivector component.
    /// * `xz` - XY-bivector component.
    /// * `xw` - XY-bivector component.
    /// * `yz` - XY-bivector component.
    /// * `yw` - XY-bivector component.
    /// * `zw` - XY-bivector component.
    /// * `xyz` - XYZ-trivector component.
    /// * `xyw` - XYW-trivector component.
    /// * `xzw` - XZW-trivector component.
    /// * `yzw` - YZW-trivector component.
    /// * `xyzw` - pseudoscalar component.
    pub fn new(
        r: T,
        x: T,y: T,z: T,w: T,
        xy: T,xz: T,xw: T,yz: T,yw: T,zw: T,
        xyz: T,xzw: T,xyw: T,yzw: T,
        xyzw: T
    ) -> Self {
        MultiVec4(Simd16::new([
            r,
            x,y,z,w,
            xy,xz,xw,yz,yw,zw,
            xyz,xyw,xzw,yzw,
            xyzw
        ]))
    }

    /// Create new multivector containing a unit scalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_r() -> Self {
        MultiVec4(Simd16::new([T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit X-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_x() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit Y-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_y() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit Z-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_z() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit W-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_w() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XY-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xy() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XZ-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xz() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XW-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit YZ-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_yz() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit YW-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_yw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit ZW-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_zw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XYZ-trivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xyz() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XYW-trivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xyw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit XZW-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xzw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit YZW-bivector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_yzw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero()]))
    }

    /// Create new multivector containing a unit pseudoscalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xyzw() -> Self {
        MultiVec4(Simd16::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one()]))
    }

    /// Get scalar component.
    ///
    /// **Returns**
    ///
    /// The scalar component.
    pub fn r(&self) -> T {
        self.0.get(0)
    }

    /// Get X-vector component.
    ///
    /// **Returns**
    ///
    /// The X-vector component.
    pub fn x(&self) -> T {
        self.0.get(1)
    }

    /// Get Y-vector component.
    ///
    /// **Returns**
    ///
    /// The Y-vector component.
    pub fn y(&self) -> T {
        self.0.get(2)
    }

    /// Get Z-vector component.
    ///
    /// **Returns**
    ///
    /// The Z-vector component.
    pub fn z(&self) -> T {
        self.0.get(3)
    }

    /// Get W-vector component.
    ///
    /// **Returns**
    ///
    /// The W-vector component.
    pub fn w(&self) -> T {
        self.0.get(4)
    }

    /// Get XY-bivector component.
    ///
    /// **Returns**
    ///
    /// The XY-bivector component.
    pub fn xy(&self) -> T {
        self.0.get(5)
    }

    /// Get XZ-bivector component.
    ///
    /// **Returns**
    ///
    /// The XZ-bivector component.
    pub fn xz(&self) -> T {
        self.0.get(6)
    }

    /// Get XW-bivector component.
    ///
    /// **Returns**
    ///
    /// The XW-bivector component.
    pub fn xw(&self) -> T {
        self.0.get(7)
    }

    /// Get YZ-bivector component.
    ///
    /// **Returns**
    ///
    /// The YZ-bivector component.
    pub fn yz(&self) -> T {
        self.0.get(8)
    }

    /// Get YW-bivector component.
    ///
    /// **Returns**
    ///
    /// The YW-bivector component.
    pub fn yw(&self) -> T {
        self.0.get(9)
    }

    /// Get ZW-bivector component.
    ///
    /// **Returns**
    ///
    /// The ZW-bivector component.
    pub fn zw(&self) -> T {
        self.0.get(10)
    }

    /// Get XYZ-trivector component.
    ///
    /// **Returns**
    ///
    /// The XYZ-trivector component.
    pub fn xyz(&self) -> T {
        self.0.get(11)
    }

    /// Get XYW-trivector component.
    ///
    /// **Returns**
    ///
    /// The XYW-trivector component.
    pub fn xyw(&self) -> T {
        self.0.get(12)
    }

    /// Get XZW-trivector component.
    ///
    /// **Returns**
    ///
    /// The XZW-trivector component.
    pub fn xzw(&self) -> T {
        self.0.get(13)
    }

    /// Get YZW-trivector component.
    ///
    /// **Returns**
    ///
    /// The YZW-trivector component.
    pub fn yzw(&self) -> T {
        self.0.get(14)
    }

    /// Get pseudoscalar component.
    ///
    /// **Returns**
    ///
    /// The pseudoscalar component.
    pub fn xyzw(&self) -> T {
        self.0.get(15)
    }

    /// Set scalar component.
    ///
    /// **Arguments**
    ///
    /// `r` - New scalar component.
    pub fn set_r(&mut self,r: T) {
        self.0.set(0,r);
    }

    /// Set X-vector component.
    ///
    /// **Arguments**
    ///
    /// `x` - New X-vector component.
    pub fn set_x(&mut self,x: T) {
        self.0.set(1,x);
    }

    /// Set Y-vector component.
    ///
    /// **Arguments**
    ///
    /// `y` - New Y-vector component.
    pub fn set_y(&mut self,y: T) {
        self.0.set(2,y);
    }

    /// Set Z-vector component.
    ///
    /// **Arguments**
    ///
    /// `z` - New Z-vector component.
    pub fn set_z(&mut self,z: T) {
        self.0.set(3,z);
    }

    /// Set W-vector component.
    ///
    /// **Arguments**
    ///
    /// `w` - New W-vector component.
    pub fn set_w(&mut self,w: T) {
        self.0.set(4,w);
    }

    /// Set XY-bivector component.
    ///
    /// **Arguments**
    ///
    /// `xy` - New XY-bivector component.
    pub fn set_xy(&mut self,xy: T) {
        self.0.set(5,xy);
    }

    /// Set XZ-bivector component.
    ///
    /// **Arguments**
    ///
    /// `xz` - New XZ-bivector component.
    pub fn set_xz(&mut self,xz: T) {
        self.0.set(6,xz);
    }

    /// Set XW-bivector component.
    ///
    /// **Arguments**
    ///
    /// `xw` - New XW-bivector component.
    pub fn set_xw(&mut self,xw: T) {
        self.0.set(7,xw);
    }

    /// Set YZ-bivector component.
    ///
    /// **Arguments**
    ///
    /// `yz` - New YZ-bivector component.
    pub fn set_yz(&mut self,yz: T) {
        self.0.set(8,yz);
    }

    /// Set YW-bivector component.
    ///
    /// **Arguments**
    ///
    /// `yw` - New YW-bivector component.
    pub fn set_yw(&mut self,yw: T) {
        self.0.set(9,yw);
    }

    /// Set ZW-bivector component.
    ///
    /// **Arguments**
    ///
    /// `zw` - New ZW-bivector component.
    pub fn set_zw(&mut self,zw: T) {
        self.0.set(10,zw);
    }

    /// Set XYZ-trivector component.
    ///
    /// **Arguments**
    ///
    /// `xyz` - New XYZ-trivector component.
    pub fn set_xyz(&mut self,xyz: T) {
        self.0.set(11,xyz);
    }

    /// Set XYW-trivector component.
    ///
    /// **Arguments**
    ///
    /// `xyw` - New XYW-trivector component.
    pub fn set_xyw(&mut self,xyw: T) {
        self.0.set(12,xyw);
    }

    /// Set XZW-trivector component.
    ///
    /// **Arguments**
    ///
    /// `xzw` - New XZW-trivector component.
    pub fn set_xzw(&mut self,xzw: T) {
        self.0.set(13,xzw);
    }

    /// Set YZW-trivector component.
    ///
    /// **Arguments**
    ///
    /// `yzw` - New YZW-trivector component.
    pub fn set_yzw(&mut self,yzw: T) {
        self.0.set(14,yzw);
    }

    /// Set pseudoscalar component.
    ///
    /// **Arguments**
    ///
    /// `xyzw` - New pseudoscalar component.
    pub fn set_xyzw(&mut self,xyzw: T) {
        self.0.set(15,xyzw);
    }
}

impl<T: SimdableFloat> PartialEq for MultiVec4<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd16::eq(&self.0,&other.0,0xFFFFFFFF)
    }
}

impl<T: SimdableFloat> Zero for MultiVec4<T> {
    fn zero() -> Self {
        MultiVec4(Simd16::zero())
    }
}

impl<T: SimdableFloat> Display for MultiVec4<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let sx = if self.x < T::zero() {
            format!("{}x",self.x)
        } else {
            format!("+{}x",self.x)
        };
        let sy = if self.y < T::zero() {
            format!("{}y",self.y)
        } else {
            format!("+{}y",self.y)
        };
        let sz = if self.z < T::zero() {
            format!("{}z",self.z)
        } else {
            format!("+{}z",self.z)
        };
        let sw = if self.w < T::zero() {
            format!("{}w",self.w)
        } else {
            format!("+{}w",self.w)
        };
        let sxy = if self.xy < T::zero() {
            format!("{}xy",self.xy)
        } else {
            format!("+{}xy",self.xy)
        };
        let sxz = if self.xz < T::zero() {
            format!("{}xz",self.xz)
        } else {
            format!("+{}xz",self.xz)
        };
        let sxw = if self.xw < T::zero() {
            format!("{}xw",self.xw)
        } else {
            format!("+{}xw",self.xw)
        };
        let syz = if self.yz < T::zero() {
            format!("{}yz",self.yz)
        } else {
            format!("+{}yz",self.yz)
        };
        let syw = if self.yw < T::zero() {
            format!("{}yw",self.yw)
        } else {
            format!("+{}yw",self.yw)
        };
        let szw = if self.zw < T::zero() {
            format!("{}zw",self.zw)
        } else {
            format!("+{}zw",self.zw)
        };
        let sxyz = if self.xyz < T::zero() {
            format!("{}xyz",self.xyz)
        } else {
            format!("+{}xyz",self.xyz)
        };
        let sxyw = if self.xyw < T::zero() {
            format!("{}xyw",self.xyw)
        } else {
            format!("+{}xyw",self.xyw)
        };
        let sxzw = if self.xzw < T::zero() {
            format!("{}xzw",self.xzw)
        } else {
            format!("+{}xzw",self.xzw)
        };
        let syzw = if self.yzw < T::zero() {
            format!("{}yzw",self.yzw)
        } else {
            format!("+{}yzw",self.yzw)
        };
        let sxyzw = if self.xyzw < T::zero() {
            format!("{}xyzw",self.xyzw)
        } else {
            format!("+{}xyzw",self.xyzw)
        };
        write!(f,"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",self.r(),sx,sy,sz,sw,sxy,sxz,sxw,syz,syw,szw,sxyz,sxyw,sxzw,syzw,sxyzw)
    }
}

impl<T: SimdableFloat> Add<MultiVec4<T>> for MultiVec4<T> {
    type Output = Self;
    fn add(self,other: MultiVec4<T>) -> Self {
        MultiVec4(Simd16::add(self.0,other.0))
    }
}

impl<T: SimdableFloat> AddAssign<MultiVec4<T>> for MultiVec4<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd16::add(self.0,other.0);
    }
}

impl<T: SimdableFloat> Sub<MultiVec4<T>> for MultiVec4<T> {
    type Output = Self;
    fn sub(self,other: MultiVec4<T>) -> Self {
        MultiVec4(Simd16::sub(self.0,other.0))
    }
}

impl<T: SimdableFloat> SubAssign<MultiVec4<T>> for MultiVec4<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd16::sub(self.0,other.0);
    }
}

macro_rules! scalar_multivec4_mul {
    ($t:ty) => {
        impl Mul<MultiVec4<$t>> for $t {
            type Output = MultiVec4<$t>;
            fn mul(self,other: MultiVec4<$t>) -> MultiVec4<$t> {
                MultiVec4(Simd16::mul(Simd16::splat(self),other.0))
            }
        }
    }
}

scalar_multivec4_mul!(f32);
scalar_multivec4_mul!(f64);

impl<T: SimdableFloat> Mul<T> for MultiVec4<T> {
    type Output = MultiVec4<T>;
    fn mul(self,other: T) -> Self {
        MultiVec4(Simd16::mul(self.0,Simd16::splat(other)))
    }
}

impl<T: SimdableFloat> MulAssign<T> for MultiVec4<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd16::mul(self.0,Simd16::splat(other));
    }
}

impl<T: SimdableFloat> Mul<MultiVec4<T>> for MultiVec4<T> {
    type Output = MultiVec4<T>;
    fn mul(self,other: MultiVec4<T>) -> Self {
        MultiVec4::new(
            self.r() * other.r() + self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w - self.xy * other.xy - self.xz * other.xz - self.xw * other.xw - self.yz * other.yz - self.yw * other.yw - self.zw * other.zw - self.xyz * other.xyz - self.xyw * other.xyw - self.xzw * other.xzw - self.yzw * other.yzw + self.xyzw * other.xyzw,
            self.r() * other.x + self.x * other.r() - self.y * other.xy - self.z * other.xz - self.w * other.xw + self.xy * other.y + self.xz * other.z + self.xw * other.w - self.yz * other.xyz - self.yw * other.xyw - self.zw * other.xzw - self.xyz * other.yz - self.xyw * other.yw - self.xzw * other.zw + self.yzw * other.xyzw - self.xyzw * other.yzw,
            self.r() * other.y + self.x * other.xy + self.y * other.r() - self.z * other.yz - self.w * other.yw - self.xy * other.x + self.xz * other.xyz + self.xw * other.xyw + self.yz * other.z + self.yw * other.w - self.zw * other.yzw + self.xyz * other.xz + self.xyw * other.xw - self.xzw * other.xyzw - self.yzw * other.zw + self.xyzw * other.xzw,
            self.r() * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r() - self.w * other.zw - self.xy * other.xyz - self.xz * other.x + self.xw * other.xzw - self.yz * other.y + self.yw * other.yzw + self.zw * other.w - self.xyz * other.xy + self.xyw * other.xyzw + self.xzw * other.xw + self.yzw * other.yw - self.xyzw * other.xyw,
            self.r() * other.w + self.x * other.xw + self.y * other.yw + self.z * other.zw + self.w * other.r() - self.xy * other.xyw - self.xz * other.xzw - self.xw * other.x - self.yz * other.yzw - self.yw * other.y - self.zw * other.z - self.xyz * other.xyzw - self.xyw * other.xy - self.xzw * other.xz - self.yzw * other.yz + self.xyzw * other.xyz,
            self.r() * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.w * other.xyw + self.xy * other.r() - self.xz * other.yz - self.xw * other.yw + self.yz * other.xz + self.yw * other.xw - self.zw * other.xyzw + self.xyz * other.z + self.xyw * other.w - self.xzw * other.yzw + self.yzw * other.xzw - self.xyzw * other.zw,
            self.r() * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.w * other.xzw + self.xy * other.yz + self.xz * other.r() - self.xw * other.zw - self.yz * other.xy + self.yw * other.xyzw + self.zw * other.xw - self.xyz * other.y + self.xyw * other.yzw + self.xzw * other.w - self.yzw * other.xyw + self.xyzw * other.yw,
            self.r() * other.xw + self.x * other.w - self.y * other.xyw - self.z * other.xzw - self.w * other.x + self.xy * other.yw + self.xz * other.zw + self.xw * other.r() - self.yz * other.xyzw - self.yw * other.xy - self.zw * other.xz - self.xyz * other.yzw - self.xyw * other.y - self.xzw * other.z + self.yzw * other.xyz - self.xyzw * other.yz,
            self.r() * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y + self.w * other.yzw - self.xy * other.xz + self.xz * other.xy - self.xw * other.xyzw + self.yz * other.r() - self.yw * other.zw + self.zw * other.yw + self.xyz * other.x - self.xyw * other.xzw + self.xzw * other.xyw + self.yzw * other.w - self.xyzw * other.xw,
            self.r() * other.yw + self.x * other.xyw + self.y * other.w - self.z * other.yzw - self.w * other.y - self.xy * other.xw + self.xz * other.xyzw + self.xw * other.xy + self.yz * other.zw + self.yw * other.r() - self.zw * other.yz + self.xyz * other.xzw + self.xyw * other.x - self.xzw * other.xyz - self.yzw * other.z + self.xyzw * other.xz,
            self.r() * other.zw + self.x * other.xzw + self.y * other.yzw + self.z * other.w - self.w * other.z - self.xy * other.xyzw - self.xz * other.xw + self.xw * other.xz - self.yz * other.yw + self.yw * other.yz + self.zw * other.r() - self.xyz * other.xyw + self.xyw * other.xyz + self.xzw * other.x + self.yzw * other.y - self.xyzw * other.xy,
            self.r() * other.xyz + self.x * other.yz - self.y * other.xz + self.z * other.xy - self.w * other.xyzw + self.xy * other.z - self.xz * other.y + self.xw * other.yzw + self.yz * other.x - self.yw * other.xzw + self.zw * other.xyw + self.xyz * other.r() - self.xyw * other.zw + self.xzw * other.yw - self.yzw * other.xw + self.xyzw * other.w,
            self.r() * other.xyw + self.x * other.yw - self.y * other.xw + self.z * other.xyzw + self.w * other.xy + self.xy * other.w - self.xz * other.yzw - self.xw * other.y + self.yz * other.xzw + self.yw * other.x - self.zw * other.xyz + self.xyz * other.zw + self.xyw * other.r() - self.xzw * other.yz + self.yzw * other.xz - self.xyzw * other.z,
            self.r() * other.xzw + self.x * other.zw - self.y * other.xyzw - self.z * other.xw + self.w * other.xz + self.xy * other.yzw + self.xz * other.w - self.xw * other.z - self.yz * other.xyw + self.yw * other.xyz + self.zw * other.x - self.xyz * other.yw + self.xyw * other.yz + self.xzw * other.r() - self.yzw * other.xy + self.xyzw * other.y,
            self.r() * other.yzw + self.x * other.xyzw + self.y * other.zw - self.z * other.yw + self.w * other.yz - self.xy * other.xzw + self.xz * other.xyw - self.xw * other.xyz + self.yz * other.w - self.yw * other.z + self.zw * other.y + self.xyz * other.xw - self.xyw * other.xz + self.xzw * other.xy + self.yzw * other.r() - self.xyzw * other.x,
            self.r() * other.xyzw + self.x * other.yzw - self.y * other.xzw + self.z * other.xyw - self.w * other.xyz + self.xy * other.zw - self.xz * other.yw + self.xw * other.yz + self.yz * other.xw - self.yw * other.xz + self.zw * other.xy + self.xyz * other.w - self.xyw * other.z + self.xzw * other.y - self.yzw * other.x + self.xyzw * other.r()
        )
    }
}

impl<T: SimdableFloat> MulAssign<MultiVec4<T>> for MultiVec4<T> {
    fn mul_assign(&mut self,other: MultiVec4<T>) {
        let nr = self.r() * other.r() + self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w - self.xy * other.xy - self.xz * other.xz - self.xw * other.xw - self.yz * other.yz - self.yw * other.yw - self.zw * other.zw - self.xyz * other.xyz - self.xyw * other.xyw - self.xzw * other.xzw - self.yzw * other.yzw + self.xyzw * other.xyzw;
        let nx = self.r() * other.x + self.x * other.r() - self.y * other.xy - self.z * other.xz - self.w * other.xw + self.xy * other.y + self.xz * other.z + self.xw * other.w - self.yz * other.xyz - self.yw * other.xyw - self.zw * other.xzw - self.xyz * other.yz - self.xyw * other.yw - self.xzw * other.zw + self.yzw * other.xyzw - self.xyzw * other.yzw;
        let ny = self.r() * other.y + self.x * other.xy + self.y * other.r() - self.z * other.yz - self.w * other.yw - self.xy * other.x + self.xz * other.xyz + self.xw * other.xyw + self.yz * other.z + self.yw * other.w - self.zw * other.yzw + self.xyz * other.xz + self.xyw * other.xw - self.xzw * other.xyzw - self.yzw * other.zw + self.xyzw * other.xzw;
        let nz = self.r() * other.z + self.x * other.xz + self.y * other.yz + self.z * other.r() - self.w * other.zw - self.xy * other.xyz - self.xz * other.x + self.xw * other.xzw - self.yz * other.y + self.yw * other.yzw + self.zw * other.w - self.xyz * other.xy + self.xyw * other.xyzw + self.xzw * other.xw + self.yzw * other.yw - self.xyzw * other.xyw;
        let nw = self.r() * other.w + self.x * other.xw + self.y * other.yw + self.z * other.zw + self.w * other.r() - self.xy * other.xyw - self.xz * other.xzw - self.xw * other.x - self.yz * other.yzw - self.yw * other.y - self.zw * other.z - self.xyz * other.xyzw - self.xyw * other.xy - self.xzw * other.xz - self.yzw * other.yz + self.xyzw * other.xyz;
        let nxy = self.r() * other.xy + self.x * other.y - self.y * other.x + self.z * other.xyz + self.w * other.xyw + self.xy * other.r() - self.xz * other.yz - self.xw * other.yw + self.yz * other.xz + self.yw * other.xw - self.zw * other.xyzw + self.xyz * other.z + self.xyw * other.w - self.xzw * other.yzw + self.yzw * other.xzw - self.xyzw * other.zw;
        let nxz = self.r() * other.xz + self.x * other.z - self.y * other.xyz - self.z * other.x + self.w * other.xzw + self.xy * other.yz + self.xz * other.r() - self.xw * other.zw - self.yz * other.xy + self.yw * other.xyzw + self.zw * other.xw - self.xyz * other.y + self.xyw * other.yzw + self.xzw * other.w - self.yzw * other.xyw + self.xyzw * other.yw;
        let nxw = self.r() * other.xw + self.x * other.w - self.y * other.xyw - self.z * other.xzw - self.w * other.x + self.xy * other.yw + self.xz * other.zw + self.xw * other.r() - self.yz * other.xyzw - self.yw * other.xy - self.zw * other.xz - self.xyz * other.yzw - self.xyw * other.y - self.xzw * other.z + self.yzw * other.xyz - self.xyzw * other.yz;
        let nyz = self.r() * other.yz + self.x * other.xyz + self.y * other.z - self.z * other.y + self.w * other.yzw - self.xy * other.xz + self.xz * other.xy - self.xw * other.xyzw + self.yz * other.r() - self.yw * other.zw + self.zw * other.yw + self.xyz * other.x - self.xyw * other.xzw + self.xzw * other.xyw + self.yzw * other.w - self.xyzw * other.xw;
        let nyw = self.r() * other.yw + self.x * other.xyw + self.y * other.w - self.z * other.yzw - self.w * other.y - self.xy * other.xw + self.xz * other.xyzw + self.xw * other.xy + self.yz * other.zw + self.yw * other.r() - self.zw * other.yz + self.xyz * other.xzw + self.xyw * other.x - self.xzw * other.xyz - self.yzw * other.z + self.xyzw * other.xz;
        let nzw = self.r() * other.zw + self.x * other.xzw + self.y * other.yzw + self.z * other.w - self.w * other.z - self.xy * other.xyzw - self.xz * other.xw + self.xw * other.xz - self.yz * other.yw + self.yw * other.yz + self.zw * other.r() - self.xyz * other.xyw + self.xyw * other.xyz + self.xzw * other.x + self.yzw * other.y - self.xyzw * other.xy;
        let nxyz = self.r() * other.xyz + self.x * other.yz - self.y * other.xz + self.z * other.xy - self.w * other.xyzw + self.xy * other.z - self.xz * other.y + self.xw * other.yzw + self.yz * other.x - self.yw * other.xzw + self.zw * other.xyw + self.xyz * other.r() - self.xyw * other.zw + self.xzw * other.yw - self.yzw * other.xw + self.xyzw * other.w;
        let nxyw = self.r() * other.xyw + self.x * other.yw - self.y * other.xw + self.z * other.xyzw + self.w * other.xy + self.xy * other.w - self.xz * other.yzw - self.xw * other.y + self.yz * other.xzw + self.yw * other.x - self.zw * other.xyz + self.xyz * other.zw + self.xyw * other.r() - self.xzw * other.yz + self.yzw * other.xz - self.xyzw * other.z;
        let nxzw = self.r() * other.xzw + self.x * other.zw - self.y * other.xyzw - self.z * other.xw + self.w * other.xz + self.xy * other.yzw + self.xz * other.w - self.xw * other.z - self.yz * other.xyw + self.yw * other.xyz + self.zw * other.x - self.xyz * other.yw + self.xyw * other.yz + self.xzw * other.r() - self.yzw * other.xy + self.xyzw * other.y;
        let nyzw = self.r() * other.yzw + self.x * other.xyzw + self.y * other.zw - self.z * other.yw + self.w * other.yz - self.xy * other.xzw + self.xz * other.xyw - self.xw * other.xyz + self.yz * other.w - self.yw * other.z + self.zw * other.y + self.xyz * other.xw - self.xyw * other.xz + self.xzw * other.xy + self.yzw * other.r() - self.xyzw * other.x;
        let nxyzw = self.r() * other.xyzw + self.x * other.yzw - self.y * other.xzw + self.z * other.xyw - self.w * other.xyz + self.xy * other.zw - self.xz * other.yw + self.xw * other.yz + self.yz * other.xw - self.yw * other.xz + self.zw * other.xy + self.xyz * other.w - self.xyw * other.z + self.xzw * other.y - self.yzw * other.x + self.xyzw * other.r();
        self.0 = Simd16::new([
            nr,
            nx,ny,nz,nw,
            nxy,nxz,nxw,nyz,nyw,nzw,
            nxyz,nxyw,nxzw,nyzw,
            nxyzw,
        ]);
    }
}

impl<T: SimdableFloat> Div<T> for MultiVec4<T> {
    type Output = MultiVec4<T>;
    fn div(self,other: T) -> Self {
        MultiVec4(Simd16::div(self.0,Simd16::splat(other)))
    }
}

impl<T: SimdableFloat> DivAssign<T> for MultiVec4<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd16::div(self.0,Simd16::splat(other));
    }
}

impl<T: SimdableFloat> Neg for MultiVec4<T> {
    type Output = MultiVec4<T>;
    fn neg(self) -> MultiVec4<T> {
        MultiVec4(Simd16::sub(Simd16::zero(),self.0))
    }
}

impl<T: SimdableFloat> From<T> for MultiVec4<T> {
    fn from(v: T) -> MultiVec4<T> {
        MultiVec4::new(v,T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero())
    }
}

impl<T: SimdableFloat> From<Vec4<T>> for MultiVec4<T> {
    fn from(v: Vec4<T>) -> MultiVec4<T> {
        MultiVec4::new(T::zero(),v.x,v.y,v.z,v.w,T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero())
    }
}
*/