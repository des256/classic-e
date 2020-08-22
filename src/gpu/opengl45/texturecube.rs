// E - OpenGL - TextureCube
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
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
pub struct TextureCube<T: gpu::GLFormat> {
    pub tex: GLuint,
    pub size: usize,
    phantom: PhantomData<T>,
}

impl<T: gpu::GLFormat> TextureCube<T> {    
    /// (temporary) Create new cube texture.
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `size` - Size of the texture.
    /// ## Returns
    /// * `Ok(TextureCube)` - The new cube texture.
    /// * `Err(SystemError)` - The cube texture could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: usize) -> Result<TextureCube<T>,SystemError> {
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
    /// ## Arguments
    /// * `graphics` - Graphics context to create texture for.
    /// * `src_xp` - Mat for the positive-X texture.
    /// * `src_xn` - Mat for the negative-X texture.
    /// * `src_yp` - Mat for the positive-Y texture.
    /// * `src_yn` - Mat for the negative-Y texture.
    /// * `src_zp` - Mat for the positive-Z texture.
    /// * `src_zn` - Mat for the negative-Z texture.
    /// ## Returns
    /// * `Ok(TextureCube)` - The new cube texture.
    /// * `Err(SystemError)` - The cube texture could not be created.
    pub fn new_from_mats(graphics: &Rc<gpu::Graphics>,src_xp: &Mat<T>,src_xn: &Mat<T>,src_yp: &Mat<T>,src_yn: &Mat<T>,src_zp: &Mat<T>,src_zn: &Mat<T>) -> Result<TextureCube<T>,SystemError> {
        let texture = TextureCube::new(graphics,src_xp.size.x)?;
        texture.load(CubeFace::PositiveX,vec2!(0,0),src_xp);
        texture.load(CubeFace::NegativeX,vec2!(0,0),src_xn);
        texture.load(CubeFace::PositiveY,vec2!(0,0),src_yp);
        texture.load(CubeFace::NegativeY,vec2!(0,0),src_yn);
        texture.load(CubeFace::PositiveZ,vec2!(0,0),src_zp);
        texture.load(CubeFace::NegativeZ,vec2!(0,0),src_zn);
        Ok(texture)
    }

    /// (temporary) Load data into up-facing texture.
    /// ## Arguments
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
            gl::TexSubImage2D(target,0,o.x as i32,o.y as i32,src.size.x as i32,src.size.y as i32,T::gl_format(),T::gl_type(),src.data.as_ptr() as *const c_void);
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

impl<T: gpu::GLFormat> Drop for TextureCube<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
