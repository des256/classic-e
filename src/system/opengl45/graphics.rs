// E - OpenGL - Graphics
// Desmond Germans, 2020

use gl::types::GLuint;

pub struct Graphics {
    pub(crate) sp: GLuint,
    pub(crate) vaas: Vec<GLuint>,
}

impl Graphics {
    pub(crate) fn new() -> Graphics {
        Graphics {
            sp: 0,
            vaas: Vec::new(),
        }
    }

    pub fn clear(&mut self,r: f32,g: f32,b: f32,a: f32) {
        unsafe {
            gl::ClearColor(r,g,b,a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangle_fan(&mut self,n: i32) {
        unsafe { gl::DrawArrays(gl::TRIANGLE_FAN,0,n) };
    }
}