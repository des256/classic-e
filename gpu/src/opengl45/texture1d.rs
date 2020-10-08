// E - OpenGL - Texture1D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
};
use gl::types::GLuint;

/// 1D texture GPU resource.
pub struct Texture1D<T: GPUTextureFormat> {
    pub tex: GLuint,
    size: usize,
    phantom: PhantomData<T>,
}

impl Graphics {
    /// (temporary) Create new empty 1D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture1D)` - The new 1D texture.
    /// * `Err(SystemError)` - The 1D texture could not be created.
    pub fn create_texture1d<T: GPUTextureFormat>(&self,size: usize) -> Result<Texture1D<T>,SystemError> {
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
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Vec containing source data.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture1D)` - The new 1D texture.
    /// * `Err(SystemError)` - The 1D texture could not be created.
    pub fn create_texture1d_from_vec<T: GPUTextureFormat>(&self,src: Vec<T>) -> Result<Texture1D<T>,SystemError> {
        let texture = self.create_texture1d(src.len())?;
        texture.load(0,&src);
        Ok(texture)
    }
}

impl<T: GPUTextureFormat> Texture1D<T> {
    /// (temporary) Load data into 1D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Vec containing source data.
    pub fn load(&self,o: usize,src: &Vec<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_1D,self.tex);
            gl::TexSubImage1D(gl::TEXTURE_1D,0,o as i32,src.len() as i32,T::gl_format(),T::gl_type(),src.as_ptr() as *const c_void);
        }

    }

    /// (temporary) Set texture filter mode.
    /// 
    /// **Arguments**
    /// 
    /// * `filter` - New filter mode (one of `TextureFilter::*`).
    pub fn set_filter(&self,filter: TextureFilter) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match filter {
            TextureFilter::Nearest => unsafe {
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            },
            TextureFilter::Linear => unsafe {
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            }
        }
    }

    /// (temporary) Set texture wrapping mode along X direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_x(&self,wrap: TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::MIRRORED_REPEAT as i32); },            
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T: GPUTextureFormat> Drop for Texture1D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
