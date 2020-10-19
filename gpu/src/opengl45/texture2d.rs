// E - OpenGL - Texture2D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

pub struct CoreTexture2D<T: GPUTextureFormat> {
    _graphics: Rc<Graphics>,
    pub tex: GLuint,
    pub size: Vec2<usize>,
    phantom: PhantomData<T>,
}

impl<T: GPUTextureFormat> Drop for CoreTexture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

/// 2D texture GPU resource.
pub struct Texture2D<T: GPUTextureFormat> {
    pub(crate) core: Rc<CoreTexture2D<T>>,
    pub r: Rect<usize>,
}

impl<T: GPUTextureFormat> Texture2D<T> {
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
    pub fn new(graphics: &Rc<Graphics>,size: Vec2<usize>) -> Result<Rc<Texture2D<T>>,SystemError> {
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
        let core = Rc::new(CoreTexture2D {
            _graphics: Rc::clone(&graphics),
            tex: tex,
            size: size,
            phantom: PhantomData,
        });
        Ok(Rc::new(Texture2D {
            core: core,
            r: rect!(0,0,size.x,size.y),
        }))
    }

    /// (temporary) Create 2D subtexture from other 2D texture.
    ///
    /// The underlying texture resource, including filtering is shared.
    ///
    /// It is possible to create a subtexture of another subtexture. The
    /// rectangle is always relative to the source.
    ///
    /// **Arguments**
    ///
    /// *source* - Source texture.
    /// *r* - Source rectangle.
    ///
    /// **Returns**
    ///
    /// The new 2D subtexture.
    pub fn new_sub(source: &Rc<Texture2D<T>>,r: Rect<usize>) -> Result<Rc<Texture2D<T>>,SystemError> {
        if (r.o.x > source.r.s.x) || (r.o.y > source.r.s.y) {
            return Err(SystemError::Generic);
        }
        let o = source.r.o + r.o;
        let mut s = r.s;
        if o.x + s.x > source.r.s.x {
            s.x = source.r.s.x - o.x;
        }
        if o.y + s.y > source.r.s.y {
            s.y = source.r.s.y - o.y;
        }
        Ok(Rc::new(Texture2D {
            core: Rc::clone(&source.core),
            r: rect!(o,s),
        }))
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
    pub fn new_from_mat(graphics: &Rc<Graphics>,src: Mat<T>) -> Result<Rc<Texture2D<T>>,SystemError> {
        let texture = Texture2D::new(graphics,src.size)?;
        texture.load(Vec2::<usize>::zero(),&src);
        Ok(texture)
    }

    /// (temporary) Load data into 2D texture.
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Mat containing source data.
    pub fn load(&self,o: Vec2<usize>,src: &Mat<T>) {
        if (o.x >= self.r.s.x) || (o.y >= self.r.s.y) {
            return;
        }
        let o = self.r.o + o;
        let mut s = src.size;
        if o.x + s.x > self.r.s.x {
            s.x = self.r.s.x - o.x;
        }
        if o.y + s.y > self.r.s.y {
            s.y = self.r.s.y - o.y;
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.core.tex);
            gl::TexSubImage2D(gl::TEXTURE_2D,0,o.x as i32,o.y as i32,s.x as i32,s.y as i32,T::gl_format(),T::gl_type(),src.data().as_ptr() as *const c_void);
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

    /// (temporary) Set texture wrapping mode along Y direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_y(&self,wrap: TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.core.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}
