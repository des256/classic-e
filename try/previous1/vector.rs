// e::math vector types
// by Desmond Germans, 2019

use std::ops::*;

#[derive(Copy,Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy,Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy,Clone)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Self;
    fn add(self,other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;
    fn sub(self,other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

// a few aliases
pub type u32_2 = Vec2<u32>;
pub type i32_2 = Vec2<i32>;
pub type u64_2 = Vec2<u64>;
pub type i64_2 = Vec2<i64>;
pub type usize_2 = Vec2<usize>;
pub type isize_2 = Vec2<isize>;
pub type f32_2 = Vec2<f32>;
pub type f64_2 = Vec2<f64>;
