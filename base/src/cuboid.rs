// E - Cuboid
// Desmond Germans, 2020

use crate::*;
use std::fmt::{
    Display,
    Debug,
    Formatter,
    Result
};

/// Cuboid.
#[derive(Copy,Clone,Debug)]
pub struct Cuboid<T: Number> {
    pub o: Vec3<T>,
    pub s: Vec3<T>,
}

impl<T: Number> Cuboid<T> {
    /// Create new cuboid.
    ///
    /// **Arguments**
    ///
    /// * `ox` - Left edge X-coordinate.
    /// * `oy` - Top edge Y-coordinate.
    /// * `oz` - Forward edge Z-coordinate.
    /// * `sx` - Width.
    /// * `sy` - Height.
    /// * `sz` - Depth.
    ///
    /// **Returns**
    ///
    /// New cuboid.
    pub fn new(ox: T,oy: T,oz: T,sx: T,sy: T,sz: T) -> Cuboid<T> {
        Cuboid { o: vec3!(ox,oy,oz),s: vec3!(sx,sy,sz), }
    }

    /// Create new cuboid from origin-size vectors.
    ///
    /// **Arguments**
    ///
    /// * `o` - Top-left-forward origin.
    /// * `s` - Size.
    ///
    /// **Returns**
    ///
    /// New cuboid.
    pub fn new_os(o: Vec3<T>,s: Vec3<T>) -> Cuboid<T> {
        Cuboid { o: o,s: s, }
    }

    /// Containment check.
    ///
    /// **Arguments**
    ///
    /// * `p` - Point to check.
    ///
    /// **Returns**
    ///
    /// * `true` - The point lies inside the block.
    /// * `false` - The point lies outside the block.
    pub fn contains(&self,p: &Vec3<T>) -> bool {
        (p.x >= self.o.x) &&
        (p.y >= self.o.y) &&
        (p.z >= self.o.z) &&
        (p.x < self.o.x + self.s.x) &&
        (p.y < self.o.y + self.s.y) &&
        (p.z < self.o.z + self.s.z)
    }
}

impl<T: Number> Zero for Cuboid<T> {
    fn zero() -> Self {
        Cuboid { o: Vec3::<T>::zero(),s: Vec3::<T>::zero(), }
    }
}

impl<T: Number> Display for Cuboid<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({},{},{} {}x{}x{})",self.o.x,self.o.y,self.o.z,self.s.x,self.s.y,self.s.z)
    }
}

#[macro_export]
/// Create cuboid.
macro_rules! cuboid {
    ($ox:expr,$oy:expr,$oz:expr,$sx:expr,$sy:expr,$sz:expr) => { Cuboid::new($ox,$oy,$oz,$sx,$sy,$sz) };
    ($o:expr,$s:expr) => { Cuboid::new_os($o,$s) };
}
