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
pub struct MultiVec3<T: SimdableFloat>(Simd8<T>);

impl<T: SimdableFloat> MultiVec3<T> {
    pub fn new(
        r: T,
        x: T,y: T,z: T,
        xy: T,xz: T,yz: T,
        xyz: T
    ) -> Self {
        MultiVec3(Simd8::new([r,x,y,z,xy,xz,yz,xyz]))
    }

    pub fn unit_r() -> Self {
        MultiVec3(Simd8::new([T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_x() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_y() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_z() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_xy() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_xz() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero(),T::zero()]))
    }

    pub fn unit_yz() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one(),T::zero()]))
    }

    pub fn unit_xyz() -> Self {
        MultiVec3(Simd8::new([T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::one()]))
    }

    pub fn r(&self) -> T {
        self.0.get(0)
    }

    pub fn x(&self) -> T {
        self.0.get(1)
    }

    pub fn y(&self) -> T {
        self.0.get(2)
    }

    pub fn z(&self) -> T {
        self.0.get(3)
    }

    pub fn xy(&self) -> T {
        self.0.get(4)
    }

    pub fn xz(&self) -> T {
        self.0.get(5)
    }

    pub fn yz(&self) -> T {
        self.0.get(6)
    }

    pub fn xyz(&self) -> T {
        self.0.get(7)
    }

    pub fn set_r(&mut self,r: T) {
        self.0.set(0,r);
    }

    pub fn set_x(&mut self,x: T) {
        self.0.set(1,x);
    }

    pub fn set_y(&mut self,y: T) {
        self.0.set(2,y);
    }

    pub fn set_z(&mut self,z: T) {
        self.0.set(3,z);
    }

    pub fn set_xy(&mut self,xy: T) {
        self.0.set(4,xy);
    }

    pub fn set_xz(&mut self,xz: T) {
        self.0.set(5,xz);
    }

    pub fn set_yz(&mut self,yz: T) {
        self.0.set(6,yz);
    }

    pub fn set_xyz(&mut self,xyz: T) {
        self.0.set(7,xyz);
    }
}

impl<T: SimdableFloat> PartialEq for MultiVec3<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd8::eq(&self.0,&other.0,0xFF)
    }
}

impl<T: SimdableFloat> Zero for MultiVec3<T> {
    fn zero() -> Self {
        MultiVec3(Simd8::zero())
    }
}

impl<T: SimdableFloat> Display for MultiVec3<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        let sx = if self.x() < T::zero() {
            format!("{}x",self.x())
        } else {
            format!("+{}x",self.x())
        };
        let sy = if self.y() < T::zero() {
            format!("{}x",self.y())
        } else {
            format!("+{}x",self.y())
        };
        let sz = if self.z() < T::zero() {
            format!("{}z",self.z())
        } else {
            format!("+{}z",self.z())
        };
        let sxy = if self.xy() < T::zero() {
            format!("{}xy",self.xy())
        } else {
            format!("+{}xy",self.xy())
        };
        let sxz = if self.xz() < T::zero() {
            format!("{}xz",self.xz())
        } else {
            format!("+{}xz",self.xz())
        };
        let syz = if self.yz() < T::zero() {
            format!("{}yz",self.yz())
        } else {
            format!("+{}yz",self.yz())
        };
        let sxyz = if self.xyz() < T::zero() {
            format!("{}xyz",self.xyz())
        } else {
            format!("+{}xyz",self.xyz())
        };
        write!(f,"{}{}{}{}{}{}{}{}",self.r(),sx,sy,sz,sxy,sxz,syz,sxyz)
    }
}

impl<T: SimdableFloat> Add<MultiVec3<T>> for MultiVec3<T> {
    type Output = Self;
    fn add(self,other: MultiVec3<T>) -> Self {
        MultiVec3(Simd8::add(self.0,other.0))
    }
}

impl<T: SimdableFloat> AddAssign<MultiVec3<T>> for MultiVec3<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd8::add(self.0,other.0);
    }
}

impl<T: SimdableFloat> Sub<MultiVec3<T>> for MultiVec3<T> {
    type Output = Self;
    fn sub(self,other: MultiVec3<T>) -> Self {
        MultiVec3(Simd8::sub(self.0,other.0))
    }
}

impl<T: SimdableFloat> SubAssign<MultiVec3<T>> for MultiVec3<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd8::sub(self.0,other.0);
    }
}

macro_rules! scalar_multivec3_mul {
    ($t:ty) => {
        impl Mul<MultiVec3<$t>> for $t {
            type Output = MultiVec3<$t>;
            fn mul(self,other: MultiVec3<$t>) -> MultiVec3<$t> {
                MultiVec3(Simd8::mul(Simd8::splat(self),other.0))
            }
        }        
    }
}

scalar_multivec3_mul!(f32);
scalar_multivec3_mul!(f64);

impl<T: SimdableFloat> Mul<T> for MultiVec3<T> {
    type Output = MultiVec3<T>;
    fn mul(self,other: T) -> Self {
        MultiVec3(Simd8::mul(self.0,Simd8::splat(other)))
    }
}

impl<T: SimdableFloat> MulAssign<T> for MultiVec3<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd8::mul(self.0,Simd8::splat(other));
    }
}

impl<T: SimdableFloat> Mul<MultiVec3<T>> for MultiVec3<T> {
    type Output = MultiVec3<T>;
    fn mul(self,other: MultiVec3<T>) -> Self {
        MultiVec3::new(
            self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() - self.xy() * other.xy() - self.xz() * other.xz() - self.yz() * other.yz() - self.xyz() * other.xyz(),
            self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() + self.xy() * other.y() + self.xz() * other.z() - self.yz() * other.xyz() - self.xyz() * other.yz(),
            self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.xy() * other.x() + self.xz() * other.xyz() + self.yz() * other.z() + self.xyz() * other.xz(),
            self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.xy() * other.xyz() - self.xz() * other.x() - self.yz() * other.y() - self.xyz() * other.xy(),
            self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.xy() * other.r() - self.xz() * other.yz() + self.yz() * other.xz() + self.xyz() * other.z(),
            self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.xy() * other.yz() + self.xz() * other.r() - self.yz() * other.xy() - self.xyz() * other.y(),
            self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() - self.xy() * other.xz() + self.xz() * other.xy() + self.yz() * other.r() + self.xyz() * other.x(),
            self.r() * other.xyz() + self.x() * other.yz() + self.y() * other.xz() + self.z() * other.xy() + self.xy() * other.z() - self.xz() * other.y() + self.yz() * other.x() + self.xyz() * other.r()
        )
    }
}

impl<T: SimdableFloat> MulAssign<MultiVec3<T>> for MultiVec3<T> {
    fn mul_assign(&mut self,other: MultiVec3<T>) {
        let nr = self.r() * other.r() + self.x() * other.x() + self.y() * other.y() + self.z() * other.z() - self.xy() * other.xy() - self.xz() * other.xz() - self.yz() * other.yz() - self.xyz() * other.xyz();
        let nx = self.r() * other.x() + self.x() * other.r() - self.y() * other.xy() - self.z() * other.xz() + self.xy() * other.y() + self.xz() * other.z() - self.yz() * other.xyz() - self.xyz() * other.yz();
        let ny = self.r() * other.y() + self.x() * other.xy() + self.y() * other.r() - self.z() * other.yz() - self.xy() * other.x() + self.xz() * other.xyz() + self.yz() * other.z() + self.xyz() * other.xz();
        let nz = self.r() * other.z() + self.x() * other.xz() + self.y() * other.yz() + self.z() * other.r() - self.xy() * other.xyz() - self.xz() * other.x() - self.yz() * other.y() - self.xyz() * other.xy();
        let nxy = self.r() * other.xy() + self.x() * other.y() - self.y() * other.x() + self.z() * other.xyz() + self.xy() * other.r() - self.xz() * other.yz() + self.yz() * other.xz() + self.xyz() * other.z();
        let nxz = self.r() * other.xz() + self.x() * other.z() - self.y() * other.xyz() - self.z() * other.x() + self.xy() * other.yz() + self.xz() * other.r() - self.yz() * other.xy() - self.xyz() * other.y();
        let nyz = self.r() * other.yz() + self.x() * other.xyz() + self.y() * other.z() - self.z() * other.y() - self.xy() * other.xz() + self.xz() * other.xy() + self.yz() * other.r() + self.xyz() * other.x();
        let nxyz = self.r() * other.xyz() + self.x() * other.yz() + self.y() * other.xz() + self.z() * other.xy() + self.xy() * other.z() - self.xz() * other.y() + self.yz() * other.x() + self.xyz() * other.r();
        self.0 = Simd8::new([nr,nx,ny,nz,nxy,nxz,nyz,nxyz]);
    }
}

impl<T: SimdableFloat> Div<T> for MultiVec3<T> {
    type Output = MultiVec3<T>;
    fn div(self,other: T) -> Self {
        MultiVec3(Simd8::div(self.0,Simd8::splat(other)))
    }
}

impl<T: SimdableFloat> DivAssign<T> for MultiVec3<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd8::div(self.0,Simd8::splat(other));
    }
}

impl<T: SimdableFloat> Neg for MultiVec3<T> {
    type Output = MultiVec3<T>;
    fn neg(self) -> MultiVec3<T> {
        MultiVec3(Simd8::sub(Simd8::zero(),self.0))
    }
}

impl<T: SimdableFloat> From<T> for MultiVec3<T> {
    fn from(v: T) -> MultiVec3<T> {
        MultiVec3::new(v,T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero())
    }
}

impl<T: SimdableFloat> From<Complex<T>> for MultiVec3<T> {
    fn from(v: Complex<T>) -> MultiVec3<T> {
        MultiVec3::new(v.r(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),v.i())
    }
}

impl<T: SimdableFloat> From<Quat<T>> for MultiVec3<T> {
    fn from(v: Quat<T>) -> MultiVec3<T> {
        MultiVec3::new(v.r(),T::zero(),T::zero(),T::zero(),v.i(),v.j(),v.k(),T::zero())
    }
}

impl<T: SimdableFloat> From<Vec3<T>> for MultiVec3<T> {
    fn from(v: Vec3<T>) -> MultiVec3<T> {
        MultiVec3::new(T::zero(),v.x(),v.y(),v.z(),T::zero(),T::zero(),T::zero(),T::zero())
    }
}

impl<T: SimdableFloat> From<Vec3A<T>> for MultiVec3<T> {
    fn from(v: Vec3A<T>) -> MultiVec3<T> {
        MultiVec3::new(T::zero(),v.x(),v.y(),v.z(),T::zero(),T::zero(),T::zero(),T::zero())
    }
}
