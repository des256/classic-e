// e::audio
// by Desmond Germans, 2019

mod vector;
pub use vector::*;

// Linux
#[cfg(target_os="linux")]
mod linux_xcb_glx_opengl45;
//mod linux_xcb_gles2;
//mod linux_xcb_vulkan;
#[cfg(target_os="linux")]
pub use linux_xcb_glx_opengl45::*;
//pub use linux_xcb_gles2::*;
//pub use linux_xcb_vulkan::*;
#[cfg(target_os="linux")]
mod opengl45;
//mod gles2;
//mod vulkan;
#[cfg(target_os="linux")]
pub use opengl45::*;
//pub use gles2::*
//pub use vulkan::*;
#[cfg(target_os="linux")]
mod linux_alsa;
#[cfg(target_os="linux")]
pub use linux_alsa::*;

// Windows
//#[cfg(target_os="win32")]
//mod windows_opengl45;
//mod windows_gles2;
//mod windows_vulkan;
//mod windows_directx12;
//#[cfg(target_os="win32")]
//pub mod windows_opengl45::*;
//pub mod windows_gles2::*;
//pub mod windows_vulkan::*;
//pub mod windows_directx12::*;
//#[cfg(target_os="win32")]
//mod opengl45;
//mod gles2;
//mod vulkan;
//mod directx12;
//#[cfg(target_os="win32")]
//pub mod opengl45::*;
//pub mod gles2::*;
//pub mod vulkan::*;
//pub mod directx12::*;
//#[cfg(target_os="win32")]
//mod windows_directsound;
//#[cfg(target_os="win32")]
//pub use windows_directsound::*;

// Android
//#[cfg(target_os="android")]
//mod android_gles2;
//#[cfg(target_os="android")]
//pub mod android_gles2;
//#[cfg(target_os="android")]
//mod gles2;
//#[cfg(target_os="android")]
//pub mod gles2::*;
//#[cfg(target_os="android")]
//mod android;
//#[cfg(target_os="android")]
//pub use android::*;

// Web
//#[cfg(target_arch="wasm32")]
//mod web_webgl;
//#[cfg(target_arch="wasm32")]
//pub mod web_webgl;
//#[cfg(target_arch="wasm32")]
//mod webgl;
//#[cfg(target_arch="wasm32")]
//pub mod webgl;
//#[cfg(target_arch="wasm32")]
//mod web_webaudio;
//#[cfg(target_arch="wasn32")]
//pub use web_webaudio::*;

mod image;
pub use image::*;

mod rect;
pub use rect::*;

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod tga;
pub mod gif;
pub mod pbm;
pub mod tiff;
pub mod xbm;

mod application;
pub use application::*;
