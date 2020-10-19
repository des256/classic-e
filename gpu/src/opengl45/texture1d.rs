// E - OpenGL - Texture1D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

pub struct CoreTexture1D<T: GPUTextureFormat> {
    _graphics: Rc<Graphics>,
    pub tex: GLuint,
    pub size: usize,
    phantom: PhantomData<T>,
}

impl<T: GPUTextureFormat> Drop for CoreTexture1D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

/// 1D texture GPU resource.
pub struct Texture1D<T: GPUTextureFormat> {
    pub(crate) core: Rc<CoreTexture1D<T>>,
    pub e: Extent<usize>,
}

impl<T: GPUTextureFormat> Texture1D<T> {
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
    pub fn new(graphics: &Rc<Graphics>,size: usize) -> Result<Rc<Texture1D<T>>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_1D,tex);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_1D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage1D(gl::TEXTURE_1D,1,T::gl_internal_format(),size as i32);
        };
        let core = Rc::new(CoreTexture1D {
            _graphics: Rc::clone(&graphics),
            tex: tex,
            size: size,
            phantom: PhantomData,
        });
        Ok(Rc::new(Texture1D {
            core: core,
            e: extent!(0,size),
        }))
    }

    /// (temporary) Create 1D subtexture from other 1D texture.
    ///
    /// The underlying texture resource, including filtering is shared.
    ///
    /// It is possible to create a subtexture of another subtexture. The
    /// range is always relative to the source.
    ///
    /// **Arguments**
    ///
    /// *source* - Source texture.
    /// *range* - Source range.
    ///
    /// **Returns**
    ///
    /// The new 1D subtexture.
    pub fn new_sub(source: &Rc<Texture1D<T>>,e: Extent<usize>) -> Result<Rc<Texture1D<T>>,SystemError> {
        if e.o > source.e.s {
            return Err(SystemError::Generic);
        }
        let o = source.e.o + e.o;
        let mut s = e.s;
        if o + s > source.e.s {
            s = source.e.s - o;
        }
        Ok(Rc::new(Texture1D {
            core: Rc::clone(&source.core),
            e: extent!(o,s),
        }))
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
    pub fn new_from_vec(graphics: &Rc<Graphics>,src: Vec<T>) -> Result<Rc<Texture1D<T>>,SystemError> {
        let texture = Texture1D::new(graphics,src.len())?;
        texture.load(0,&src);
        Ok(texture)
    }

    /// (temporary) Load data into 1D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Vec containing source data.
    pub fn load(&self,o: usize,src: &Vec<T>) {
        if o > self.e.s {
            return;
        }
        let o = self.e.o + o;
        let mut s = src.len();
        if o + s > self.e.s {
            s = self.e.s - o;
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_1D,self.core.tex);
            gl::TexSubImage1D(gl::TEXTURE_1D,0,o as i32,s as i32,T::gl_format(),T::gl_type(),src.as_ptr() as *const c_void);
        }

    }

    /// (temporary) Set texture filter mode.
    /// 
    /// **Arguments**
    /// 
    /// * `filter` - New filter mode (one of `TextureFilter::*`).
    pub fn set_filter(&self,filter: TextureFilter) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.core.tex); }
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
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.core.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}
