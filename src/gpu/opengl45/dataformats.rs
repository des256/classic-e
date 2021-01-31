// E - GPU (OpenGL 4.5) - Data formats
// Desmond Germans, 2020

use crate::*;
use gl::types::{
    GLuint,
    GLenum,
    GLvoid,
};

/// Format trait for shader uniform access.
pub trait GPUUniformFormat: Clone + Copy {
    fn len() -> usize;
    fn set(location: i32,value: Self);
}

impl GPUUniformFormat for u32 {
    fn len() -> usize { 4 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform1ui(location,value) }; }
}

impl GPUUniformFormat for i32 {
    fn len() -> usize { 4 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform1i(location,value) }; }
}

impl GPUUniformFormat for f32 {
    fn len() -> usize { 4 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform1f(location,value) }; }
}

impl GPUUniformFormat for Vec2<u32> {
    fn len() -> usize { 8 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform2ui(location,value.x,value.y) }; }
}

impl GPUUniformFormat for Vec2<i32> {
    fn len() -> usize { 8 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform2i(location,value.x,value.y) }; }
}

impl GPUUniformFormat for Vec2<f32> {
    fn len() -> usize { 8 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform2f(location,value.x,value.y) }; }
}

impl GPUUniformFormat for Vec3<u32> {
    fn len() -> usize { 12 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform3ui(location,value.x,value.y,value.z) }; }
}

impl GPUUniformFormat for Vec3<i32> {
    fn len() -> usize { 12 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform3i(location,value.x,value.y,value.z) }; }
}

impl GPUUniformFormat for Vec3<f32> {
    fn len() -> usize { 12 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform3f(location,value.x,value.y,value.z) }; }
}

impl GPUUniformFormat for Vec3A<u32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4ui(location,value.x,value.y,value.z,0) }; }
}

impl GPUUniformFormat for Vec3A<i32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4i(location,value.x,value.y,value.z,0) }; }
}

impl GPUUniformFormat for Vec3A<f32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4f(location,value.x,value.y,value.z,0.0) }; }
}

impl GPUUniformFormat for Vec4<u32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4ui(location,value.x,value.y,value.z,value.w) }; }
}

impl GPUUniformFormat for Vec4<i32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4i(location,value.x,value.y,value.z,value.w) }; }
}

impl GPUUniformFormat for Vec4<f32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4f(location,value.x,value.y,value.z,value.w) }; }
}

impl GPUUniformFormat for Rect<u32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4ui(location,value.o.x,value.o.y,value.s.x,value.s.y) }; }
}

impl GPUUniformFormat for Rect<i32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4i(location,value.o.x,value.o.y,value.s.x,value.s.y) }; }
}

impl GPUUniformFormat for Rect<f32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4f(location,value.o.x,value.o.y,value.s.x,value.s.y) }; }
}

impl GPUUniformFormat for Quat<f32> {
    fn len() -> usize { 16 }
    fn set(location: i32,value: Self) { unsafe { gl::Uniform4f(location,value.r,value.i,value.j,value.k) }; }
}

/// Format trait for vertex buffer specification.
pub trait GPUVertexFormat {
    fn len() -> usize;
    fn bind();
}

impl GPUVertexFormat for u32 {
    fn len() -> usize { 4 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,1,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for i32 {
    fn len() -> usize { 4 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,1,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for f32 {
    fn len() -> usize { 4 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,1,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec2<u32> {
    fn len() -> usize { 8 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,2,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec2<i32> {
    fn len() -> usize { 8 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,2,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec2<f32> {
    fn len() -> usize { 8 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3<u32> {
    fn len() -> usize { 12 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,3,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3<i32> {
    fn len() -> usize { 12 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,3,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3<f32> {
    fn len() -> usize { 12 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3A<u32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3A<i32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec3A<f32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec4<u32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec4<i32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Vec4<f32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Rect<u32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::UNSIGNED_INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Rect<i32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribIPointer(0,4,gl::INT,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Rect<f32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

impl GPUVertexFormat for Quat<f32> {
    fn len() -> usize { 16 }
    fn bind() { unsafe { gl::EnableVertexAttribArray(0); gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid); } }
}

/// Format trait for texture specification.
pub trait GPUTextureFormat: Clone + Copy + Zero {
    fn gl_internal_format() -> GLuint;
    fn gl_format() -> GLuint;
    fn gl_type() -> GLenum;
}

impl GPUTextureFormat for u8 {
    fn gl_internal_format() -> GLuint { gl::R8UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for i8 {
    fn gl_internal_format() -> GLuint { gl::R8I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for u16 {
    fn gl_internal_format() -> GLuint { gl::R16UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for i16 {
    fn gl_internal_format() -> GLuint { gl::R16I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for u32 {
    fn gl_internal_format() -> GLuint { gl::R32UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for i32 {
    fn gl_internal_format() -> GLuint { gl::R32I as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for f32 {
    fn gl_internal_format() -> GLuint { gl::R32F as GLuint }
    fn gl_format() -> GLenum { gl::RED }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Vec2<u8> {
    fn gl_internal_format() -> GLuint { gl::RG8UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for Vec2<i8> {
    fn gl_internal_format() -> GLuint { gl::RG8I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for Vec2<u16> {
    fn gl_internal_format() -> GLuint { gl::RG16UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for Vec2<i16> {
    fn gl_internal_format() -> GLuint { gl::RG16I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for Vec2<u32> {
    fn gl_internal_format() -> GLuint { gl::RG32UI as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for Vec2<i32> {
    fn gl_internal_format() -> GLuint { gl::RG32I as GLuint }
    fn gl_format() -> GLenum { gl::RG_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for Vec2<f32> {
    fn gl_internal_format() -> GLuint { gl::RG32F as GLuint }
    fn gl_format() -> GLenum { gl::RG }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Vec3<u8> {
    fn gl_internal_format() -> GLuint { gl::RGB8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for Vec3<i8> {
    fn gl_internal_format() -> GLuint { gl::RGB8I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for Vec3<u16> {
    fn gl_internal_format() -> GLuint { gl::RGB16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for Vec3<i16> {
    fn gl_internal_format() -> GLuint { gl::RGB16I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for Vec3<u32> {
    fn gl_internal_format() -> GLuint { gl::RGB32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for Vec3<i32> {
    fn gl_internal_format() -> GLuint { gl::RGB32I as GLuint }
    fn gl_format() -> GLenum { gl::RGB_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for Vec3<f32> {
    fn gl_internal_format() -> GLuint { gl::RGB32F as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Vec3A<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for Vec3A<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for Vec3A<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for Vec3A<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for Vec3A<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for Vec3A<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for Vec3A<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Vec4<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for Vec4<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for Vec4<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for Vec4<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for Vec4<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for Vec4<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for Vec4<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Rect<u8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for Rect<i8> {
    fn gl_internal_format() -> GLuint { gl::RGBA8I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::BYTE }
}

impl GPUTextureFormat for Rect<u16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUTextureFormat for Rect<i16> {
    fn gl_internal_format() -> GLuint { gl::RGBA16I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::SHORT }
}

impl GPUTextureFormat for Rect<u32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32UI as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

impl GPUTextureFormat for Rect<i32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32I as GLuint }
    fn gl_format() -> GLenum { gl::RGBA_INTEGER }
    fn gl_type() -> GLenum { gl::INT }
}

impl GPUTextureFormat for Rect<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for Quat<f32> {
    fn gl_internal_format() -> GLuint { gl::RGBA32F as GLuint }
    fn gl_format() -> GLenum { gl::RGBA }
    fn gl_type() -> GLenum { gl::FLOAT }
}

impl GPUTextureFormat for pixel::R8 {
    fn gl_internal_format() -> GLuint { gl::R8 as GLuint }
    fn gl_format() -> GLenum { gl::RED }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for pixel::R3G3B2 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::RGB }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE_3_3_2 }
}

impl GPUTextureFormat for pixel::R5G6B5 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_5_6_5_REV }
}

impl GPUTextureFormat for pixel::ARGB4 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_4_4_4_4_REV }
}

impl GPUTextureFormat for pixel::A1RGB5 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT_1_5_5_5_REV }
}

impl GPUTextureFormat for pixel::RGB8 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl GPUTextureFormat for pixel::ARGB8 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT_8_8_8_8_REV }
}

/// Textured rectangle as uniform.
#[derive(Copy,Clone)]
#[repr(C)]
pub struct TexRect {
    pub r: Rect<f32>,  // rectangle: x, y, w, h
    pub t: Rect<f32>,  // texture coordinates: x, y, w, h
}

impl GPUUniformFormat for TexRect {
    fn len() -> usize { 32 }
    fn set(_location: i32,_value: Self) { panic!("cannot set TexRect as uniform"); }
}
