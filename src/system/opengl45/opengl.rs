// E - OpenGL - Graphics
// Desmond Germans, 2020

use crate::*;
use gl::types::GLuint;
use std::cell::Cell;

pub struct OpenGL {
    pub(crate) sp: Cell<GLuint>,
    pub(crate) vaas: Cell<Vec<GLuint>>,
}

pub enum BlendMode {
    Replace,
    Over,
}

impl OpenGL {
    pub(crate) fn new() -> Result<OpenGL,SystemError> {

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }
        
        Ok(OpenGL {
            sp: Cell::new(0),
            vaas: Cell::new(Vec::new()),
        })
    }

    pub fn clear<T>(&self,color: T) where Vec4<f32>: From<T> {
        let color = Vec4::<f32>::from(color);
        unsafe {
            gl::ClearColor(color.x,color.y,color.z,color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangle_fan(&self,n: i32) {
        unsafe { gl::DrawArrays(gl::TRIANGLE_FAN,0,n) };
    }

    pub fn draw_triangles(&self,n: i32) {
        unsafe { gl::DrawArrays(gl::TRIANGLES,0,n) };
    }

    pub fn set_blend(&self,mode: BlendMode) {
        match mode {
            BlendMode::Replace => unsafe { gl::Disable(gl::BLEND); },
            _ => unsafe { gl::Enable(gl::BLEND); },
        }
        match mode {
            BlendMode::Over => unsafe { gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA); },
            _ => { },
        }
    }
}
