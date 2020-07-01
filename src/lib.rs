// E
// Desmond Germans, 2020

mod zeroone;
pub use zeroone::*;

mod pixel;
pub use pixel::*;

#[macro_use]
mod vector;
pub use vector::*;

mod matrix;
pub use matrix::*;

#[macro_use]
mod rect;
pub use rect::*;

mod system;
pub use system::*;

mod image;
pub use image::*;

mod ui;
pub use ui::*;

pub mod prelude {
    pub fn init_vec2<T>(x: T,y: T) -> crate::Vec2<T> {
        crate::Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn init_vec3<T>(x: T,y: T,z: T) -> crate::Vec3<T> {
        crate::Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    
    pub fn init_vec4<T>(x: T,y: T,z: T,w: T) -> crate::Vec4<T> {
        crate::Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn init_rect<T>(o: crate::Vec2<T>,s: crate::Vec2<T>) -> crate::Rect<T> {
        crate::Rect {
            o: o,
            s: s,
        }
    }
}
