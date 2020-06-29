// e event
// by Desmond Germans, 2019

use crate::*;

pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(isize_2,u8),
    MouseRelease(isize_2,u8),
    MouseMove(isize_2),
    MouseWheel(isize_2),
    Geometry(isize_r),
    Close,
}
