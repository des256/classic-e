// e::canvas::Rect
// by Desmond Germans, 2019

use math::*;

#[derive(Copy,Clone)]
pub struct Rect {
    pub orig: usize_2,
    pub size: usize_2,
}

impl Rect {
    pub fn new(x0: usize,y0: usize,width: usize,height: usize) -> Rect {
        Rect {
            orig: usize_2 { x: x0,y: y0 },
            size: usize_2 { x: width,y: height },
        }
    }
}
