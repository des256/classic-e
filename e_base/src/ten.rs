// E - Ten
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        marker::PhantomData,
        ops::{
            Index,
            IndexMut,
        },
    },
};

/// Generic 3-dimensional array of elements.
pub struct Ten<T: Clone + Copy + Zero> {
    pub size: Vec3<usize>,
    data: Box<[T]>,
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
            data: vec![T::zero(); (size.x * size.y * size.z) as usize].into_boxed_slice(),
            phantom: PhantomData,
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T: Clone + Copy + Zero> Index<(usize,usize,usize)> for Ten<T> {
    type Output = T;
    fn index(&self,index: (usize,usize,usize)) -> &Self::Output {
        &self.data[(index.2 * self.size.y + index.1) * self.size.x + index.0]
    }
}

impl<T: Clone + Copy + Zero> IndexMut<(usize,usize,usize)> for Ten<T> {
    fn index_mut(&mut self,index: (usize,usize,usize)) -> &mut Self::Output {
        &mut self.data[(index.2 * self.size.y + index.1) * self.size.x + index.0]
    }
}

impl<T: Clone + Copy + Zero> Index<usize> for Ten<T> {
    type Output = T;
    fn index(&self,index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Clone + Copy + Zero> IndexMut<usize> for Ten<T> {
    fn index_mut(&mut self,index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Clone + Copy + Zero> Index<Vec3<usize>> for Ten<T> {
    type Output = T;
    fn index(&self,index: Vec3<usize>) -> &Self::Output {
        &self.data[(index.z * self.size.y + index.y) * self.size.x + index.x]
    }
}

impl<T: Clone + Copy + Zero> IndexMut<Vec3<usize>> for Ten<T> {
    fn index_mut(&mut self,index: Vec3<usize>) -> &mut Self::Output {
        &mut self.data[(index.z * self.size.y + index.y) * self.size.x + index.x]
    }
}
