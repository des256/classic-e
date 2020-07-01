// E - OpenGL - VertexBuffer
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::UIError;
use crate::Graphics;
use std::ffi::c_void;
use gl::types::GLvoid;
use crate::Vec2;
use crate::Vec4;

pub trait Vertex {
    fn bind() -> Vec<GLuint>;
    fn len() -> isize;
}

impl Vertex for Vec2<f32> {
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

impl Vertex for Vec4<f32> {
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

pub struct VertexBuffer<T: Vertex> {
    _vertices: Vec<T>,
    vbo: GLuint,
}

impl<T: Vertex> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,&self.vbo);
        }
    }
}

impl Graphics {
    pub fn create_vertexbuffer<T: Vertex>(&self,vertices: Vec<T>) -> Result<VertexBuffer<T>,UIError> {
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

    pub fn bind_vertexbuffer<T: Vertex>(&self,vertexbuffer: &VertexBuffer<T>) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER,vertexbuffer.vbo) };
        self.vaas.set(T::bind());
    }

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
