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
pub struct Rect<T: Simdable>(Simd4<T>);

impl<T: Simdable> Rect<T> {
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
        Rect(Simd4::new([ox,oy,sx,sy]))
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
        Rect(Simd4::new([o.x(),o.y(),s.x(),s.y()]))
    }

    /// Get left edge X-coordinate.
    ///
    /// **Returns**
    ///
    /// Left edge X-coordinate.
    pub fn ox(&self) -> T {
        self.0.get(0)
    }

    /// Get top edge Y-coordinate.
    ///
    /// **Returns**
    ///
    /// Top edge Y-coordinate.
    pub fn oy(&self) -> T {
        self.0.get(1)
    }

    /// Get width.
    ///
    /// **Returns**
    ///
    /// The width.
    pub fn sx(&self) -> T {
        self.0.get(2)
    }

    /// Get height.
    ///
    /// **Returns**
    ///
    /// The height.
    pub fn sy(&self) -> T {
        self.0.get(3)
    }

    /// Get top-left origin.
    ///
    /// **Returns**
    ///
    /// Top-left origin.
    pub fn o(&self) -> Vec2<T> {
        vec2!(self.0.get(0),self.0.get(1))
    }

    /// Get size.
    ///
    /// **Returns**
    ///
    /// Size.
    pub fn s(&self) -> Vec2<T> {
        vec2!(self.0.get(2),self.0.get(3))
    }

    /// Set left edge X-coordinate.
    ///
    /// **Arguments**
    ///
    /// * `ox` - New left edge X-coordinate.
    pub fn set_ox(&mut self,ox: T) {
        self.0.set(0,ox);
    }

    /// Set top edge Y-coordinate.
    ///
    /// **Arguments**
    ///
    /// * `oy` - New top edge Y-coordinate.
    pub fn set_oy(&mut self,oy: T) {
        self.0.set(1,oy);
    }

    /// Set width.
    ///
    /// **Arguments**
    ///
    /// * `sx` - New width.
    pub fn set_sx(&mut self,sx: T) {
        self.0.set(2,sx);
    }

    /// Set height.
    ///
    /// **Arguments**
    ///
    /// * `sy` - New height.
    pub fn set_sy(&mut self,sy: T) {
        self.0.set(3,sy);
    }

    /// Set top-left origin.
    ///
    /// **Arguments**
    ///
    /// * `o` - New top-left origin.
    pub fn set_o(&mut self,o: Vec2<T>) {
        self.0.set(0,o.x());
        self.0.set(1,o.y());
    }

    /// Set size.
    ///
    /// **Arguments**
    ///
    /// * `s` - New size.
    pub fn set_s(&mut self,s: Vec2<T>) {
        self.0.set(2,s.x());
        self.0.set(3,s.y());
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
        (p.x() >= self.ox()) &&
        (p.y() >= self.oy()) &&
        (p.x() < self.ox() + self.sx()) &&
        (p.y() < self.oy() + self.sy())
    }
}

impl<T: Simdable> Zero for Rect<T> {
    fn zero() -> Self {
        Rect(Simd4::zero())
    }
}

impl<T: Simdable> Display for Rect<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{} {}x{})",self.ox(),self.oy(),self.sx(),self.sy())
    }
}

#[macro_export]
macro_rules! rect {
    ($ox:expr,$oy:expr,$sx:expr,$sy:expr) => { Rect::new($ox,$oy,$sx,$sy) };
    ($o:expr,$s:expr) => { Rect::new_os($o,$s) };
}
