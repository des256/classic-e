// E - OpenGL - VertexBuffer
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    marker::PhantomData,
    ptr::null,
};
use gl::types::GLuint;

/// Vertex buffer GPU resource.
pub struct VertexBuffer<T: GPUVertexFormat> {
    pub(crate) vao: GLuint,
    pub(crate) vbo: GLuint,
    phantom: PhantomData<T>,
}

impl Graphics {
    /// (temporary) Create new vertex buffer.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create vertexbuffer for.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(VertexBuffer)` - The new vertex buffer.
    /// * `Err(SystemError)` - The vertex buffer could not be created.
    pub fn create_vertexbuffer<T: GPUVertexFormat>(&self) -> Result<VertexBuffer<T>,SystemError> {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            gl::BufferData(gl::ARRAY_BUFFER,1,null() as *const c_void,gl::STATIC_DRAW);
            T::bind();
        }
        Ok(VertexBuffer {
            vao: vao,
            vbo: vbo,
            phantom: PhantomData,
        })
    }

    /// (temporary) Create new vertex buffer from Vec.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create texture for.
    /// * `src` - Vec containing source data.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(VertexBuffer)` - The new vertex buffer.
    /// * `Err(SystemError)` - The vertex buffer could not be created.
    pub fn create_vertexbuffer_from_vec<T: GPUVertexFormat>(&self,src: Vec<T>) -> Result<VertexBuffer<T>,SystemError> {
        let vertexbuffer = self.create_vertexbuffer()?;
        vertexbuffer.load(0,&src);
        Ok(vertexbuffer)
    }
}

impl<T: GPUVertexFormat> VertexBuffer<T> {
    /// (temporary) Load data into vertex buffer
    /// 
    /// **Arguments**
    /// 
    /// * `o` - offset.
    /// * `src` - Vec containing source data.
    pub fn load(&self,o: usize,src: &Vec<T>) {
        unsafe { 
            gl::BindVertexArray(self.vao);
            gl::BufferData(gl::ARRAY_BUFFER,((o + src.len()) * T::len()) as isize,null() as *const c_void,gl::STATIC_DRAW);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (T::len() * o) as isize,
                (T::len() * src.len()) as isize,
                src.as_ptr() as *const c_void
            );
        }
    }
}

impl<T: GPUVertexFormat> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.vbo);
            gl::DeleteVertexArrays(1,&self.vao);
        }
    }
}