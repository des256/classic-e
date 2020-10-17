// E - Mat
// Desmond Germans, 2020

use crate::*;
use std::marker::PhantomData;

/// Generic 2-dimensional array of elements.
#[derive(Clone)]
pub struct Mat<T: Clone + Copy + Zero> {
    pub size: Vec2<usize>,
    pub data: Box<[T]>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Copy + Zero> Mat<T> {
    /// Create new 2D array.
    ///
    /// **Arguments**
    ///
    /// * `size` - Size of the array.
    ///
    /// **Returns**
    ///
    /// The new array, filled with `zero()`.
    pub fn new(size: Vec2<usize>) -> Mat<T> {
        Mat {
            size: size,
            data: vec![T::zero(); (size.x * size.y) as usize].into_boxed_slice(),
            phantom: PhantomData,
        }
    }

    /// (maybe) Set element in the array.
    ///
    /// **Arguments**
    ///
    /// * `p` - Coordinates of the element.
    /// * `v` - Element value.
    pub fn set(&mut self,p: Vec2<usize>,v: T) {
        self.data[p.y * self.size.x + p.x] = v;
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
    pub fn get(&self,p: Vec2<usize>) -> T {
        self.data[p.y * self.size.x + p.x]
    }
}
