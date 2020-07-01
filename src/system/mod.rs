// E - System
// Desmond Germans, 2020

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::Vec2;
use crate::Rect;

pub enum Button {
    Left,
    Middle,
    Right,
}

impl Display for Button {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Button::Left => { write!(f,"left") },
            Button::Middle => { write!(f,"middle") },
            Button::Right => { write!(f,"right") },
        }
    }
}

impl Debug for Button {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        match self {
            Button::Left => { write!(f,"left") },
            Button::Middle => { write!(f,"middle") },
            Button::Right => { write!(f,"right") },
        }
    }
}

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

pub enum Event<'a> {
    Paint(&'a Graphics,Rect<isize>),
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(Vec2<isize>,Button),
    MouseRelease(Vec2<isize>,Button),
    MouseWheel(Wheel),
    MouseMove(Vec2<isize>),
    Resize(Vec2<isize>),
    Close,
}

pub enum UIError {
    Generic,
}

impl Debug for UIError {
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
