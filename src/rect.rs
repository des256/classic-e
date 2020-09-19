// E - Rect
// Desmond Germans, 2020

use crate::*;

macro_rules! impl_rect (
    ($r:ident,$v:ty,$h:ty,$t:ty) => (
        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone)]
        pub struct $r {
            pub o: $h,
            pub s: $h,
        }

        impl $r {
            pub fn from_os(o: $h,s: $h) -> $r {
                $r {
                    o: o,
                    s: s,
                }
            }

            pub fn contains(&self,p: &$h) -> bool {
                (*p.x() >= *self.o.x()) &&
                (*p.y() >= *self.o.y()) &&
                (*p.x() < *self.o.x() + *self.s.x()) &&
                (*p.y() < *self.o.y() + *self.s.y())
            }
        }

        impl Zero for $r {
            fn zero() -> $r {
                $r {
                    o: <$h>::zero(),
                    s: <$h>::zero(),
                }
            }
        }
    );
);

impl_rect!(u8r,u8x4,u8x2,u8);
impl_rect!(i8r,i8x4,i8x2,i8);
impl_rect!(u16r,u16x4,u16x2,u16);
impl_rect!(i16r,i16x4,i16x2,i16);
impl_rect!(u32r,u32x4,u32x2,u32);
impl_rect!(i32r,i32x4,i32x2,i32);
impl_rect!(u64r,u64x4,u64x2,u64);
impl_rect!(i64r,i64x4,i64x2,i64);
impl_rect!(usizer,usizex4,usizex2,usize);
impl_rect!(isizer,isizex4,isizex2,isize);
impl_rect!(f32r,f32x4,f32x2,f32);
impl_rect!(f64r,f64x4,f64x2,f64);
