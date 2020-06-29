// e Texture2DArray (OpenGL 4.5)
// by Desmond Germans, 2019

use std::os::raw::c_void;
use gl::types::GLuint;
use crate::*;

pub struct Texture2DArray {
    //layers: usize,
    //size: isize_2,
    tex: GLuint,
}

impl Texture2DArray {
    pub fn new(layers: usize,size: isize_2) -> Texture2DArray {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY,tex);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage3D(gl::TEXTURE_2D_ARRAY,1,gl::RGBA8,size.x as i32,size.y as i32,layers as i32);
        }
        Texture2DArray {
            //layers: layers,
            //size: size,
            tex: tex,
        }
    }

    pub fn upload_image(&mut self,layer: usize,p: isize_2,image: &Image) {
        unsafe {
            gl::TexSubImage3D(gl::TEXTURE_2D_ARRAY,0,p.x as i32,p.y as i32,layer as i32,image.size.x as i32,image.size.y as i32,1,gl::RGBA,gl::UNSIGNED_INT_8_8_8_8_REV,image.data.as_ptr() as *const u32 as *const c_void);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D_ARRAY,self.tex); }
    }
}

impl Drop for Texture2DArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1,&self.tex); }
    }
}
