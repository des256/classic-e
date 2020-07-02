// E - Mat
// Desmond Germans, 2020

use crate::Zero;
use crate::Vec2;
use std::marker::PhantomData;

pub struct Mat<T: Clone + Copy + Zero> {
    pub size: Vec2<usize>,
    pub data: Box<[T]>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Copy + Zero> Mat<T> {
    pub fn new(size: Vec2<usize>) -> Mat<T> {
        Mat {
            size: size,
            data: vec![T::zero(); (size.x * size.y) as usize].into_boxed_slice(),
            phantom: PhantomData,
        }
    }

    pub fn set(&mut self,p: Vec2<usize>,v: T) {
        self.data[p.y * self.size.x + p.x] = v;
    }

    pub fn get(&self,p: Vec2<usize>) -> T {
        self.data[p.y * self.size.x + p.x]
    }
}
