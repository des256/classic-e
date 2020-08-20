// E - GPU
// Desmond Germans, 2020

//! GPU access.

/// (temporary) Blending mode.
#[derive(Clone,Copy)]
pub enum BlendMode {
    /// Replace target with source.
    Replace,
    /// Blend target with source, using source alpha.
    Over,
}

#[cfg(target_os="linux")]
mod opengl45;
#[cfg(target_os="linux")]
pub use opengl45::*;

#[cfg(target_os="windows")]
mod opengl45;
#[cfg(target_os="windows")]
pub use opengl45::*;

#[cfg(target_os="macos")]
mod metal;
#[cfg(target_os="macos")]
pub use metal::*;

#[cfg(target_os="android")]
mod gles20;
#[cfg(target_os="android")]
pub use gles20::*;

#[cfg(target_os="ios")]
mod gles20;
#[cfg(target_os="ios")]
pub use gles20::*;

#[cfg(target_arch="wasm32")]
mod webgl1;
#[cfg(target_arch="wasm32")]
pub use webgl1::*;
