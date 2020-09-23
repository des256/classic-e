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

/// Elementary 2-multivector.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec2<T: Simd4>(<T as Simd4>::Type);

macro_rules! impl_multivec2 (
    ($t:ty; $o:expr; $z:expr) => (
        impl MultiVec2<$t> {
            pub fn new(
                r: $t,
                x: $t,y: $t,
                xy: $t
            ) -> Self {
                MultiVec2(<$t as Simd4>::Type::new(r,x,y,xy))
            }

            pub fn unit_r() -> Self {
                MultiVec2(<$t as Simd4>::Type::new($o,$z,$z,$z))
            }

            pub fn unit_x() -> Self {
                MultiVec2(<$t as Simd4>::Type::new($z,$o,$z,$z))
            }

            pub fn unit_y() -> Self {
                MultiVec2(<$t as Simd4>::Type::new($z,$z,$o,$z))
            }

            pub fn unit_xy() -> Self {
                MultiVec2(<$t as Simd4>::Type::new($z,$z,$z,$o))
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

            pub fn xy(&self) -> $t {
                self.0.get(3)
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

            pub fn set_xy(&mut self,xy: $t) {
                self.0.set(3,xy);
            }
        }

        impl PartialEq for MultiVec2<$t> {
            fn eq(&self,other: &Self) -> bool {
                <$t as Simd4>::Type::eq(&self.0,&other.0,0xF)
            }
        }

        impl Zero for MultiVec2<$t> {
            fn zero() -> Self {
                MultiVec2(<$t as Simd4>::Type::zero())
            }
        }

        impl Display for MultiVec2<$t> {
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
                let sxy = if self.xy() < $z {
                    format!("{}xy",self.xy())
                } else {
                    format!("+{}xy",self.xy())
                };
                write!(f,"{}{}{}{}",self.r(),sx,sy,sxy)
            }
        }

        impl Add<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = Self;
            fn add(self,other: MultiVec2<$t>) -> Self {
                MultiVec2(<$t as Simd4>::Type::add(&self.0,&other.0))
            }
        }

        impl AddAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn add_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::add(&self.0,&other.0);
            }
        }

        impl Sub<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = Self;
            fn sub(self,other: MultiVec2<$t>) -> Self {
                MultiVec2(<$t as Simd4>::Type::sub(&self.0,&other.0))
            }
        }

        impl SubAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn sub_assign(&mut self,other: Self) {
                self.0 = <$t as Simd4>::Type::sub(&self.0,&other.0);
            }
        }

        impl Mul<MultiVec2<$t>> for $t {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> MultiVec2<$t> {
                MultiVec2(<$t as Simd4>::Type::mul(&<$t as Simd4>::Type::splat(self),&other.0))
            }
        }

        impl Mul<$t> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn mul(self,other: $t) -> Self {
                MultiVec2(<$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }

        impl MulAssign<$t> for MultiVec2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::mul(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }

        impl Mul<MultiVec2<$t>> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> Self {
                MultiVec2(<$t as Simd4>::Type::new(
                    self.r() * other.r() - self.x() * other.x() - self.y() * other.y() - self.xy() * other.xy(),
                    self.r() * other.x() + self.x() * other.r() + self.y() * other.xy() - self.xy() * other.y(),
                    self.r() * other.y() + self.y() * other.r() - self.x() * other.xy() + self.xy() * other.x(),
                    self.r() * other.xy() + self.xy() * other.r() + self.x() * other.y() - self.y() * other.x()
                ))
            }
        }

        impl MulAssign<MultiVec2<$t>> for MultiVec2<$t> {
            fn mul_assign(&mut self,other: MultiVec2<$t>) {
                let nr = self.r() * other.r() - self.x() * other.x() - self.y() * other.y() - self.xy() * other.xy();
                let nx = self.r() * other.x() + self.x() * other.r() + self.y() * other.xy() - self.xy() * other.y();
                let ny = self.r() * other.y() + self.y() * other.r() - self.x() * other.xy() + self.xy() * other.x();
                let nxy = self.r() * other.xy() + self.xy() * other.r() + self.x() * other.y() - self.y() * other.x();
                self.set_r(nr);
                self.set_x(nx);
                self.set_y(ny);
                self.set_xy(nxy);
            }
        }

        impl Div<$t> for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn div(self,other: $t) -> Self {
                MultiVec2(<$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other)))
            }
        }
        
        impl DivAssign<$t> for MultiVec2<$t> {
            fn div_assign(&mut self,other: $t) {
                self.0 = <$t as Simd4>::Type::div(&self.0,&<$t as Simd4>::Type::splat(other));
            }
        }

        impl Neg for MultiVec2<$t> {
            type Output = MultiVec2<$t>;
            fn neg(self) -> MultiVec2<$t> {
                MultiVec2(<$t as Simd4>::Type::sub(&<$t as Simd4>::Type::zero(),&self.0))
            }
        }

        impl From<$t> for MultiVec2<$t> {
            fn from(v: $t) -> MultiVec2<$t> {
                MultiVec2::<$t>::new(v,$z,$z,$z)
            }
        }

        impl From<Vec2<$t>> for MultiVec2<$t> {
            fn from(v: Vec2<$t>) -> MultiVec2<$t> {
                MultiVec2::<$t>::new($z,v.x(),v.y(),$z)
            }
        }

        impl From<Complex<$t>> for MultiVec2<$t> {
            fn from(v: Complex<$t>) -> MultiVec2<$t> {
                MultiVec2::<$t>::new(v.r(),$z,$z,v.i())
            }
        }
    );
);

impl_multivec2!(f32; 1.0; 0.0);
impl_multivec2!(f64; 1.0; 0.0);
