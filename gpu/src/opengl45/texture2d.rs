// E - OpenGL - Texture2D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
};
use gl::types::GLuint;

/// 2D texture GPU resource.
pub struct Texture2D<T: GPUTextureFormat> {
    pub tex: GLuint,
    size: Vec2<usize>,
    phantom: PhantomData<T>,
}

impl Graphics {
    /// (temporary) Create new empty 2D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture2D)` - The new 2D texture.
    /// * `Err(SystemError)` - The 2D texture could not be created.
    pub fn create_texture2d<T: GPUTextureFormat>(&self,size: Vec2<usize>) -> Result<Texture2D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,T::gl_internal_format(),size.x() as i32,size.y() as i32);
        };
        Ok(Texture2D {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new 2D texture from Mat.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Mat containing source data.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture2D)` - The new 2D texture.
    /// * `Err(SystemError)` - The 2D texture could not be created.
    pub fn create_texture2d_from_mat<T: GPUTextureFormat>(&self,src: Mat<T>) -> Result<Texture2D<T>,SystemError> {
        let texture = self.create_texture2d(src.size)?;
        texture.load(Vec2::<usize>::zero(),&src);
        Ok(texture)
    }
}

impl<T: GPUTextureFormat> Texture2D<T> {
    /// (temporary) Load data into 2D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Mat containing source data.
    pub fn load(&self,o: Vec2<usize>,src: &Mat<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
            gl::TexSubImage2D(gl::TEXTURE_2D,0,o.x() as i32,o.y() as i32,src.size.x() as i32,src.size.y() as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void) };
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

    /// (temporary) Set texture wrapping mode along Y direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_y(&self,wrap: TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::MIRRORED_REPEAT as i32); },            
        }
    }

    pub fn size(&self) -> Vec2<usize> {
        self.size
    }
}

impl<T: GPUTextureFormat> Drop for Texture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
