// E - OpenGL - Texture2D
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::Image;
use std::ffi::c_void;
use std::marker::PhantomData;
use gl::types::GLenum;
use crate::RGB8;
use crate::ARGB8;
use crate::Graphics;
use crate::UIError;
use crate::Vec2;

pub trait OpenGLFormat {
    fn gl_internal_format() -> GLuint;
    fn gl_format() -> GLuint;
    fn gl_type() -> GLenum;
}

impl OpenGLFormat for RGB8 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for ARGB8 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT_8_8_8_8_REV }
}

pub struct Texture2D<T: OpenGLFormat> {
    pub tex: GLuint,
    pub size: Vec2<usize>,
    phantom: PhantomData<T>,
}

impl<T: OpenGLFormat> Drop for Texture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

impl Graphics {
    pub fn create_texture2d<T: OpenGLFormat>(&self,image: &Image<T>) -> Result<Texture2D<T>,UIError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,T::gl_internal_format(),image.size.x as i32,image.size.y as i32);
            gl::TexSubImage2D(gl::TEXTURE_2D,0,0,0,image.size.x as i32,image.size.y as i32,T::gl_format(),T::gl_type(),image.data.as_ptr() as *const c_void);
        };
        Ok(Texture2D {
            tex: tex,
            size: image.size,
            phantom: PhantomData,
        })
    }

    pub fn bind_texture2d<T: OpenGLFormat>(&self,layer: usize,texture: &Texture2D<T>) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + layer as u32);
            gl::BindTexture(gl::TEXTURE_2D,texture.tex);
        }
    }

    pub fn unbind_texture2d(&self,layer: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + layer as u32);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
    }
}
