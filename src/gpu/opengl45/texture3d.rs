// E - OpenGL - Texture3D
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    ffi::c_void,
    marker::PhantomData,
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
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture3D)` - The new 3D texture.
    /// * `Err(SystemError)` - The 3D texture could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: Vec3<usize>) -> Result<Texture3D<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_3D,tex);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage3D(gl::TEXTURE_3D,1,T::gl_internal_format(),size.x() as i32,size.y() as i32,size.z() as i32);
        }
        Ok(Texture3D {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new 3D texture from Ten.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Ten containing source data.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Texture3D)` - The new 3D texture.
    /// * `Err(SystemError)` - The 3D texture could not be created.
    pub fn new_from_ten(graphics: &Rc<gpu::Graphics>,src: Ten<T>) -> Result<Texture3D<T>,SystemError> {
        let texture = Texture3D::new(graphics,src.size)?;
        texture.load(Vec3::<usize>::zero(),&src);
        Ok(texture)
    }

    /// (temporary) Load data into 3D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Ten containing source data.
    pub fn load(&self,o: Vec3<usize>,src: &Ten<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D,self.tex);
            gl::TexSubImage3D(gl::TEXTURE_3D,0,o.x() as i32,o.y() as i32,o.z() as i32,src.size.x() as i32,src.size.y() as i32,src.size.z() as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void);
        }
    }

    /// (temporary) Set texture filter mode.
    /// 
    /// **Arguments**
    /// 
    /// * `filter` - New filter mode (one of `TextureFilter::*`).
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

    /// (temporary) Set texture wrapping mode along X direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_x(&self,wrap: gpu::TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            gpu::TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_BORDER as i32); },
            gpu::TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32); },
            gpu::TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32); },
            gpu::TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::MIRRORED_REPEAT as i32); },            
        }
    }

    /// (temporary) Set texture wrapping mode along Y direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_y(&self,wrap: gpu::TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            gpu::TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_BORDER as i32); },
            gpu::TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32); },
            gpu::TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32); },
            gpu::TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::MIRRORED_REPEAT as i32); },            
        }
    }

    /// (temporary) Set texture wrapping mode along Z direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_z(&self,wrap: gpu::TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            gpu::TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_BORDER as i32); },
            gpu::TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_EDGE as i32); },
            gpu::TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32); },
            gpu::TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}

impl<T: gpu::GLFormat> Drop for Texture3D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
