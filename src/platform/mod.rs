// E - platforms
// Desmond Germans, 2020

use crate::*;

/// Mouse button.
#[derive(Copy,Clone,Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Mouse wheel direction.
#[derive(Copy,Clone,Debug)]
pub enum MouseWheel {
    Up,
    Down,
    Left,
    Right,
}

/// User interaction event.
#[derive(Copy,Clone,Debug)]
pub enum Event {
    /// Key was pressed.
    KeyPress(u8),
    /// Key was released.
    KeyRelease(u8),
    /// Mouse button was pressed.
    MousePress(i32x2,MouseButton),
    /// Mouse button was released.
    MouseRelease(i32x2,MouseButton),
    /// Mouse wheel was moved.
    MouseWheel(MouseWheel),
    /// Mouse was moved.
    MouseMove(i32x2),
    /// Window was resized.
    Size(i32x2),
    /// Window was moved.
    Move(i32x2),
    /// The window requires redrawing.
    Render,
    /// Window close button was pressed.
    Close,
}

/// System error result.
#[derive(Copy,Clone,Debug)]
pub enum SystemError {
    /// (temporary) Generic error.
    Generic,
}

/// Window trait.
pub trait Window {
    fn handle(&self,event: Event);
    fn rect(&self) -> i32r;
    fn set_rect(&self,r: i32r);
    fn id(&self) -> u64;
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
