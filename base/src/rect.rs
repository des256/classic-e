// E - Rect
// Desmond Germans, 2020

use crate::*;

#[derive(Copy,Clone,Debug)]
pub struct Rect<T: Simdable>(Simd4<T>);

impl<T: Simdable> Rect<T> {
    pub fn new(ox: T,oy: T,sx: T,sy: T) -> Rect<T> {
        Rect(Simd4::new([ox,oy,sx,sy]))
    }

    pub fn new_os(o: Vec2<T>,s: Vec2<T>) -> Rect<T> {
        Rect(Simd4::new([o.x(),o.y(),s.x(),s.y()]))
    }

    pub fn ox(&self) -> T {
        self.0.get(0)
    }

    pub fn oy(&self) -> T {
        self.0.get(1)
    }

    pub fn sx(&self) -> T {
        self.0.get(2)
    }

    pub fn sy(&self) -> T {
        self.0.get(3)
    }

    pub fn o(&self) -> Vec2<T> {
        vec2!(self.0.get(0),self.0.get(1))
    }

    pub fn s(&self) -> Vec2<T> {
        vec2!(self.0.get(2),self.0.get(3))
    }

    pub fn set_ox(&mut self,ox: T) {
        self.0.set(0,ox);
    }

    pub fn set_oy(&mut self,oy: T) {
        self.0.set(1,oy);
    }

    pub fn set_sx(&mut self,sx: T) {
        self.0.set(2,sx);
    }

    pub fn set_sy(&mut self,sy: T) {
        self.0.set(3,sy);
    }

    pub fn set_o(&mut self,o: Vec2<T>) {
        self.0.set(0,o.x());
        self.0.set(1,o.y());
    }

    pub fn set_s(&mut self,s: Vec2<T>) {
        self.0.set(2,s.x());
        self.0.set(3,s.y());
    }

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

#[macro_export]
macro_rules! rect {
    ($ox:expr,$oy:expr,$sx:expr,$sy:expr) => { Rect::new($ox,$oy,$sx,$sy) };
    ($o:expr,$s:expr) => { Rect::new_os($o,$s) };
}
