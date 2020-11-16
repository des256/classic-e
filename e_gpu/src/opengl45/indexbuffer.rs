// E - OpenGL - IndexBuffer
// Desmond Germans, 2020

use crate::*;
use std::{
    ffi::c_void,
    ptr::null,
    rc::Rc,
    marker::PhantomData,
};
use gl::types::{
    GLuint,
    GLenum,
};

/// Format trait for index buffers.
pub trait GPUIndexFormat {
    fn len() -> isize;
    fn gl_type() -> GLenum;
}

impl GPUIndexFormat for u16 {
    fn len() -> isize { 2 }
    fn gl_type() -> GLenum { gl::UNSIGNED_SHORT }
}

impl GPUIndexFormat for u32 {
    fn len() -> isize { 4 }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

/// Index buffer GPU resource.
pub struct IndexBuffer<T: GPUIndexFormat> {
    _graphics: Rc<Graphics>,
    pub(crate) ibo: GLuint,
    phantom: PhantomData<T>,
}

impl<T: GPUIndexFormat> IndexBuffer<T> {
    /// (temporary) Create new index buffer.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create indexbuffer for.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(IndexBuffer)` - The new index buffer.
    /// * `Err(SystemError)` - The index buffer could not be created.
    pub fn new(graphics: &Rc<Graphics>) -> Result<Rc<IndexBuffer<T>>,SystemError> {
        let mut ibo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1,&mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,0,null() as *const c_void,gl::DYNAMIC_DRAW);
        }
        Ok(Rc::new(IndexBuffer {
            _graphics: Rc::clone(&graphics),
            ibo: ibo,
            phantom: PhantomData,
        }))
    }

    /// (temporary) Create new index buffer from vec.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create indexbuffer for.
    /// * `indices` - Vector of indices to upload.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(IndexBuffer)` - The new index buffer.
    /// * `Err(SystemError)` - The index buffer could not be created.
    pub fn new_from_vec(graphics: &Rc<Graphics>,indices: &Vec<T>) -> Result<Rc<IndexBuffer<T>>,SystemError> {
        let indexbuffer = IndexBuffer::<T>::new(graphics)?;
        indexbuffer.load(indices);
        Ok(indexbuffer)
    }

    /// (temporary) Load all or part of the indices from vec.
    /// 
    /// **Arguments**
    /// 
    /// * `indices` - The replacement indices vec.
    pub fn load(&self,indices: &Vec<T>) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,T::len() * indices.len() as isize,indices.as_ptr() as *const c_void,gl::DYNAMIC_DRAW);
        }
    }
}

impl<T: GPUIndexFormat> Drop for IndexBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.ibo);
        }
    }
}
