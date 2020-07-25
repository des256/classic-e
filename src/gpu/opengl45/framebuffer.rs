// E - OpenGL - Framebuffer
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use gl::types::GLuint;

/// Framebuffer GPU resource.
pub struct Framebuffer {
    fbo: GLuint,
    pub tex: GLuint,
    pub size: Vec2<usize>,
}

impl Framebuffer {
    /// Create new framebuffer for a GPU.
    pub fn new(_gpu: &Rc<gpu::GPU>,size: Vec2<usize>) -> Result<Framebuffer,SystemError> {
        let mut fbo: GLuint = 0;
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1,&mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER,fbo);
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,gl::RGBA8,size.x as i32,size.y as i32);
            gl::FramebufferTexture(gl::FRAMEBUFFER,gl::COLOR_ATTACHMENT0,tex,0);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(SystemError::Generic);
            }
        }
        Ok(Framebuffer {
            fbo: fbo,
            tex: tex,
            size: size,
        })
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1,&self.fbo);
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

impl gpu::GPU {
    /// (temporary) Bind framebuffer as current target.
    pub fn bind_framebuffer(&self,framebuffer: &Framebuffer) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER,framebuffer.fbo);
            gl::Viewport(0,0,framebuffer.size.x as i32,framebuffer.size.y as i32);
            gl::Scissor(0,0,framebuffer.size.x as i32,framebuffer.size.y as i32);
        }
    }

    /// (temporary) Unbind framebuffer as current target.
    pub fn unbind_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
        }
    }

    /// (temporary) Bind framebuffer as 2D texture.
    pub fn bind_framebuffer_as_texture2d(&self,layer: usize,framebuffer: &Framebuffer) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + layer as u32);
            gl::BindTexture(gl::TEXTURE_2D,framebuffer.tex);
        }
    }
}
