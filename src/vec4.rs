// E - Vector
// Desmond Germans, 2020

// Vec4<T> implements a 4D vector.

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

#[derive(Copy,Clone,Debug)]
pub struct Vec4<T: Simdable>(Simd4<T>);

impl<T: Simdable> Vec4<T> {
    pub fn new(x: T,y: T,z: T,w: T) -> Self {
        Vec4(Simd4::new([x,y,z,w]))
    }

    pub fn unit_x() -> Self {
        Vec4(Simd4::new([T::one(),T::zero(),T::zero(),T::zero()]))
    }

    pub fn unit_y() -> Self {
        Vec4(Simd4::new([T::zero(),T::one(),T::zero(),T::zero()]))
    }

    pub fn unit_z() -> Self {
        Vec4(Simd4::new([T::zero(),T::zero(),T::one(),T::zero()]))
    }

    pub fn unit_w() -> Self {
        Vec4(Simd4::new([T::zero(),T::zero(),T::zero(),T::one()]))
    }

    pub fn x(&self) -> T {
        self.0.get(0)
    }

    pub fn y(&self) -> T {
        self.0.get(1)
    }

    pub fn z(&self) -> T {
        self.0.get(2)
    }

    pub fn w(&self) -> T {
        self.0.get(3)
    }

    pub fn set_x(&mut self,x: T) {
        self.0.set(0,x);
    }

    pub fn set_y(&mut self,y: T) {
        self.0.set(1,y);
    }

    pub fn set_z(&mut self,z: T) {
        self.0.set(2,z);
    }

    pub fn set_w(&mut self,w: T) {
        self.0.set(3,w);
    }
}

impl<T: Simdable> PartialEq for Vec4<T> {
    fn eq(&self,other: &Self) -> bool {
        Simd4::<T>::eq(&self.0,&other.0,0xF)
    }
}

impl<T: Simdable> Zero for Vec4<T> {
    fn zero() -> Self {
        Vec4(Simd4::<T>::zero())
    }
}

impl<T: Simdable> Display for Vec4<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{},{})",self.x(),self.y(),self.z(),self.w())
    }
}

impl<T: Simdable> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Vec4(Simd4::<T>::add(self.0,other.0))
    }
}

impl<T: Simdable> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::add(self.0,other.0);
    }
}

impl<T: Simdable> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Vec4(Simd4::<T>::sub(self.0,other.0))
    }
}

impl<T: Simdable> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self,other: Self) {
        self.0 = Simd4::<T>::sub(self.0,other.0);
    }
}

macro_rules! scalar_vec4_mul {
    ($t:ty) => {
        impl Mul<Vec4<$t>> for $t {
            type Output = Vec4<$t>;
            fn mul(self,other: Vec4<$t>) -> Vec4<$t> {
                Vec4(Simd4::<$t>::mul(Simd4::<$t>::splat(self),other.0))
            }
        }
    }
}

scalar_vec4_mul!(u8);
scalar_vec4_mul!(i8);
scalar_vec4_mul!(u16);
scalar_vec4_mul!(i16);
scalar_vec4_mul!(u32);
scalar_vec4_mul!(i32);
scalar_vec4_mul!(u64);
scalar_vec4_mul!(i64);
scalar_vec4_mul!(f32);
scalar_vec4_mul!(f64);
scalar_vec4_mul!(usize);
scalar_vec4_mul!(isize);

impl<T: Simdable> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self,other: T) -> Self {
        Vec4(Simd4::<T>::mul(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::mul(self.0,Simd4::splat(other));
    }
}        

impl<T: Simdable> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self,other: T) -> Self {
        Vec4(Simd4::<T>::div(self.0,Simd4::splat(other)))
    }
}
    
impl<T: Simdable> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self,other: T) {
        self.0 = Simd4::<T>::div(self.0,Simd4::splat(other));
    }
}

impl<T: Simdable> Neg for Vec4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec4(Simd4::<T>::sub(Simd4::zero(),self.0))
    }
}

impl<T: Simdable> Vec4<T> {
    pub fn dot(_a: Self,_b: Self) -> T {
        // TODO: a.x * b.x + a.y * b.y
        T::zero()
    }

    pub fn abs(&self) -> T {
        // TODO: (self.x * self.x + self.y * self.y).sqrt()
        T::zero()
    }

    pub fn norm(&self) -> Self {
        // TODO:
        /*
        let d = self.abs();
        if d != <$t>::zero() {
            *self / d
        }
        else {
            *self
        }
        */
        Self::zero()
    }
}

#[macro_export]
macro_rules! vec4 {
    ($x:expr,$y:expr,$z:expr,$w:expr) => { Vec4::new($x,$y,$z,$w) };
}
