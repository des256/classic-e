// E
// Desmond Germans, 2020

//! # E
//!
//! It's E. E for everything.

use std::fmt::{Display,Formatter,Debug};

mod zeroone;
pub use zeroone::*;

mod mat;
pub use mat::*;

#[macro_use]
mod vector;
pub use vector::*;

mod matrix;
pub use matrix::*;

#[macro_use]
mod rect;
pub use rect::*;

#[derive(Copy,Clone)]
pub enum Mouse {
    Left,
    Middle,
    Right,
}

impl Display for Mouse {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Mouse::Left => { write!(f,"left") },
            Mouse::Middle => { write!(f,"middle") },
            Mouse::Right => { write!(f,"right") },
        }
    }
}

impl Debug for Mouse {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Mouse::Left => { write!(f,"left") },
            Mouse::Middle => { write!(f,"middle") },
            Mouse::Right => { write!(f,"right") },
        }
    }
}

#[derive(Copy,Clone)]
pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Wheel {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

impl Debug for Wheel {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

#[derive(Copy,Clone)]
pub enum Event {
    Paint(Rect<isize>),
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(Vec2<isize>,Mouse),
    MouseRelease(Vec2<isize>,Mouse),
    MouseWheel(Wheel),
    MouseMove(Vec2<isize>),
    Resize(Vec2<isize>),
    Close,
}

pub enum SystemError {
    Generic,
}

impl Debug for SystemError {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        write!(f,"generic error")
    }
}

mod system;
pub use system::*;

mod window;
pub use window::*;

pub mod pixel;

pub mod image;

pub mod gpu;

pub mod ui;
