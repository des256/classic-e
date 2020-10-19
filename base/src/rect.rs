// E - Rect
// Desmond Germans, 2020

use crate::*;
use std::fmt::{
    Display,
    Debug,
    Formatter,
    Result
};

/// Rectangle.
#[derive(Copy,Clone,Debug)]
pub struct Rect<T: Number> {
    pub o: Vec2<T>,
    pub s: Vec2<T>,
}

impl<T: Number> Rect<T> {
    /// Create new rectangle.
    ///
    /// **Arguments**
    ///
    /// * `ox` - Left edge X-coordinate.
    /// * `oy` - Top edge Y-coordinate.
    /// * `sx` - Width.
    /// * `sy` - Height.
    ///
    /// **Returns**
    ///
    /// New rectangle.
    pub fn new(ox: T,oy: T,sx: T,sy: T) -> Rect<T> {
        Rect { o: vec2!(ox,oy),s: vec2!(sx,sy), }
    }

    /// Create new rectangle from origin-size vectors.
    ///
    /// **Arguments**
    ///
    /// * `o` - Top-left origin.
    /// * `s` - Size.
    ///
    /// **Returns**
    ///
    /// New rectangle.
    pub fn new_os(o: Vec2<T>,s: Vec2<T>) -> Rect<T> {
        Rect { o: o,s: s, }
    }

    /// Containment check.
    ///
    /// **Arguments**
    ///
    /// * `p` - Point to check.
    ///
    /// **Returns**
    ///
    /// * `true` - The point lies inside the rectangle.
    /// * `false` - The point lies outside the rectangle.
    pub fn contains(&self,p: &Vec2<T>) -> bool {
        (p.x >= self.o.x) &&
        (p.y >= self.o.y) &&
        (p.x < self.o.x + self.s.x) &&
        (p.y < self.o.y + self.s.y)
    }
}

impl<T: Number> Zero for Rect<T> {
    fn zero() -> Self {
        Rect { o: Vec2::<T>::zero(),s: Vec2::<T>::zero(), }
    }
}

impl<T: Number> Display for Rect<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{} {}x{})",self.o.x,self.o.y,self.s.x,self.s.y)
    }
}

#[macro_export]
/// Create rectangle.
macro_rules! rect {
    ($ox:expr,$oy:expr,$sx:expr,$sy:expr) => { Rect::new($ox,$oy,$sx,$sy) };
    ($o:expr,$s:expr) => { Rect::new_os($o,$s) };
}
