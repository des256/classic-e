// E - platforms
// Desmond Germans, 2020

use base::*;

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

/// PlatformWindow trait.
pub trait HandleEvent {

    /// Handle incoming event.
    fn handle(&self,event: Event);

    /// Return unique window ID.
    fn id(&self) -> u64;
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
