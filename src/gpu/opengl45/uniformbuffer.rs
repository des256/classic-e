// E - OpenGL - UniformBuffer
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    ffi::c_void,
    marker::PhantomData,
    ptr::null,
};
use gl::types::{
    GLuint,
};

#[doc(hidden)]
pub trait GLUniform {
    fn len() -> isize;
}

impl GLUniform for Vec2<f32> {
    fn len() -> isize {
        8
    }
}

impl GLUniform for Vec4<f32> {
    fn len() -> isize {
        16
    }
}

/// Uniform buffer GPU resource.
pub struct UniformBuffer<T: GLUniform> {
    pub(crate) ubo: GLuint,
    phantom: PhantomData<T>,
}

impl<T: GLUniform> UniformBuffer<T> {
    /// (temporary) Create new uniform buffer.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create uniform buffer for.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(UniformBuffer)` - The new uniform buffer.
    /// * `Err(SystemError)` - The uniform buffer could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>) -> Result<UniformBuffer<T>,SystemError> {
        let mut ubo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1,&mut ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER,ubo);
            gl::BufferData(gl::UNIFORM_BUFFER,1,null() as *const c_void,gl::DYNAMIC_DRAW);
        }
        Ok(UniformBuffer {
            ubo: ubo,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new uniform buffer from Vec.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create uniform buffer for.
    /// * `src` - Vec containing source data.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(UniformBuffer)` - The new uniform buffer.
    /// * `Err(SystemError)` - The uniform buffer could not be created.
    pub fn new_from_vec(_graphics: &Rc<gpu::Graphics>,src: Vec<T>) -> Result<UniformBuffer<T>,SystemError> {
        let uniformbuffer = UniformBuffer::new(_graphics)?;
        uniformbuffer.load(0,&src);
        Ok(uniformbuffer)
    }

    /// (temporary) Load data into uniform buffer
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Vec containing source data.
    pub fn load(&self,o: usize,src: &Vec<T>) {
        unsafe { 
            gl::BindBuffer(gl::UNIFORM_BUFFER,self.ubo);
            gl::BufferData(gl::UNIFORM_BUFFER,T::len() * (o + src.len()) as isize,null() as *const c_void,gl::STATIC_DRAW);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                T::len() * (o as isize),
                T::len() * src.len() as isize,
                src.as_ptr() as *const c_void
            );
        }
    }
}

impl<T: GLUniform> Drop for UniformBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.ubo);
        }
    }
}
