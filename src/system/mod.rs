// E - System
// Desmond Germans, 1998-2021

use {
    crate::*,
    std::{
        fmt,
    },
};

/// Mouse button.
#[derive(Copy,Clone,Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl fmt::Display for MouseButton {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MouseButton::Left => { write!(f,"Left") },
            MouseButton::Middle => { write!(f,"Middle") },
            MouseButton::Right => { write!(f,"Right") },
        }
    }
}

/// Mouse wheel direction.
#[derive(Copy,Clone,Debug)]
pub enum MouseWheel {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for MouseWheel {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MouseWheel::Up => { write!(f,"Up") },
            MouseWheel::Down => { write!(f,"Down") },
            MouseWheel::Left => { write!(f,"Left") },
            MouseWheel::Right => { write!(f,"Right") },
        }
    }
}

/// Mouse cursor.
pub enum MouseCursor {
    Arrow,
    VArrow,
    Hourglass,
    Crosshair,
    Finger,
    OpenHand,
    GrabbingHand,
    MagnifyingGlass,
    Caret,
    SlashedCircle,
    SizeNSEW,
    SizeNESW,
    SizeNWSE,
    SizeWE,
    SizeNS,
}

/// User interaction event.
#[derive(Copy,Clone,Debug)]
pub enum Event {
    /// Key was pressed.
    KeyPress(u8),
    /// Key was released.
    KeyRelease(u8),
    /// Mouse button was pressed.
    MousePress(Vec2<i32>,MouseButton),
    /// Mouse button was released.
    MouseRelease(Vec2<i32>,MouseButton),
    /// Mouse wheel was moved.
    MouseWheel(MouseWheel),
    /// Mouse was moved.
    MouseMove(Vec2<i32>),
    /// Window was moved or resized.
    Configure(Rect<i32>),
    /// The window requires redrawing.
    Render,
    /// Window close button was pressed.
    Close,
}

impl fmt::Display for Event {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::KeyPress(c) => { write!(f,"KeyPress({})",c) },
            Event::KeyRelease(c) => { write!(f,"KeyRelease({})",c) },
            Event::MousePress(p,b) => { write!(f,"MousePress({},{})",p,b) },
            Event::MouseRelease(p,b) => { write!(f,"MouseRelease({},{})",p,b) },
            Event::MouseWheel(w) => { write!(f,"MouseWheel({})",w) },
            Event::MouseMove(p) => { write!(f,"MouseMove({})",p) },
            Event::Configure(r) => { write!(f,"Configure({})",r) },
            Event::Render => { write!(f,"Render") },
            Event::Close => { write!(f,"Close") },
        }
    }
}

#[cfg(feature="system_linux")]
mod linux;
#[cfg(feature="system_linux")]
pub use linux::*;

#[cfg(feature="system_windows")]
mod windows;
#[cfg(feature="system_windows")]
pub use windows::*;

#[cfg(feature="system_macos")]
mod macos;
#[cfg(feature="system_macos")]
pub use macos::*;

#[cfg(feature="system_android")]
mod android;
#[cfg(feature="system_android")]
pub use android::*;

#[cfg(feature="system_ios")]
mod ios;
#[cfg(feature="system_ios")]
pub use ios::*;

#[cfg(feature="system_web")]
mod web;
#[cfg(feature="system_web")]
pub use web::*;
