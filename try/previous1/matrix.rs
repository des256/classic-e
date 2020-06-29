// e::math matrix types
// by Desmond Germans, 2019

use std::ops::*;

pub struct Mat2x2<T> {
    x: Vec2<T>,
    y: Vec2<T>,
}

impl<T: Add<Output = T>> Add for Mat2x2<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Mat2x2<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// a few aliases
pub type f32_2x2 = Mat2x2<f32>;
pub type f64_2x2 = Mat2x2<f64>;
