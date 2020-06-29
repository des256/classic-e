// e texture2d (OpenGL 4.5)
// by Desmond Germans, 2019

use std::os::raw::c_void;
use gl::types::GLuint;
use crate::*;

pub struct Texture2D {
    //size: isize_2,
    tex: GLuint,
}

impl Texture2D {
    pub fn new(size: isize_2) -> Texture2D {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,gl::RGBA8,size.x as i32,size.y as i32);
        }
        Texture2D {
            //size: size,
            tex: tex,
        }
    }

    pub fn upload_image(&mut self,p: isize_2,image: &Image) {
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D,0,p.x as i32,p.y as i32,image.size.x as i32,image.size.y as i32,gl::RGBA,gl::UNSIGNED_INT_8_8_8_8_REV,image.data.as_ptr() as *const u32 as *const c_void);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D,self.tex); }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1,&self.tex); }
    }
}
