// E - GPU
// Desmond Germans, 2020

//! GPU access.

// NOTE: Migrating to Vulkan as default GPU layer.

use e_base::*;
use e_system::*;

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
mod vulkan;
// mod opengl45;
// mod gles20;
#[cfg(target_os="linux")]
pub use vulkan::*;
// pub use opengl45::*;
// pub use gles20::*;

#[cfg(target_os="windows")]
mod vulkan;
// mod opengl45;
// mod gles20;
// mod directx12;
#[cfg(target_os="windows")]
pub use vulkan::*;
// pub use opengl45::*;
// pub use gles20::*;
// pub use directx12::*;

#[cfg(target_os="macos")]
mod metal;
// mod opengl45::*;
#[cfg(target_os="macos")]
pub use metal::*;
// pub use opengl45::*;

#[cfg(target_os="android")]
mod vulkan;
// mod gles20;
#[cfg(target_os="android")]
pub use vulkan::*;
// pub use gles20::*;

#[cfg(target_os="ios")]
mod vulkan;
// mod gles20;
#[cfg(target_os="ios")]
pub use vulkan::*;
// pub use gles20::*;

#[cfg(target_arch="wasm32")]
mod webgl;
// mod webgpu;
#[cfg(target_arch="wasm32")]
pub use webgl::*;
// pub use webgpu::*;
