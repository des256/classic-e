// E - GPU
// Desmond Germans, 1998-2021

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

#[cfg(feature="gpu_vulkan")]
mod vulkan;
#[cfg(feature="gpu_vulkan")]
pub use vulkan::*;

#[cfg(feature="gpu_opengl45")]
mod opengl45;
#[cfg(feature="gpu_opengl45")]
pub use opengl45::*;

#[cfg(feature="gpu_directx12")]
mod directx12;
#[cfg(feature="gpu_directx12")]
pub use directx12::*;

#[cfg(feature="gpu_gles20")]
mod gles20;
#[cfg(feature="gpu_gles20")]
pub use gles20::*;

#[cfg(feature="gpu_metal")]
mod metal;
#[cfg(feature="gpu_metal")]
pub use metal::*;

#[cfg(feature="gpu_webgl")]
mod webgl;
#[cfg(feature="gpu_webgl")]
pub use webgl::*;

#[cfg(feature="gpu_webgpu")]
mod webgpu;
#[cfg(feature="gpu_webgpu")]
pub use webgpu::*;
