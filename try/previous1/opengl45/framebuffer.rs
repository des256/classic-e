// e::video::Framebuffer: framebuffer render target
// by Desmond Germans, 2019

use std::os::raw::c_void;
#[doc(no_inline)]
extern crate gl;
use gl::types::GLuint;

/// Render target.
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub tex: GLuint,
    pub rbo: GLuint,
    pub fbo: GLuint,
}

impl Framebuffer {
    /// Create framebuffer object.
    /// # Arguments
    /// * `width` - Internal width of this framebuffer.
    /// * `height` - Internal height of this framebuffer.
    /// # Returns
    /// Newly created framebuffer object.
    pub fn new(width: u32,height: u32) -> Framebuffer {
        let mut tex: GLuint = 0;
        let mut rbo: GLuint = 0;
        let mut fbo: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1,&mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER,fbo);
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGB as i32,width as i32,height as i32,0,gl::RGB,gl::UNSIGNED_BYTE,0 as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER,gl::COLOR_ATTACHMENT0,gl::TEXTURE_2D,tex,0);
            gl::GenRenderbuffers(1,&mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER,rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER,gl::DEPTH_COMPONENT16,width as i32,height as i32);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER,gl::DEPTH_ATTACHMENT,gl::RENDERBUFFER,rbo);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer: unable to complete framebuffer object");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
        }

        Framebuffer {
            width: width,
            height: height,
            tex: tex,
            rbo: rbo,
            fbo: fbo,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER,self.fbo);
            gl::Viewport(0,0,self.width as i32,self.height as i32);
            gl::Scissor(0,0,self.width as i32,self.height as i32);
        }
    }
}

impl Drop for Framebuffer {
    /// Destroy the framebuffer.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1,&self.fbo);
            gl::DeleteRenderbuffers(1,&self.rbo);
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
