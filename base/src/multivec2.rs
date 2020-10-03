// E - 2D Multivector
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

/// 2D multivector.
#[derive(Copy,Clone,Debug)]
pub struct MultiVec2<T: SimdableFloat>(Simd4<T>);

impl<T: SimdableFloat> MultiVec2<T> {
    /// Create new 2D multivector.
    ///
    /// **Arguments**
    ///
    /// * `r` - Scalar component.
    /// * `x` - X-vector component.
    /// * `y` - Y-vector component.
    /// * `xy` - Pseudoscalar component.
    pub fn new(
        r: T,
        x: T,y: T,
        xy: T
    ) -> Self {
        MultiVec2(Simd4::new([r,x,y,xy]))
    }

    /// Create new multivector containing a unit scalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_r() -> Self {
        MultiVec2(Simd4::new([T::one(),T::zero(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit X-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_x() -> Self {
        MultiVec2(Simd4::new([T::zero(),T::one(),T::zero(),T::zero()]))
    }

    /// Create new multivector containing a unit Y-vector.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_y() -> Self {
        MultiVec2(Simd4::new([T::zero(),T::zero(),T::one(),T::zero()]))
    }

    /// Create new multivector containing a unit pseudoscalar.
    ///
    /// **Returns**
    ///
    /// The new multivector.
    pub fn unit_xy() -> Self {
        MultiVec2(Simd4::new([T::zero(),T::zero(),T::zero(),T::one()]))
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

    /// Get pseudoscalar component.
    ///
    /// **Returns**
    ///
    /// The pseudoscalar component.
    pub fn xy(&self) -> T {
        self.0.get(3)
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

    /// Set pseudoscalar component.
    ///
    /// **Arguments**
    ///
    /// `xy` - New pseudoscalar component.
    pub fn set_xy(&mut self,xy: T) {
        self.0.set(3,xy);
    }
}

impl<T: SimdableFloat> PartialEq for MultiVec2<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd4::eq(&self.0,&other.0,0xF)
    }
}

impl<T: SimdableFloat> Zero for MultiVec2<T> {
    fn zero() -> Self {
        MultiVec2(Simd4::zero())
    }
}

impl<T: SimdableFloat> Display for MultiVec2<T> {
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
        let sxy = if self.xy() < T::zero() {
            format!("{}xy",self.xy())
        } else {
            format!("+{}xy",self.xy())
        };
        write!(f,"{}{}{}{}",self.r(),sx,sy,sxy)
    }
}

impl<T: SimdableFloat> Add<MultiVec2<T>> for MultiVec2<T> {
    type Output = Self;
    fn add(self,other: MultiVec2<T>) -> Self {
        MultiVec2(Simd4::add(self.0,other.0))
    }
}

impl<T: SimdableFloat> AddAssign<MultiVec2<T>> for MultiVec2<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd4::add(self.0,other.0);
    }
}

impl<T: SimdableFloat> Sub<MultiVec2<T>> for MultiVec2<T> {
    type Output = Self;
    fn sub(self,other: MultiVec2<T>) -> Self {
        MultiVec2(Simd4::sub(self.0,other.0))
    }
}

impl<T: SimdableFloat> SubAssign<MultiVec2<T>> for MultiVec2<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd4::sub(self.0,other.0);
    }
}

macro_rules! scalar_multivec2_mul {
    ($t:ty) => {
        impl Mul<MultiVec2<$t>> for $t {
            type Output = MultiVec2<$t>;
            fn mul(self,other: MultiVec2<$t>) -> MultiVec2<$t> {
                MultiVec2(Simd4::mul(Simd4::splat(self),other.0))
            }
        }        
    }
}

scalar_multivec2_mul!(f32);
scalar_multivec2_mul!(f64);

impl<T: SimdableFloat> Mul<T> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn mul(self,other: T) -> Self {
        MultiVec2(Simd4::mul(self.0,Simd4::splat(other)))
    }
}

impl<T: SimdableFloat> MulAssign<T> for MultiVec2<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd4::mul(self.0,Simd4::splat(other));
    }
}

impl<T: SimdableFloat> Mul<MultiVec2<T>> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn mul(self,other: MultiVec2<T>) -> Self {
        MultiVec2::new(
            self.r() * other.r() - self.x() * other.x() - self.y() * other.y() - self.xy() * other.xy(),
            self.r() * other.x() + self.x() * other.r() + self.y() * other.xy() - self.xy() * other.y(),
            self.r() * other.y() + self.y() * other.r() - self.x() * other.xy() + self.xy() * other.x(),
            self.r() * other.xy() + self.xy() * other.r() + self.x() * other.y() - self.y() * other.x()
        )
    }
}

impl<T: SimdableFloat> MulAssign<MultiVec2<T>> for MultiVec2<T> {
    fn mul_assign(&mut self,other: MultiVec2<T>) {
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

impl<T: SimdableFloat> Div<T> for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn div(self,other: T) -> Self {
        MultiVec2(Simd4::div(self.0,Simd4::splat(other)))
    }
}

impl<T: SimdableFloat> DivAssign<T> for MultiVec2<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd4::div(self.0,Simd4::splat(other));
    }
}

impl<T: SimdableFloat> Neg for MultiVec2<T> {
    type Output = MultiVec2<T>;
    fn neg(self) -> MultiVec2<T> {
        MultiVec2(Simd4::sub(Simd4::zero(),self.0))
    }
}

impl<T: SimdableFloat> From<T> for MultiVec2<T> {
    fn from(v: T) -> MultiVec2<T> {
        MultiVec2::<T>::new(v,T::zero(),T::zero(),T::zero())
    }
}

impl<T: SimdableFloat> From<Vec2<T>> for MultiVec2<T> {
    fn from(v: Vec2<T>) -> MultiVec2<T> {
        MultiVec2::<T>::new(T::zero(),v.x(),v.y(),T::zero())
    }
}

impl<T: SimdableFloat> From<Complex<T>> for MultiVec2<T> {
    fn from(v: Complex<T>) -> MultiVec2<T> {
        MultiVec2::<T>::new(v.r(),T::zero(),T::zero(),v.i())
    }
}
