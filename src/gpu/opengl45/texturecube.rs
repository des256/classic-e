// E - OpenGL - TextureCube
// Desmond Germans, 2020

use crate::*;
use std::{
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

/// Cube texture GPU resource.
pub struct TextureCube<T: gpu::GLFormat> {
    pub tex: GLuint,
    pub size: usize,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> TextureCube<T> {    
    /// (temporary) Create new cube texture.
    /// # Arguments
    /// * `graphics` - Graphics context to create texture for.
    pub fn new(_graphics: &Rc<gpu::Graphics>) -> Result<TextureCube<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP,tex);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_R,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_CUBE_MAP,1,T::gl_internal_format(),64,64);
        };
        Ok(TextureCube {
            tex: tex,
            size: 64,
            phantom: PhantomData,
        })
    }
}

impl<T: gpu::GLFormat> Drop for TextureCube<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
