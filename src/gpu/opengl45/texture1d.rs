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
    /// * `size` - Size of the texture.
    /// ## Returns
    /// * `Ok(Texture1D)` - The new 1D texture.
    /// * `Err(SystemError)` - The 1D texture could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: usize) -> Result<Texture1D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_1D,tex);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage1D(gl::TEXTURE_1D,1,T::gl_internal_format(),size as i32);
        };
        Ok(Texture1D {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new 1D texture from Vec.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Vec containing source data.
    /// ## Returns
    /// * `Ok(Texture1D)` - The new 1D texture.
    /// * `Err(SystemError)` - The 1D texture could not be created.
    pub fn new_from_vec(graphics: &Rc<gpu::Graphics>,src: &Vec<T>) -> Result<Texture1D<T>,SystemError> {
        let texture = Texture1D::new(graphics,src.len())?;
        texture.load(0,src);
        Ok(texture)
    }

    /// (temporary) Load data into 1D texture.
    /// ## Arguments
    /// * `o` - offset.
    /// * `src` - Vec containing source data.
    pub fn load(&self,o: usize,src: &Vec<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_1D,self.tex);
            gl::TexSubImage1D(gl::TEXTURE_1D,0,o as i32,src.len() as i32,T::gl_format(),T::gl_type(),src.as_ptr() as *const c_void);
        }

    }


    pub fn set_filter(&self,filter: gpu::TextureFilter) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match filter {
            gpu::TextureFilter::Nearest => unsafe {
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            },
            gpu::TextureFilter::Linear => unsafe {
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            }
        }
    }

    pub fn set_wrap_x(&self,wrap: gpu::TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            gpu::TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_BORDER as i32); },
            gpu::TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32); },
            gpu::TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32); },
            gpu::TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}

impl<T: gpu::GLFormat> Drop for Texture1D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
