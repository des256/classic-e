// E - platform
// Desmond Germans, 2020

use base::*;
use std::fmt;

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

// OS-specific code
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

#[cfg(target_arch="wasm32")]
mod web;
#[cfg(target_arch="wasm32")]
pub use web::*;
