// E - platforms
// Desmond Germans, 2020

use crate::*;

/// Mouse button.
#[derive(Copy,Clone)]
pub enum Mouse {
    Left,
    Middle,
    Right,
}

impl std::fmt::Display for Mouse {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mouse::Left => { write!(f,"left") },
            Mouse::Middle => { write!(f,"middle") },
            Mouse::Right => { write!(f,"right") },
        }
    }
}

impl std::fmt::Debug for Mouse {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mouse::Left => { write!(f,"left") },
            Mouse::Middle => { write!(f,"middle") },
            Mouse::Right => { write!(f,"right") },
        }
    }
}

/// Mouse wheel direction.
#[derive(Copy,Clone)]
pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Wheel {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

impl std::fmt::Debug for Wheel {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

/// User interaction event.
#[derive(Copy,Clone)]
pub enum Event {
    /// Need to paint rectangle in the window.
    Paint(Rect<isize>),
    /// Key was pressed.
    KeyPress(u8),
    /// Key was released.
    KeyRelease(u8),
    /// Mouse button was pressed.
    MousePress(Vec2<isize>,Mouse),
    /// Mouse button was released.
    MouseRelease(Vec2<isize>,Mouse),
    /// Mouse wheel was moved.
    MouseWheel(Wheel),
    /// Mouse was moved.
    MouseMove(Vec2<isize>),
    /// Window was resized.
    Resize(Vec2<isize>),
    /// Window close button was pressed.
    Close,
}

/// System error result.
pub enum SystemError {
    /// (temporary) Generic error.
    Generic,
}

impl std::fmt::Debug for SystemError {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"generic error")
    }
}

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;

#[cfg(target_os="macos")]
mod macos;
#[cfg(target_os="macos")]
pub use macos::*;

#[cfg(target_os="android")]
mod android;
#[cfg(target_os="android")]
pub use android::*;

#[cfg(target_os="ios")]
mod ios;
#[cfg(target_os="ios")]
pub use ios::*;
