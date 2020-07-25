// E - GPU (OpenGL 4.5)
// Desmond Germans, 2020

use crate::*;
use gl::types::{
    GLuint,
    GLenum,
};

#[doc(hidden)]
pub trait OpenGLFormat: Clone + Copy + Zero {
    fn gl_internal_format() -> GLuint;
    fn gl_format() -> GLuint;
    fn gl_type() -> GLenum;
}

impl OpenGLFormat for u8 {
    fn gl_internal_format() -> GLuint { gl::R8UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for i8 {
    fn gl_internal_format() -> GLuint { gl::R8I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl OpenGLFormat for u16 {
    fn gl_internal_format() -> GLuint { gl::R16UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl OpenGLFormat for i16 {
    fn gl_internal_format() -> GLuint { gl::R16I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl OpenGLFormat for u32 {
    fn gl_internal_format() -> GLuint { gl::R32UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl OpenGLFormat for i32 {
    fn gl_internal_format() -> GLuint { gl::R32I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl OpenGLFormat for f32 {
    fn gl_internal_format() -> GLuint { gl::R32F as GLuint }
    fn gl_format() -> GLenum { gl::RED }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl OpenGLFormat for Vec2<u8> {
    fn gl_internal_format() -> GLuint { gl::RG8UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for Vec2<i8> {
    fn gl_internal_format() -> GLuint { gl::RG8I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl OpenGLFormat for Vec2<u16> {
    fn gl_internal_format() -> GLuint { gl::RG16UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl OpenGLFormat for Vec2<i16> {
    fn gl_internal_format() -> GLuint { gl::RG16I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl OpenGLFormat for Vec2<u32> {
    fn gl_internal_format() -> GLuint { gl::RG32UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl OpenGLFormat for Vec2<i32> {
    fn gl_internal_format() -> GLuint { gl::RG32I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl OpenGLFormat for Vec2<f32> {
    fn gl_internal_format() -> GLuint { gl::RG32F as GLuint }
    fn gl_format() -> GLenum { gl::RG }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl OpenGLFormat for Vec3<u8> {
    fn gl_internal_format() -> GLuint { gl::RGB8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for Vec3<i8> {
    fn gl_internal_format() -> GLuint { gl::RGB8I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl OpenGLFormat for Vec3<u16> {
    fn gl_internal_format() -> GLuint { gl::RGB16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl OpenGLFormat for Vec3<i16> {
    fn gl_internal_format() -> GLuint { gl::RGB16I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl OpenGLFormat for Vec3<u32> {
    fn gl_internal_format() -> GLuint { gl::RGB32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl OpenGLFormat for Vec3<i32> {
    fn gl_internal_format() -> GLuint { gl::RGB32I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl OpenGLFormat for Vec3<f32> {
    fn gl_internal_format() -> GLuint { gl::RGB32F as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl OpenGLFormat for Vec4<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for Vec4<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl OpenGLFormat for Vec4<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl OpenGLFormat for Vec4<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl OpenGLFormat for Vec4<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl OpenGLFormat for Vec4<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl OpenGLFormat for Vec4<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl OpenGLFormat for pixel::R3G3B2 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE_3_3_2 }
}

impl OpenGLFormat for pixel::R5G6B5 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_5_6_5_REV }
}

impl OpenGLFormat for pixel::ARGB4 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_4_4_4_4_REV }
}

impl OpenGLFormat for pixel::A1RGB5 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_1_5_5_5_REV }
}

impl OpenGLFormat for pixel::RGB8 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for pixel::ARGB8 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT_8_8_8_8_REV }
}

mod gpu;
pub use gpu::*;

mod shader;
pub use shader::*;

mod texture2d;
pub use texture2d::*;

mod framebuffer;
pub use framebuffer::*;

mod vertexbuffer;
pub use vertexbuffer::*;
