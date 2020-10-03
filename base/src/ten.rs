// E - Ten
// Desmond Germans, 2020

use crate::*;
use std::marker::PhantomData;

/// Generic 3-dimensional array of elements.
pub struct Ten<T: Clone + Copy + Zero> {
    pub size: Vec3<usize>,
    pub data: Box<[T]>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Copy + Zero> Ten<T> {
    /// Create new 3D array.
    ///
    /// **Arguments**
    ///
    /// * `size` - Size of the array.
    ///
    /// **Returns**
    ///
    /// The new array, filled with `zero()`.
    pub fn new(size: Vec3<usize>) -> Ten<T> {
        Ten {
            size: size,
            data: vec![T::zero(); (size.x() * size.y() * size.z()) as usize].into_boxed_slice(),
            phantom: PhantomData,
        }
    }

    /// (maybe) Set element in the array.
    ///
    /// **Arguments**
    ///
    /// * `p` - Coordinates of the element.
    /// * `v` - Element value.
    pub fn set(&mut self,p: Vec3<usize>,v: T) {
        self.data[(p.z() * self.size.y() + p.y()) * self.size.x() + p.x()] = v;
    }

    /// (maybe) Get element from the array.
    ///
    /// **Arguments**
    ///
    /// * `p` - Coordinates of the element.
    ///
    /// **Returns**
    ///
    /// Element value.
    pub fn get(&self,p: Vec3<usize>) -> T {
        self.data[(p.z() * self.size.y() + p.y()) * self.size.x() + p.x()]
    }
}
