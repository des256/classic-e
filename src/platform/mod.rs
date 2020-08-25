// E - platforms
// Desmond Germans, 2020

use crate::*;

/// Mouse button.
#[derive(Copy,Clone,Debug)]
pub enum Mouse {
    Left,
    Middle,
    Right,
}

/// Mouse wheel direction.
#[derive(Copy,Clone,Debug)]
pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

/// User interaction event.
#[derive(Copy,Clone,Debug)]
pub enum Event {
    /// The window requires redrawing.
    Render,
    /// Key was pressed.
    KeyPress(u8),
    /// Key was released.
    KeyRelease(u8),
    /// Mouse button was pressed.
    MousePress(Vec2<i32>,Mouse),
    /// Mouse button was released.
    MouseRelease(Vec2<i32>,Mouse),
    /// Mouse wheel was moved.
    MouseWheel(Wheel),
    /// Mouse was moved.
    MouseMove(Vec2<i32>),
    /// Window was resized.
    Resize(Vec2<i32>),
    /// Window close button was pressed.
    Close,
}

/// System error result.
#[derive(Copy,Clone,Debug)]
pub enum SystemError {
    /// (temporary) Generic error.
    Generic,
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
