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
    /// (temporary) Create new empty 2D texture.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// ## Returns
    /// * `Ok(Texture2D)` - The new 2D texture.
    /// * `Err(SystemError)` - The 2D texture could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: Vec2<usize>) -> Result<Texture2D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,T::gl_internal_format(),size.x as i32,size.y as i32);
        };
        Ok(Texture2D {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new 2D texture from Mat.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Mat containing source data.
    /// ## Returns
    /// * `Ok(Texture2D)` - The new 2D texture.
    /// * `Err(SystemError)` - The 2D texture could not be created.
    pub fn new_from_mat(graphics: &Rc<gpu::Graphics>,src: &Mat<T>) -> Result<Texture2D<T>,SystemError> {
        let texture = Texture2D::new(graphics,src.size)?;
        texture.load(vec2!(0,0),src);
        Ok(texture)
    }

    /// (temporary) Load data into 2D texture.
    /// ## Arguments
    /// * `o` - offset.
    /// * `src` - Mat containing source data.
    pub fn load(&self,o: Vec2<usize>,src: &Mat<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
            gl::TexSubImage2D(gl::TEXTURE_2D,0,o.x as i32,o.y as i32,src.size.x as i32,src.size.y as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void) };
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

    pub fn set_wrap_y(&self,wrap: gpu::TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            gpu::TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_BORDER as i32); },
            gpu::TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32); },
            gpu::TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32); },
            gpu::TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}

impl<T: gpu::GLFormat> Drop for Texture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
