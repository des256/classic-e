// E - GPU (OpenGL 4.5)
// Desmond Germans, 2020

use crate::*;
use gl::types::{
    GLuint,
    GLenum,
};

#[doc(hidden)]
pub trait GPUDataFormat: Clone + Copy + Zero {
    fn gl_internal_format() -> GLuint;
    fn gl_format() -> GLuint;
    fn gl_type() -> GLenum;
}

impl GPUDataFormat for u8 {
    fn gl_internal_format() -> GLuint { gl::R8UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for i8 {
    fn gl_internal_format() -> GLuint { gl::R8I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUDataFormat for u16 {
    fn gl_internal_format() -> GLuint { gl::R16UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUDataFormat for i16 {
    fn gl_internal_format() -> GLuint { gl::R16I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUDataFormat for u32 {
    fn gl_internal_format() -> GLuint { gl::R32UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUDataFormat for i32 {
    fn gl_internal_format() -> GLuint { gl::R32I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUDataFormat for f32 {
    fn gl_internal_format() -> GLuint { gl::R32F as GLuint }
    fn gl_format() -> GLenum { gl::RED }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUDataFormat for Vec2<u8> {
    fn gl_internal_format() -> GLuint { gl::RG8UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for Vec2<i8> {
    fn gl_internal_format() -> GLuint { gl::RG8I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUDataFormat for Vec2<u16> {
    fn gl_internal_format() -> GLuint { gl::RG16UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUDataFormat for Vec2<i16> {
    fn gl_internal_format() -> GLuint { gl::RG16I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUDataFormat for Vec2<u32> {
    fn gl_internal_format() -> GLuint { gl::RG32UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUDataFormat for Vec2<i32> {
    fn gl_internal_format() -> GLuint { gl::RG32I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUDataFormat for Vec2<f32> {
    fn gl_internal_format() -> GLuint { gl::RG32F as GLuint }
    fn gl_format() -> GLenum { gl::RG }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUDataFormat for Vec3<u8> {
    fn gl_internal_format() -> GLuint { gl::RGB8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for Vec3<i8> {
    fn gl_internal_format() -> GLuint { gl::RGB8I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUDataFormat for Vec3<u16> {
    fn gl_internal_format() -> GLuint { gl::RGB16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUDataFormat for Vec3<i16> {
    fn gl_internal_format() -> GLuint { gl::RGB16I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUDataFormat for Vec3<u32> {
    fn gl_internal_format() -> GLuint { gl::RGB32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUDataFormat for Vec3<i32> {
    fn gl_internal_format() -> GLuint { gl::RGB32I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUDataFormat for Vec3<f32> {
    fn gl_internal_format() -> GLuint { gl::RGB32F as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUDataFormat for Vec3A<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for Vec3A<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUDataFormat for Vec3A<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUDataFormat for Vec3A<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUDataFormat for Vec3A<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUDataFormat for Vec3A<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUDataFormat for Vec3A<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUDataFormat for Vec4<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for Vec4<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUDataFormat for Vec4<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUDataFormat for Vec4<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUDataFormat for Vec4<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUDataFormat for Vec4<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUDataFormat for Vec4<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUDataFormat for pixel::R8 {
    fn gl_internal_format() -> GLuint { gl::R8 as GLuint }
    fn gl_format() -> GLenum { gl::RED }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for pixel::R3G3B2 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE_3_3_2 }
}

impl GPUDataFormat for pixel::R5G6B5 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_5_6_5_REV }
}

impl GPUDataFormat for pixel::ARGB4 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_4_4_4_4_REV }
}

impl GPUDataFormat for pixel::A1RGB5 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_1_5_5_5_REV }
}

impl GPUDataFormat for pixel::RGB8 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUDataFormat for pixel::ARGB8 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT_8_8_8_8_REV }
}

mod graphics;
pub use graphics::*;

mod compute;
pub use compute::*;

mod shader;
pub use shader::*;

mod texture1d;
pub use texture1d::*;

mod texture2d;
pub use texture2d::*;

mod texture2darray;
pub use texture2darray::*;

mod texture3d;
pub use texture3d::*;

mod texturecube;
pub use texturecube::*;

mod framebuffer;
pub use framebuffer::*;

mod vertexbuffer;
pub use vertexbuffer::*;

mod indexbuffer;
pub use indexbuffer::*;

mod uniformbuffer;
pub use uniformbuffer::*;