// E - OpenGL - TextureCube
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    ffi::c_void,
    marker::PhantomData,
};
use gl::types::GLuint;

pub enum CubeFace {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

/// Cube texture GPU resource.
pub struct TextureCube<T: GPUDataFormat> {
    pub tex: GLuint,
    size: usize,
    phantom: PhantomData<T>,
}

impl<T: GPUDataFormat> TextureCube<T> {    
    /// (temporary) Create new cube texture.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(TextureCube)` - The new cube texture.
    /// * `Err(SystemError)` - The cube texture could not be created.
    pub fn new(_graphics: &Rc<Graphics>,size: usize) -> Result<TextureCube<T>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP,tex);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_R,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_CUBE_MAP,1,T::gl_internal_format(),size as i32,size as i32);
        };
        Ok(TextureCube {
            tex: tex,
            size: size,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new cube texture from 6 Mats.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `src_xp` - Mat for the positive-X texture.
    /// * `src_xn` - Mat for the negative-X texture.
    /// * `src_yp` - Mat for the positive-Y texture.
    /// * `src_yn` - Mat for the negative-Y texture.
    /// * `src_zp` - Mat for the positive-Z texture.
    /// * `src_zn` - Mat for the negative-Z texture.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(TextureCube)` - The new cube texture.
    /// * `Err(SystemError)` - The cube texture could not be created.
    pub fn new_from_mats(graphics: &Rc<Graphics>,src_xp: Mat<T>,src_xn: Mat<T>,src_yp: Mat<T>,src_yn: Mat<T>,src_zp: Mat<T>,src_zn: Mat<T>) -> Result<TextureCube<T>,SystemError> {
        let texture = TextureCube::new(graphics,src_xp.size.x())?;
        texture.load(CubeFace::PositiveX,Vec2::<usize>::zero(),&src_xp);
        texture.load(CubeFace::NegativeX,Vec2::<usize>::zero(),&src_xn);
        texture.load(CubeFace::PositiveY,Vec2::<usize>::zero(),&src_yp);
        texture.load(CubeFace::NegativeY,Vec2::<usize>::zero(),&src_yn);
        texture.load(CubeFace::PositiveZ,Vec2::<usize>::zero(),&src_zp);
        texture.load(CubeFace::NegativeZ,Vec2::<usize>::zero(),&src_zn);
        Ok(texture)
    }

    /// (temporary) Load data into up-facing texture.
    /// 
    /// **Arguments**
    /// 
    /// * `cf` - Cube face identifier (one of `CubeFace::*`).
    /// * `o` - offset.
    /// * `src` - Mat containing source data.
    pub fn load(&self,cf: CubeFace,o: Vec2<usize>,src: &Mat<T>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP,self.tex);
            let target = match cf {
                CubeFace::PositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
                CubeFace::NegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
                CubeFace::PositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
                CubeFace::NegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
                CubeFace::PositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
                CubeFace::NegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            };
            gl::TexSubImage2D(target,0,o.x() as i32,o.y() as i32,src.size.x() as i32,src.size.y() as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void);
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

    /// (temporary) Set texture wrapping mode along Z direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_z(&self,wrap: TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::MIRRORED_REPEAT as i32); },            
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T: GPUDataFormat> Drop for TextureCube<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
