// E - Extent
// Desmond Germans, 2020

use crate::*;
use std::fmt::{
    Display,
    Debug,
    Formatter,
    Result
};

/// Extent.
#[derive(Copy,Clone,Debug)]
pub struct Extent<T: Number> {
    pub o: T,
    pub s: T,
}

impl<T: Number> Extent<T> {
    /// Create new extent.
    ///
    /// **Arguments**
    ///
    /// * `o` - Start coordinate.
    /// * `s` - Length.
    ///
    /// **Returns**
    ///
    /// New extent.
    pub fn new(o: T,s: T) -> Extent<T> {
        Extent { o: o,s: s, }
    }

    /// Containment check.
    ///
    /// **Arguments**
    ///
    /// * `p` - Point to check.
    ///
    /// **Returns**
    ///
    /// * `true` - The point lies inside the extent.
    /// * `false` - The point lies outside the extent.
    pub fn contains(&self,p: T) -> bool {
        (p >= self.o) &&
        (p < self.o + self.s)
    }
}

impl<T: Number> Zero for Extent<T> {
    fn zero() -> Self {
        Extent { o: <T>::zero(),s: <T>::zero(), }
    }
}

impl<T: Number> Display for Extent<T> {
    fn fmt(&self,f: &mut Formatter) -> Result {
        write!(f,"({} {})",self.o,self.s)
    }
}

#[macro_export]
/// Create extent.
macro_rules! extent {
    ($o:expr,$s:expr) => { Extent::new($o,$s) };
}
