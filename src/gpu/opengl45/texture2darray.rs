// E - OpenGL - Texture2DArray
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

/// 2D texture array GPU resource.
pub struct Texture2DArray<T: gpu::GLFormat> {  // start with one layer, rebuild later
    pub tex: GLuint,
    pub size: Vec3<usize>,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> Texture2DArray<T> {
    /// (temporary) Create new empty 2D texture array.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// ## Returns
    /// * `Ok(Texture2DArray)` - The new 2D texture.
    /// * `Err(SystemError)` - The 2D texture array could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: Vec3<usize>) -> Result<Texture2DArray<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY,tex);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage3D(gl::TEXTURE_2D_ARRAY,1,T::gl_internal_format(),size.x as i32,size.y as i32,size.z as i32);
        };
        Ok(Texture2DArray {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new 2D texture array from Ten.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture array for.
    /// * `src` - Ten containing source data.
    /// ## Returns
    /// * `Ok(Texture2DArray)` - The new 2D texture array.
    /// * `Err(SystemError)` - The 2D texture array could not be created.
    pub fn new_from_ten(graphics: &Rc<gpu::Graphics>,src: Ten<T>) -> Result<Texture2DArray<T>,SystemError> {
        let texture = Texture2DArray::new(graphics,src.size)?;
        texture.load(vec3!(0,0,0),&src);
        Ok(texture)
    }

    /// (temporary) Load data into 2D texture array.
    /// ## Arguments
    /// * `o` - offset.
    /// * `src` - Ten containing source data.
    pub fn load(&self,o: Vec3<usize>,src: &Ten<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY,self.tex);
            gl::TexSubImage3D(gl::TEXTURE_2D_ARRAY,0,o.x as i32,o.y as i32,o.z as i32,src.size.x as i32,src.size.y as i32,src.size.z as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void);
        }
    }

    /// (temporary) Load data into layer of 2D texture array.
    /// ## Arguments
    /// * `layer` - layer.
    /// * `o` - offset.
    /// * `src` - Mat containing source data.
    pub fn load_mat(&self,layer: usize,o: Vec2<usize>,src: &Mat<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY,self.tex);
            gl::TexSubImage3D(gl::TEXTURE_2D_ARRAY,0,o.x as i32,o.y as i32,layer as i32,src.size.x as i32,src.size.y as i32,1,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void);
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

impl<T: gpu::GLFormat> Drop for Texture2DArray<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
