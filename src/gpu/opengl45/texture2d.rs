// E - OpenGL - Texture2D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

/// 2D texture GPU resource.
pub struct Texture2D<T: gpu::GLFormat> {
    pub tex: GLuint,
    pub size: Vec2<usize>,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> Texture2D<T> {    
    /// (temporary) Create new 2D texture.
    /// # Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `image` - Mat to upload to the GPU.
    pub fn new(_gpu: &Rc<gpu::Graphics>,image: &Mat<T>) -> Result<Texture2D<T>,SystemError> {
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
}

impl<T: gpu::GLFormat> Drop for Texture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
