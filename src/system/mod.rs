// E - System
// Desmond Germans, 2020

use crate::*;
use std::fmt::{Debug,Display,Formatter};

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

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;
#[cfg(target_os="linux")]
mod opengl45;
#[cfg(target_os="linux")]
pub use opengl45::*;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;
#[cfg(target_os="windows")]
mod opengl45;
#[cfg(target_os="windows")]
pub use opengl45::*;

#[cfg(target_os="macos")]
mod macos;
#[cfg(target_os="macos")]
pub use macos::*;
#[cfg(target_os="macos")]
mod opengl45;
#[cfg(target_os="macos")]
pub use opengl45::*;

#[cfg(target_os="android")]
mod android;
#[cfg(target_os="android")]
pub use android::*;
#[cfg(target_os="android")]
mod gles20;
#[cfg(target_os="android")]
pub use gles20::*;

#[cfg(target_os="ios")]
mod ios;
#[cfg(target_os="ios")]
pub use ios::*;
#[cfg(target_os="ios")]
mod gles20;
#[cfg(target_os="ios")]
pub use gles20::*;

#[cfg(target_arch="wasm32")]
mod web;
#[cfg(target_arch="wasm32")]
pub use web::*;
#[cfg(target_arch="wasm32")]
mod webgl1;
#[cfg(target_arch="wasm32")]
pub use webgl1::*;
