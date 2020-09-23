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
pub struct MultiVec3<T: Simd8>(<T as Simd8>::Type);

macro_rules! impl_multivec3 (
    ($t:ty; $o:expr; $z:expr) => (
        impl MultiVec3<$t> {
            pub fn new(
                r: $t,
                x: $t,y: $t,z: $t,
                xy: $t,xz: $t,yz: $t,
                xyz: $t
            ) -> Self {
                MultiVec3(<$t as Simd8>::Type::new(r,x,y,z,xy,xz,yz,xyz))
            }

            pub fn unit_r() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($o,$z,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_x() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$o,$z,$z,$z,$z,$z,$z))
            }

            pub fn unit_y() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$o,$z,$z,$z,$z,$z))
            }

            pub fn unit_z() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$z,$o,$z,$z,$z,$z))
            }

            pub fn unit_xy() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$z,$z,$o,$z,$z,$z))
            }

            pub fn unit_xz() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$z,$z,$z,$o,$z,$z))
            }

            pub fn unit_yz() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$z,$z,$z,$z,$o,$z))
            }

            pub fn unit_xyz() -> Self {
                MultiVec3(<$t as Simd8>::Type::new($z,$z,$z,$z,$z,$z,$z,$o))
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

            pub fn xy(&self) -> $t {
                self.0.get(4)
            }

            pub fn xz(&self) -> $t {
                self.0.get(5)
            }

            pub fn yz(&self) -> $t {
                self.0.get(6)
            }

            pub fn xyz(&self) -> $t {
                self.0.get(7)
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

            pub fn set_xy(&mut self,xy: $t) {
                self.0.set(4,xy);
            }

            pub fn set_xz(&mut self,xz: $t) {
                self.0.set(5,xz);
            }

            pub fn set_yz(&mut self,yz: $t) {
                self.0.set(6,yz);
            }

            pub fn set_xyz(&mut self,xyz: $t) {
                self.0.set(7,xyz);
            }
        }

        impl PartialEq for MultiVec3<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd8>::Type::eq(&self.0,&other.0,0xFF)
            }
        }

        impl Zero for MultiVec3<$t> {
            fn zero() -> Self {
                MultiVec3(<$t as Simd8>::Type::zero())
            }
        }

        impl Display for MultiVec3<$t> {
            fn fmt(&self,f: &mut Formatter) -> Result {
                let sx = if self.x() < $z {
                    format!("{}x",self.x())
                } else {
                    format!("+{}x",self.x())
                };
                let sy = if self.y() < $z {
                    format!("{}x",self.y())
                } else {
                    format!("+{}x",self.y())
                };
                let sz = if self.z() < $z {
                    format!("{}z",self.z())
                } else {
                    format!("+{}z",self.z())
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
                let syz = if self.yz() < $z {
                    format!("{}yz",self.yz())
                } else {
                    format!("+{}yz",self.yz())
                };
                let sxyz = if self.xyz() < $z {
                    format!("{}xyz",self.xyz())
                } else {
                    format!("+{}xyz",self.xyz())
                };
                write!(f,"{}{}{}{}{}{}{}{}",self.r(),sx,sy,sz,sxy,sxz,syz,sxyz)
            }
        }

        impl Add<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = Self;
            fn add(self,other: MultiVec3<$t>) -> Self {
                MultiVec3(<$t as Simd8>::Type::add(&self.0,&other.0))
            }
        }

        impl AddAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd8>::Type::add(&self.0,&other.0);
            }
        }

        impl Sub<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = Self;
            fn sub(self,other: MultiVec3<$t>) -> Self {
                MultiVec3(<$t as Simd8>::Type::sub(&self.0,&other.0))
            }
        }

        impl SubAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd8>::Type::sub(&self.0,&other.0);
            }
        }

        impl Mul<MultiVec3<$t>> for $t {
            type Output = MultiVec3<$t>;
            fn mul(self,other: MultiVec3<$t>) -> MultiVec3<$t> {
                MultiVec3(<$t as Simd8>::Type::mul(&<$t as Simd8>::Type::splat(self),&other.0))
            }
        }

        impl Mul<$t> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn mul(self,other: $t) -> Self {
                MultiVec3(<$t as Simd8>::Type::mul(&self.0,&<$t as Simd8>::Type::splat(other)))
            }
        }

        impl MulAssign<$t> for MultiVec3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd8>::Type::mul(&self.0,&<$t as Simd8>::Type::splat(other));
            }
        }

        impl Mul<MultiVec3<$t>> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn mul(self,other: MultiVec3<$t>) -> Self {
                MultiVec3(<$t as Simd8>::Type::new(
                    self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() - self.xy() * other.xy() - self.xz() * other.xz() - self.yz() * other.yz() - self.xyz() * other.xyz(),
                    self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() + self.xy() * other.y() + self.xz() * other.z() - self.yz() * other.xyz() - self.xyz() * other.yz(),
                    self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.xy() * other.x() + self.xz() * other.xyz() + self.yz() * other.z() + self.xyz() * other.xz(),
                    self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.xy() * other.xyz() - self.xz() * other.x() - self.yz() * other.y() - self.xyz() * other.xy(),
                    self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.xy() * other.r() - self.xz() * other.yz() + self.yz() * other.xz() + self.xyz() * other.z(),
                    self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.xy() * other.yz() + self.xz() * other.r() - self.yz() * other.xy() - self.xyz() * other.y(),
                    self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() - self.xy() * other.xz() + self.xz() * other.xy() + self.yz() * other.r() + self.xyz() * other.x(),
                    self.r() * other.xyz() + self.x() * other.yz() + self.y() * other.xz() + self.z() * other.xy() + self.xy() * other.z() - self.xz() * other.y() + self.yz() * other.x() + self.xyz() * other.r()
                ))
            }
        }

        impl MulAssign<MultiVec3<$t>> for MultiVec3<$t> {
            fn mul_assign(&mut self,other: MultiVec3<$t>) {
                let nr = self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() - self.xy() * other.xy() - self.xz() * other.xz() - self.yz() * other.yz() - self.xyz() * other.xyz();
                let nx = self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() + self.xy() * other.y() + self.xz() * other.z() - self.yz() * other.xyz() - self.xyz() * other.yz();
                let ny = self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.xy() * other.x() + self.xz() * other.xyz() + self.yz() * other.z() + self.xyz() * other.xz();
                let nz = self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.xy() * other.xyz() - self.xz() * other.x() - self.yz() * other.y() - self.xyz() * other.xy();
                let nxy = self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.xy() * other.r() - self.xz() * other.yz() + self.yz() * other.xz() + self.xyz() * other.z();
                let nxz = self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.xy() * other.yz() + self.xz() * other.r() - self.yz() * other.xy() - self.xyz() * other.y();
                let nyz = self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() - self.xy() * other.xz() + self.xz() * other.xy() + self.yz() * other.r() + self.xyz() * other.x();
                let nxyz = self.r() * other.xyz() + self.x() * other.yz() + self.y() * other.xz() + self.z() * other.xy() + self.xy() * other.z() - self.xz() * other.y() + self.yz() * other.x() + self.xyz() * other.r();
                self.0 = <$t as Simd8>::Type::new(nr,nx,ny,nz,nxy,nxz,nyz,nxyz);
            }
        }

        impl Div<$t> for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn div(self,other: $t) -> Self {
                MultiVec3(<$t as Simd8>::Type::div(&self.0,&<$t as Simd8>::Type::splat(other)))
            }
        }
        
        impl DivAssign<$t> for MultiVec3<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd8>::Type::div(&self.0,&<$t as Simd8>::Type::splat(other));
            }
        }

        impl Neg for MultiVec3<$t> {
            type Output = MultiVec3<$t>;
            fn neg(self) -> MultiVec3<$t> {
                MultiVec3(<$t as Simd8>::Type::sub(&<$t as Simd8>::Type::zero(),&self.0))
            }
        }

        impl From<$t> for MultiVec3<$t> {
            fn from(v: $t) -> MultiVec3<$t> {
                MultiVec3::<$t>::new(v,$z,$z,$z,$z,$z,$z,$z)
            }
        }

        impl From<Vec3<$t>> for MultiVec3<$t> {
            fn from(v: Vec3<$t>) -> MultiVec3<$t> {
                MultiVec3::<$t>::new($z,v.x(),v.y(),v.z(),$z,$z,$z,$z)
            }
        }
    );
);

impl_multivec3!(f32; 1.0; 0.0);
impl_multivec3!(f64; 1.0; 0.0);
