// E - Multivector
// Desmond Germans, 2020

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

/// Elementary 3-multivector.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec4<T: Simd16>(<T as Simd16>::Type);

macro_rules! impl_multivec4 (
    ($t:ty; $o:expr; $z:expr) => (
        impl MultiVec4<$t> {
            pub fn new(
                r: $t,
                x: $t,y: $t,z: $t,w: $t,
                xy: $t,xz: $t,xw: $t,yz: $t,yw: $t,zw: $t,
                xyz: $t,xzw: $t,xyw: $t,yzw: $t,
                xyzw: $t
            ) -> Self {
                MultiVec4(<$t as Simd16>::Type::new(
                    r,
                    x,y,z,w,
                    xy,xz,xw,yz,yw,zw,
                    xyz,xyw,xzw,yzw,
                    xyzw
                ))
            }

            pub fn unit_r() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_x() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_y() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_z() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_w() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_xy() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_xz() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_xw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_yz() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_yw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_zw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z,$z))
            }

            pub fn unit_xyz() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z,$z))
            }

            pub fn unit_xyw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z,$z))
            }

            pub fn unit_xzw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z,$z))
            }

            pub fn unit_yzw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o,$z))
            }

            pub fn unit_xyzw() -> Self {
                MultiVec4(<$t as Simd16>::Type::new($z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$z,$o))
            }

            pub fn r(&self) -> $t {
                self.0.get(0)
            }

            pub fn x(&self) -> $t {
                self.0.get(1)
            }

            pub fn y(&self) -> $t {
                self.0.get(2)
            }

            pub fn z(&self) -> $t {
                self.0.get(3)
            }

            pub fn w(&self) -> $t {
                self.0.get(4)
            }

            pub fn xy(&self) -> $t {
                self.0.get(5)
            }

            pub fn xz(&self) -> $t {
                self.0.get(6)
            }

            pub fn xw(&self) -> $t {
                self.0.get(7)
            }

            pub fn yz(&self) -> $t {
                self.0.get(8)
            }

            pub fn yw(&self) -> $t {
                self.0.get(9)
            }

            pub fn zw(&self) -> $t {
                self.0.get(10)
            }

            pub fn xyz(&self) -> $t {
                self.0.get(11)
            }

            pub fn xyw(&self) -> $t {
                self.0.get(12)
            }

            pub fn xzw(&self) -> $t {
                self.0.get(13)
            }

            pub fn yzw(&self) -> $t {
                self.0.get(14)
            }

            pub fn xyzw(&self) -> $t {
                self.0.get(15)
            }

            pub fn set_r(&mut self,r: $t) {
                self.0.set(0,r);
            }

            pub fn set_x(&mut self,x: $t) {
                self.0.set(1,x);
            }

            pub fn set_y(&mut self,y: $t) {
                self.0.set(2,y);
            }

            pub fn set_z(&mut self,z: $t) {
                self.0.set(3,z);
            }

            pub fn set_w(&mut self,w: $t) {
                self.0.set(4,w);
            }

            pub fn set_xy(&mut self,xy: $t) {
                self.0.set(5,xy);
            }

            pub fn set_xz(&mut self,xz: $t) {
                self.0.set(6,xz);
            }

            pub fn set_xw(&mut self,xw: $t) {
                self.0.set(7,xw);
            }

            pub fn set_yz(&mut self,yz: $t) {
                self.0.set(8,yz);
            }

            pub fn set_yw(&mut self,yw: $t) {
                self.0.set(9,yw);
            }

            pub fn set_zw(&mut self,zw: $t) {
                self.0.set(10,zw);
            }

            pub fn set_xyz(&mut self,xyz: $t) {
                self.0.set(11,xyz);
            }

            pub fn set_xyw(&mut self,xyw: $t) {
                self.0.set(12,xyw);
            }

            pub fn set_xzw(&mut self,xzw: $t) {
                self.0.set(13,xzw);
            }

            pub fn set_yzw(&mut self,yzw: $t) {
                self.0.set(14,yzw);
            }

            pub fn set_xyzw(&mut self,xyzw: $t) {
                self.0.set(15,xyzw);
            }
        }

        impl PartialEq for MultiVec4<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd16>::Type::eq(&self.0,&other.0,0xFFFFFFFF)
            }
        }

        impl Zero for MultiVec4<$t> {
            fn zero() -> Self {
                MultiVec4(<$t as Simd16>::Type::zero())
            }
        }

        impl Display for MultiVec4<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                let sx = if self.x() < $z {
                    format!("{}x",self.x())
                } else {
                    format!("+{}x",self.x())
                };
                let sy = if self.y() < $z {
                    format!("{}y",self.y())
                } else {
                    format!("+{}y",self.y())
                };
                let sz = if self.z() < $z {
                    format!("{}z",self.z())
                } else {
                    format!("+{}z",self.z())
                };
                let sw = if self.w() < $z {
                    format!("{}w",self.w())
                } else {
                    format!("+{}w",self.w())
                };
                let sxy = if self.xy() < $z {
                    format!("{}xy",self.xy())
                } else {
                    format!("+{}xy",self.xy())
                };
                let sxz = if self.xz() < $z {
                    format!("{}xz",self.xz())
                } else {
                    format!("+{}xz",self.xz())
                };
                let sxw = if self.xw() < $z {
                    format!("{}xw",self.xw())
                } else {
                    format!("+{}xw",self.xw())
                };
                let syz = if self.yz() < $z {
                    format!("{}yz",self.yz())
                } else {
                    format!("+{}yz",self.yz())
                };
                let syw = if self.yw() < $z {
                    format!("{}yw",self.yw())
                } else {
                    format!("+{}yw",self.yw())
                };
                let szw = if self.zw() < $z {
                    format!("{}zw",self.zw())
                } else {
                    format!("+{}zw",self.zw())
                };
                let sxyz = if self.xyz() < $z {
                    format!("{}xyz",self.xyz())
                } else {
                    format!("+{}xyz",self.xyz())
                };
                let sxyw = if self.xyw() < $z {
                    format!("{}xyw",self.xyw())
                } else {
                    format!("+{}xyw",self.xyw())
                };
                let sxzw = if self.xzw() < $z {
                    format!("{}xzw",self.xzw())
                } else {
                    format!("+{}xzw",self.xzw())
                };
                let syzw = if self.yzw() < $z {
                    format!("{}yzw",self.yzw())
                } else {
                    format!("+{}yzw",self.yzw())
                };
                let sxyzw = if self.xyzw() < $z {
                    format!("{}xyzw",self.xyzw())
                } else {
                    format!("+{}xyzw",self.xyzw())
                };
                write!(f,"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",self.r(),sx,sy,sz,sw,sxy,sxz,sxw,syz,syw,szw,sxyz,sxyw,sxzw,syzw,sxyzw)
            }
        }

        impl Add<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = Self;
            fn add(self,other: MultiVec4<$t>) -> Self {
                MultiVec4(<$t as Simd16>::Type::add(&self.0,&other.0))
            }
        }

        impl AddAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd16>::Type::add(&self.0,&other.0);
            }
        }

        impl Sub<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = Self;
            fn sub(self,other: MultiVec4<$t>) -> Self {
                MultiVec4(<$t as Simd16>::Type::sub(&self.0,&other.0))
            }
        }

        impl SubAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd16>::Type::sub(&self.0,&other.0);
            }
        }

        impl Mul<MultiVec4<$t>> for $t {
            type Output = MultiVec4<$t>;
            fn mul(self,other: MultiVec4<$t>) -> MultiVec4<$t> {
                MultiVec4(<$t as Simd16>::Type::mul(&<$t as Simd16>::Type::splat(self),&other.0))
            }
        }

        impl Mul<$t> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn mul(self,other: $t) -> Self {
                MultiVec4(<$t as Simd16>::Type::mul(&self.0,&<$t as Simd16>::Type::splat(other)))
            }
        }

        impl MulAssign<$t> for MultiVec4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd16>::Type::mul(&self.0,&<$t as Simd16>::Type::splat(other));
            }
        }

        impl Mul<MultiVec4<$t>> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn mul(self,other: MultiVec4<$t>) -> Self {
                MultiVec4(<$t as Simd16>::Type::new(
                    self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w() - self.xy() * other.xy() - self.xz() * other.xz() - self.xw() * other.xw() - self.yz() * other.yz() - self.yw() * other.yw() - self.zw() * other.zw() - self.xyz() * other.xyz() - self.xyw() * other.xyw() - self.xzw() * other.xzw() - self.yzw() * other.yzw() + self.xyzw() * other.xyzw(),
                    self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() - self.w() * other.xw() + self.xy() * other.y() + self.xz() * other.z() + self.xw() * other.w() - self.yz() * other.xyz() - self.yw() * other.xyw() - self.zw() * other.xzw() - self.xyz() * other.yz() - self.xyw() * other.yw() - self.xzw() * other.zw() + self.yzw() * other.xyzw() - self.xyzw() * other.yzw(),
                    self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.w() * other.yw() - self.xy() * other.x() + self.xz() * other.xyz() + self.xw() * other.xyw() + self.yz() * other.z() + self.yw() * other.w() - self.zw() * other.yzw() + self.xyz() * other.xz() + self.xyw() * other.xw() - self.xzw() * other.xyzw() - self.yzw() * other.zw() + self.xyzw() * other.xzw(),
                    self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.w() * other.zw() - self.xy() * other.xyz() - self.xz() * other.x() + self.xw() * other.xzw() - self.yz() * other.y() + self.yw() * other.yzw() + self.zw() * other.w() - self.xyz() * other.xy() + self.xyw() * other.xyzw() + self.xzw() * other.xw() + self.yzw() * other.yw() - self.xyzw() * other.xyw(),
                    self.r() * other.w() + self.x() * other.xw() + self.y() * other.yw() + self.z() * other.zw() + self.w() * other.r() - self.xy() * other.xyw() - self.xz() * other.xzw() - self.xw() * other.x() - self.yz() * other.yzw() - self.yw() * other.y() - self.zw() * other.z() - self.xyz() * other.xyzw() - self.xyw() * other.xy() - self.xzw() * other.xz() - self.yzw() * other.yz() + self.xyzw() * other.xyz(),
                    self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.w() * other.xyw() + self.xy() * other.r() - self.xz() * other.yz() - self.xw() * other.yw() + self.yz() * other.xz() + self.yw() * other.xw() - self.zw() * other.xyzw() + self.xyz() * other.z() + self.xyw() * other.w() - self.xzw() * other.yzw() + self.yzw() * other.xzw() - self.xyzw() * other.zw(),
                    self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.w() * other.xzw() + self.xy() * other.yz() + self.xz() * other.r() - self.xw() * other.zw() - self.yz() * other.xy() + self.yw() * other.xyzw() + self.zw() * other.xw() - self.xyz() * other.y() + self.xyw() * other.yzw() + self.xzw() * other.w() - self.yzw() * other.xyw() + self.xyzw() * other.yw(),
                    self.r() * other.xw() + self.x() * other.w() - self.y() * other.xyw() - self.z() * other.xzw() - self.w() * other.x() + self.xy() * other.yw() + self.xz() * other.zw() + self.xw() * other.r() - self.yz() * other.xyzw() - self.yw() * other.xy() - self.zw() * other.xz() - self.xyz() * other.yzw() - self.xyw() * other.y() - self.xzw() * other.z() + self.yzw() * other.xyz() - self.xyzw() * other.yz(),
                    self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() + self.w() * other.yzw() - self.xy() * other.xz() + self.xz() * other.xy() - self.xw() * other.xyzw() + self.yz() * other.r() - self.yw() * other.zw() + self.zw() * other.yw() + self.xyz() * other.x() - self.xyw() * other.xzw() + self.xzw() * other.xyw() + self.yzw() * other.w() - self.xyzw() * other.xw(),
                    self.r() * other.yw() + self.x() * other.xyw() + self.y() * other.w() - self.z() * other.yzw() - self.w() * other.y() - self.xy() * other.xw() + self.xz() * other.xyzw() + self.xw() * other.xy() + self.yz() * other.zw() + self.yw() * other.r() - self.zw() * other.yz() + self.xyz() * other.xzw() + self.xyw() * other.x() - self.xzw() * other.xyz() - self.yzw() * other.z() + self.xyzw() * other.xz(),
                    self.r() * other.zw() + self.x() * other.xzw() + self.y() * other.yzw() + self.z() * other.w() - self.w() * other.z() - self.xy() * other.xyzw() - self.xz() * other.xw() + self.xw() * other.xz() - self.yz() * other.yw() + self.yw() * other.yz() + self.zw() * other.r() - self.xyz() * other.xyw() + self.xyw() * other.xyz() + self.xzw() * other.x() + self.yzw() * other.y() - self.xyzw() * other.xy(),
                    self.r() * other.xyz() + self.x() * other.yz() - self.y() * other.xz() + self.z() * other.xy() - self.w() * other.xyzw() + self.xy() * other.z() - self.xz() * other.y() + self.xw() * other.yzw() + self.yz() * other.x() - self.yw() * other.xzw() + self.zw() * other.xyw() + self.xyz() * other.r() - self.xyw() * other.zw() + self.xzw() * other.yw() - self.yzw() * other.xw() + self.xyzw() * other.w(),
                    self.r() * other.xyw() + self.x() * other.yw() - self.y() * other.xw() + self.z() * other.xyzw() + self.w() * other.xy() + self.xy() * other.w() - self.xz() * other.yzw() - self.xw() * other.y() + self.yz() * other.xzw() + self.yw() * other.x() - self.zw() * other.xyz() + self.xyz() * other.zw() + self.xyw() * other.r() - self.xzw() * other.yz() + self.yzw() * other.xz() - self.xyzw() * other.z(),
                    self.r() * other.xzw() + self.x() * other.zw() - self.y() * other.xyzw() - self.z() * other.xw() + self.w() * other.xz() + self.xy() * other.yzw() + self.xz() * other.w() - self.xw() * other.z() - self.yz() * other.xyw() + self.yw() * other.xyz() + self.zw() * other.x() - self.xyz() * other.yw() + self.xyw() * other.yz() + self.xzw() * other.r() - self.yzw() * other.xy() + self.xyzw() * other.y(),
                    self.r() * other.yzw() + self.x() * other.xyzw() + self.y() * other.zw() - self.z() * other.yw() + self.w() * other.yz() - self.xy() * other.xzw() + self.xz() * other.xyw() - self.xw() * other.xyz() + self.yz() * other.w() - self.yw() * other.z() + self.zw() * other.y() + self.xyz() * other.xw() - self.xyw() * other.xz() + self.xzw() * other.xy() + self.yzw() * other.r() - self.xyzw() * other.x(),
                    self.r() * other.xyzw() + self.x() * other.yzw() - self.y() * other.xzw() + self.z() * other.xyw() - self.w() * other.xyz() + self.xy() * other.zw() - self.xz() * other.yw() + self.xw() * other.yz() + self.yz() * other.xw() - self.yw() * other.xz() + self.zw() * other.xy() + self.xyz() * other.w() - self.xyw() * other.z() + self.xzw() * other.y() - self.yzw() * other.x() + self.xyzw() * other.r()
                ))
            }
        }

        impl MulAssign<MultiVec4<$t>> for MultiVec4<$t> {
            fn mul_assign(&mut self,other: MultiVec4<$t>) {
                let nr = self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w() - self.xy() * other.xy() - self.xz() * other.xz() - self.xw() * other.xw() - self.yz() * other.yz() - self.yw() * other.yw() - self.zw() * other.zw() - self.xyz() * other.xyz() - self.xyw() * other.xyw() - self.xzw() * other.xzw() - self.yzw() * other.yzw() + self.xyzw() * other.xyzw();
                let nx = self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() - self.w() * other.xw() + self.xy() * other.y() + self.xz() * other.z() + self.xw() * other.w() - self.yz() * other.xyz() - self.yw() * other.xyw() - self.zw() * other.xzw() - self.xyz() * other.yz() - self.xyw() * other.yw() - self.xzw() * other.zw() + self.yzw() * other.xyzw() - self.xyzw() * other.yzw();
                let ny = self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.w() * other.yw() - self.xy() * other.x() + self.xz() * other.xyz() + self.xw() * other.xyw() + self.yz() * other.z() + self.yw() * other.w() - self.zw() * other.yzw() + self.xyz() * other.xz() + self.xyw() * other.xw() - self.xzw() * other.xyzw() - self.yzw() * other.zw() + self.xyzw() * other.xzw();
                let nz = self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.w() * other.zw() - self.xy() * other.xyz() - self.xz() * other.x() + self.xw() * other.xzw() - self.yz() * other.y() + self.yw() * other.yzw() + self.zw() * other.w() - self.xyz() * other.xy() + self.xyw() * other.xyzw() + self.xzw() * other.xw() + self.yzw() * other.yw() - self.xyzw() * other.xyw();
                let nw = self.r() * other.w() + self.x() * other.xw() + self.y() * other.yw() + self.z() * other.zw() + self.w() * other.r() - self.xy() * other.xyw() - self.xz() * other.xzw() - self.xw() * other.x() - self.yz() * other.yzw() - self.yw() * other.y() - self.zw() * other.z() - self.xyz() * other.xyzw() - self.xyw() * other.xy() - self.xzw() * other.xz() - self.yzw() * other.yz() + self.xyzw() * other.xyz();
                let nxy = self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.w() * other.xyw() + self.xy() * other.r() - self.xz() * other.yz() - self.xw() * other.yw() + self.yz() * other.xz() + self.yw() * other.xw() - self.zw() * other.xyzw() + self.xyz() * other.z() + self.xyw() * other.w() - self.xzw() * other.yzw() + self.yzw() * other.xzw() - self.xyzw() * other.zw();
                let nxz = self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.w() * other.xzw() + self.xy() * other.yz() + self.xz() * other.r() - self.xw() * other.zw() - self.yz() * other.xy() + self.yw() * other.xyzw() + self.zw() * other.xw() - self.xyz() * other.y() + self.xyw() * other.yzw() + self.xzw() * other.w() - self.yzw() * other.xyw() + self.xyzw() * other.yw();
                let nxw = self.r() * other.xw() + self.x() * other.w() - self.y() * other.xyw() - self.z() * other.xzw() - self.w() * other.x() + self.xy() * other.yw() + self.xz() * other.zw() + self.xw() * other.r() - self.yz() * other.xyzw() - self.yw() * other.xy() - self.zw() * other.xz() - self.xyz() * other.yzw() - self.xyw() * other.y() - self.xzw() * other.z() + self.yzw() * other.xyz() - self.xyzw() * other.yz();
                let nyz = self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() + self.w() * other.yzw() - self.xy() * other.xz() + self.xz() * other.xy() - self.xw() * other.xyzw() + self.yz() * other.r() - self.yw() * other.zw() + self.zw() * other.yw() + self.xyz() * other.x() - self.xyw() * other.xzw() + self.xzw() * other.xyw() + self.yzw() * other.w() - self.xyzw() * other.xw();
                let nyw = self.r() * other.yw() + self.x() * other.xyw() + self.y() * other.w() - self.z() * other.yzw() - self.w() * other.y() - self.xy() * other.xw() + self.xz() * other.xyzw() + self.xw() * other.xy() + self.yz() * other.zw() + self.yw() * other.r() - self.zw() * other.yz() + self.xyz() * other.xzw() + self.xyw() * other.x() - self.xzw() * other.xyz() - self.yzw() * other.z() + self.xyzw() * other.xz();
                let nzw = self.r() * other.zw() + self.x() * other.xzw() + self.y() * other.yzw() + self.z() * other.w() - self.w() * other.z() - self.xy() * other.xyzw() - self.xz() * other.xw() + self.xw() * other.xz() - self.yz() * other.yw() + self.yw() * other.yz() + self.zw() * other.r() - self.xyz() * other.xyw() + self.xyw() * other.xyz() + self.xzw() * other.x() + self.yzw() * other.y() - self.xyzw() * other.xy();
                let nxyz = self.r() * other.xyz() + self.x() * other.yz() - self.y() * other.xz() + self.z() * other.xy() - self.w() * other.xyzw() + self.xy() * other.z() - self.xz() * other.y() + self.xw() * other.yzw() + self.yz() * other.x() - self.yw() * other.xzw() + self.zw() * other.xyw() + self.xyz() * other.r() - self.xyw() * other.zw() + self.xzw() * other.yw() - self.yzw() * other.xw() + self.xyzw() * other.w();
                let nxyw = self.r() * other.xyw() + self.x() * other.yw() - self.y() * other.xw() + self.z() * other.xyzw() + self.w() * other.xy() + self.xy() * other.w() - self.xz() * other.yzw() - self.xw() * other.y() + self.yz() * other.xzw() + self.yw() * other.x() - self.zw() * other.xyz() + self.xyz() * other.zw() + self.xyw() * other.r() - self.xzw() * other.yz() + self.yzw() * other.xz() - self.xyzw() * other.z();
                let nxzw = self.r() * other.xzw() + self.x() * other.zw() - self.y() * other.xyzw() - self.z() * other.xw() + self.w() * other.xz() + self.xy() * other.yzw() + self.xz() * other.w() - self.xw() * other.z() - self.yz() * other.xyw() + self.yw() * other.xyz() + self.zw() * other.x() - self.xyz() * other.yw() + self.xyw() * other.yz() + self.xzw() * other.r() - self.yzw() * other.xy() + self.xyzw() * other.y();
                let nyzw = self.r() * other.yzw() + self.x() * other.xyzw() + self.y() * other.zw() - self.z() * other.yw() + self.w() * other.yz() - self.xy() * other.xzw() + self.xz() * other.xyw() - self.xw() * other.xyz() + self.yz() * other.w() - self.yw() * other.z() + self.zw() * other.y() + self.xyz() * other.xw() - self.xyw() * other.xz() + self.xzw() * other.xy() + self.yzw() * other.r() - self.xyzw() * other.x();
                let nxyzw = self.r() * other.xyzw() + self.x() * other.yzw() - self.y() * other.xzw() + self.z() * other.xyw() - self.w() * other.xyz() + self.xy() * other.zw() - self.xz() * other.yw() + self.xw() * other.yz() + self.yz() * other.xw() - self.yw() * other.xz() + self.zw() * other.xy() + self.xyz() * other.w() - self.xyw() * other.z() + self.xzw() * other.y() - self.yzw() * other.x() + self.xyzw() * other.r();
                self.0 = <$t as Simd16>::Type::new(
                    nr,
                    nx,ny,nz,nw,
                    nxy,nxz,nxw,nyz,nyw,nzw,
                    nxyz,nxyw,nxzw,nyzw,
                    nxyzw,
                );
            }
        }

        impl Div<$t> for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn div(self,other: $t) -> Self {
                MultiVec4(<$t as Simd16>::Type::div(&self.0,&<$t as Simd16>::Type::splat(other)))
            }
        }
        
        impl DivAssign<$t> for MultiVec4<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd16>::Type::div(&self.0,&<$t as Simd16>::Type::splat(other));
            }
        }

        impl Neg for MultiVec4<$t> {
            type Output = MultiVec4<$t>;
            fn neg(self) -> MultiVec4<$t> {
                MultiVec4(<$t as Simd16>::Type::sub(&<$t as Simd16>::Type::zero(),&self.0))
            }
        }
    );
);

impl_multivec4!(f32; 1.0; 0.0);
impl_multivec4!(f64; 1.0; 0.0);
