// E - OpenGL - Texture3D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

/// 3D texture GPU resource.
pub struct Texture3D<T: gpu::GLFormat> {
    pub tex: GLuint,
    pub size: Vec3<usize>,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> Texture3D<T> {    
    /// (temporary) Create new 3D texture.
    /// # Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `image` - Mat to upload to the GPU.
    pub fn new(_graphics: &Rc<gpu::Graphics>,image: &Ten<T>) -> Result<Texture3D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_3D,tex);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage3D(gl::TEXTURE_3D,1,T::gl_internal_format(),image.size.x as i32,image.size.y as i32,image.size.z as i32);
            gl::TexSubImage3D(gl::TEXTURE_3D,0,0,0,0,image.size.x as i32,image.size.y as i32,image.size.z as i32,T::gl_format(),T::gl_type(),image.data.as_ptr() as *const c_void);
        };
        Ok(Texture3D {
            tex: tex,
            size: image.size,
            phantom: PhantomData,
        })
    }
}

impl<T: gpu::GLFormat> Drop for Texture3D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
