// E - Rect
// Desmond Germans, 2020

use crate::*;
use std::ops::Add;
use std::fmt::{Display,Formatter,Result};

/// Rectangle.
#[derive(Copy,Clone,Debug)]
pub struct Rect<T> {
    pub o: Vec2<T>,
    pub s: Vec2<T>,
}

impl<T: PartialOrd + Add<Output=T> + Copy> Rect<T> {
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

/// Create rectangle.
#[macro_export]
macro_rules! rect (
    ($o:expr,$s:expr) => (
        crate::Rect {
            o: $o,
            s: $s,
        }
    );
    ($ox:expr,$oy:expr,$sx:expr,$sy:expr) => (
        crate::Rect {
            o: vec2!($ox,$oy),
            s: vec2!($sx,$sy),
        }
    );
);
