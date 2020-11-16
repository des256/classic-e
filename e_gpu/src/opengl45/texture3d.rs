// E - OpenGL - Texture3D
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
};
use gl::types::GLuint;

#[doc(hidden)]
pub struct CoreTexture3D<T: GPUTextureFormat> {
    _graphics: Rc<Graphics>,
    pub tex: GLuint,
    pub size: Vec3<usize>,
    phantom: PhantomData<T>,
}

impl<T: GPUTextureFormat> Drop for CoreTexture3D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

/// 3D texture GPU resource.
pub struct Texture3D<T: GPUTextureFormat> {
    pub(crate) core: Rc<CoreTexture3D<T>>,
    pub c: Cuboid<usize>,
}

impl<T: GPUTextureFormat> Texture3D<T> {    
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
    pub fn new(graphics: &Rc<Graphics>,size: Vec3<usize>) -> Result<Rc<Texture3D<T>>,SystemError> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_3D,tex);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage3D(gl::TEXTURE_3D,1,T::gl_internal_format(),size.x as i32,size.y as i32,size.z as i32);
        }
        let core = Rc::new(CoreTexture3D {
            _graphics: Rc::clone(&graphics),
            tex: tex,
            size: size,
            phantom: PhantomData,
        });
        Ok(Rc::new(Texture3D {
            core: core,
            c: cuboid!(0,0,0,size.x,size.y,size.z),
        }))
    }

    /// (temporary) Create 3D subtexture from other 3D texture.
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
    /// The new 3D subtexture.
    pub fn new_sub(source: &Rc<Texture3D<T>>,c: Cuboid<usize>) -> Result<Rc<Texture3D<T>>,SystemError> {
        if (c.o.x > source.c.s.x) || (c.o.y > source.c.s.y) || (c.o.z > source.c.s.z) {
            return Err(SystemError::Generic);
        }
        let o = source.c.o + c.o;
        let mut s = c.s;
        if o.x + s.x > source.c.s.x {
            s.x = source.c.s.x - o.x;
        }
        if o.y + s.y > source.c.s.y {
            s.y = source.c.s.y - o.y;
        }
        if o.z + s.z > source.c.s.z {
            s.z = source.c.s.z - o.z;
        }
        Ok(Rc::new(Texture3D {
            core: Rc::clone(&source.core),
            c: cuboid!(o,s),
        }))
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
    pub fn new_from_ten(graphics: &Rc<Graphics>,src: Ten<T>) -> Result<Rc<Texture3D<T>>,SystemError> {
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
        if (o.x >= self.c.s.x) || (o.y >= self.c.s.y) || (o.z >= self.c.s.z) {
            return;
        }
        let o = self.c.o + o;
        let mut s = src.size;
        if o.x + s.x > self.c.s.x {
            s.x = self.c.s.x - o.x;
        }
        if o.y + s.y > self.c.s.y {
            s.y = self.c.s.y - o.y;
        }
        if o.z + s.z > self.c.s.z {
            s.z = self.c.s.z - o.z;
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D,self.core.tex);
            gl::TexSubImage3D(gl::TEXTURE_3D,0,o.x as i32,o.y as i32,o.z as i32,s.x as i32,s.y as i32,s.z as i32,T::gl_format(),T::gl_type(),src.data().as_ptr() as *const c_void);
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

    /// (temporary) Set texture wrapping mode along Z direction.
    /// 
    /// **Arguments**
    /// 
    /// * `wrap` - New wrapping mode (one of `TextureWrap::*`).
    pub fn set_wrap_z(&self,wrap: TextureWrap) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.core.tex); }
        match wrap {
            TextureWrap::Black => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_BORDER as i32); },
            TextureWrap::Edge => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::CLAMP_TO_EDGE as i32); },
            TextureWrap::Repeat => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::REPEAT as i32); },
            TextureWrap::Mirror => unsafe { gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_R,gl::MIRRORED_REPEAT as i32); },            
        }
    }
}
