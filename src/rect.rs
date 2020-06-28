// E - Rect
// Desmond Germans, 2020

use crate::Vec2;
use crate::Zero;
use std::ops::Add;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy,Clone)]
pub struct Rect<T> {
    pub o: Vec2<T>,
    pub s: Vec2<T>,
}

impl<T: PartialOrd + Add<Output=T> + Copy> Rect<T> {
    pub fn new(o: Vec2<T>,s: Vec2<T>) -> Rect<T> {
        Rect {
            o: o,
            s: s,
        }
    }

    pub fn contains(&self,p: &Vec2<T>) -> bool {
        (p.x >= self.o.x) && (p.y >= self.o.y) && (p.x < self.o.x + self.s.x) && (p.y < self.o.y + self.s.y)
    }
}

impl<T: Zero> Zero for Rect<T> where Vec2<T>: Zero {
    fn zero() -> Self {
        Rect {
            o: Vec2::<T>::zero(),
            s: Vec2::<T>::zero(),
        }
    }
}

impl<T: Display> Display for Rect<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{} {}x{})",self.o.x,self.o.y,self.s.x,self.s.y)
    }
}

/// Rectangle of `i32`.
#[allow(non_camel_case_types)]
pub type i32_r = Rect<i32>;

/// Rectangle of `i64`.
#[allow(non_camel_case_types)]
pub type i64_r = Rect<i64>;

/// Rectangle of `isize`.
#[allow(non_camel_case_types)]
pub type isize_r = Rect<isize>;

/// Rectangle of `f32`.
#[allow(non_camel_case_types)]
pub type f32_r = Rect<f32>;

/// Rectangle of `f64`.
#[allow(non_camel_case_types)]
pub type f64_r = Rect<f64>;