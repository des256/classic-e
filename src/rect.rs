// E - Rect
// Desmond Germans, 2020

use crate::*;

#[derive(Copy,Clone,Debug)]
pub struct Rect<T: Simd2 + Simd4>(pub <T as Simd4>::Type);

macro_rules! impl_rect {
    ($t:ty) => {
        impl Rect<$t> {
            pub fn new(ox: $t,oy: $t,sx: $t,sy: $t) -> Rect<$t> {
                Rect(<$t as Simd4>::Type::new(ox,oy,sx,sy))
            }

            pub fn new_os(o: Vec2<$t>,s: Vec2<$t>) -> Rect<$t> {
                Rect(<$t as Simd4>::Type::new(o.x(),o.y(),s.x(),s.y()))
            }

            pub fn ox(&self) -> $t {
                self.0.get(0)
            }

            pub fn oy(&self) -> $t {
                self.0.get(1)
            }

            pub fn sx(&self) -> $t {
                self.0.get(2)
            }

            pub fn sy(&self) -> $t {
                self.0.get(3)
            }

            pub fn o(&self) -> Vec2<$t> {
                Vec2(<$t as Simd2>::Type::new(self.0.get(0),self.0.get(1)))
            }
        
            pub fn s(&self) -> Vec2<$t> {
                Vec2(<$t as Simd2>::Type::new(self.0.get(2),self.0.get(3)))
            }

            pub fn set_ox(&mut self,ox: $t) {
                self.0.set(0,ox);
            }

            pub fn set_oy(&mut self,oy: $t) {
                self.0.set(1,oy);
            }

            pub fn set_sx(&mut self,sx: $t) {
                self.0.set(2,sx);
            }

            pub fn set_sy(&mut self,sy: $t) {
                self.0.set(3,sy);
            }

            pub fn set_o(&mut self,o: Vec2<$t>) {
                self.0.set(0,o.x());
                self.0.set(1,o.y());
            }

            pub fn set_s(&mut self,s: Vec2<$t>) {
                self.0.set(2,s.x());
                self.0.set(3,s.y());
            }

            pub fn contains(&self,p: &Vec2<$t>) -> bool {
                (p.x() >= self.ox()) &&
                (p.y() >= self.oy()) &&
                (p.x() < self.ox() + self.sx()) &&
                (p.y() < self.oy() + self.sy())
            }
        }

        impl Zero for Rect<$t> {
            fn zero() -> Self {
                Rect(<$t as Simd4>::Type::zero())
            }
        }
    }
}

impl_rect!(u8);
impl_rect!(i8);
impl_rect!(u16);
impl_rect!(i16);
impl_rect!(u32);
impl_rect!(i32);
impl_rect!(u64);
impl_rect!(i64);
impl_rect!(f32);
impl_rect!(f64);
impl_rect!(usize);
impl_rect!(isize);

#[macro_export]
macro_rules! rect {
    ($t:ty: $ox:expr,$oy:expr,$sx:expr,$sy:expr) => { Rect::<$t>::new($ox,$oy,$sx,$sy) };
    ($t:ty: $o:expr,$s:expr) => { Rect::<$t>::new_os($o,$s) };
}
