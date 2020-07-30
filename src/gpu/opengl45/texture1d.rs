// E - OpenGL - Texture1D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

/// 1D texture GPU resource.
pub struct Texture1D<T: gpu::GLFormat> {
    pub tex: GLuint,
    pub size: usize,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> Texture1D<T> {    
    /// (temporary) Create new 1D texture.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `image` - Vec to upload to the GPU.
    /// ## Returns
    /// * `Ok(Texture1D)` - The new 1D texture.
    /// * `Err(SystemError)` - The 1D texture could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,image: &Vec<T>) -> Result<Texture1D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_1D,tex);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage1D(gl::TEXTURE_1D,1,T::gl_internal_format(),image.len() as i32);
            gl::TexSubImage1D(gl::TEXTURE_1D,0,0,image.len() as i32,T::gl_format(),T::gl_type(),image.as_ptr() as *const c_void);
        };
        Ok(Texture1D {
            tex: tex,
            size: image.len(),
            phantom: PhantomData,
        })
    }
}

impl<T: gpu::GLFormat> Drop for Texture1D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
