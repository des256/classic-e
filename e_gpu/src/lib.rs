// E - GPU
// Desmond Germans, 2020

//! GPU access.

use e_base::*;
use e_platform::*;

/// Texture filtering modes.
#[derive(Clone,Copy)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

/// Texture wrapping modes.
#[derive(Clone,Copy)]
pub enum TextureWrap {
    Black,
    Edge,
    Repeat,
    Mirror,
}

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
// mod gles20;
// mod vulkan;
#[cfg(target_os="linux")]
pub use opengl45::*;
// pub use gles20::*;
// pub use vulkan::*;

#[cfg(target_os="windows")]
mod opengl45;
// mod gles20;
// mod directx12;
// mod vulkan;
#[cfg(target_os="windows")]
pub use opengl45::*;
// pub use gles20::*;
// pub use directx12::*;
// pub use vulkan::*;

#[cfg(target_os="macos")]
mod metal;
// mod opengl45::*;
#[cfg(target_os="macos")]
pub use metal::*;
// pub use opengl45::*;

#[cfg(target_os="android")]
mod gles20;
// mod vulkan;
#[cfg(target_os="android")]
pub use gles20::*;
// pub use vulkan::*;

#[cfg(target_os="ios")]
mod gles20;
// mod vulkan;
#[cfg(target_os="ios")]
pub use gles20::*;
// pub use vulkan::*;

#[cfg(target_arch="wasm32")]
mod webgl;
// mod webgpu;
#[cfg(target_arch="wasm32")]
pub use webgl::*;
// pub use webgpu::*;
