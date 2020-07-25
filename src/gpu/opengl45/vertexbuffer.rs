// E - OpenGL - VertexBuffer
// Desmond Germans, 2020

use crate::*;
use gl::types::{
    GLuint,
    GLvoid,
};
use std::{
    ffi::c_void,
    rc::Rc,
};

#[doc(hidden)]
pub trait GLVertex {
    fn bind() -> Vec<GLuint>;
    fn len() -> isize;
}

impl GLVertex for Vec2<f32> {
    fn bind() -> Vec<GLuint> {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
        }
        vec![0]
    }

    fn len() -> isize {
        8
    }
}

impl GLVertex for Vec4<f32> {
    fn bind() -> Vec<GLuint> {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
        }
        vec![0]
    }

    fn len() -> isize {
        16
    }
}

/// Vertex buffer GPU resource.
pub struct VertexBuffer<T: GLVertex> {
    _vertices: Vec<T>,
    vbo: GLuint,
}

impl<T: GLVertex> VertexBuffer<T> {
    /// (temporary) Create new vertex buffer.
    /// # Arguments
    /// * `gpu` - GPU context to create vertexbuffer for.
    /// * `vertices` - Vector of vertices to upload.
    pub fn new(_gpu: &Rc<gpu::GPU>,vertices: Vec<T>) -> Result<VertexBuffer<T>,SystemError> {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
            gl::BufferData(gl::ARRAY_BUFFER,T::len() * vertices.len() as isize,vertices.as_ptr() as *const c_void,gl::STATIC_DRAW);
        }
        Ok(VertexBuffer {
            _vertices: vertices,
            vbo: vbo,
        })
    }
}

impl<T: GLVertex> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.vbo);
        }
    }
}

impl gpu::GPU {
    /// (temporary) Bind current vertex buffer.
    /// # Arguments
    /// * `vertexbuffer` - Vertexbuffer to bind.
    pub fn bind_vertexbuffer<T: GLVertex>(&self,vertexbuffer: &VertexBuffer<T>) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER,vertexbuffer.vbo) };
        self.vaas.set(T::bind());
    }

    /// (temporary) Unbind current vertex buffer.
    pub fn unbind_vertexbuffer(&self) {
        let vaas = self.vaas.replace(Vec::new());
        unsafe {
            for n in &vaas {
                gl::DisableVertexAttribArray(*n);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER,0);
        }
    }
}
