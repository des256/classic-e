// E - OpenGL - Framebuffer
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use gl::types::GLuint;

/// Framebuffer GPU resource.
pub struct Framebuffer {
    pub(crate) fbo: GLuint,
    pub(crate) tex: GLuint,
    pub size: Vec2<usize>,
}

impl Framebuffer {
    /// Create new framebuffer for a graphics context.
    /// 
    /// **Arguments**
    /// 
    /// * `graphics` - Graphics context to create the framebuffer for.
    /// * `size` - Dimensions of the framebuffer.
    /// 
    /// **Returns**
    /// 
    /// * `Ok(Framebuffer)` - The new framebuffer.
    /// * `Err(SystemError)` - The framebuffer could not be created.
    pub fn new(_graphics: &Rc<gpu::Graphics>,size: Vec2<usize>) -> Result<Framebuffer,SystemError> {
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
            gl::TexStorage2D(gl::TEXTURE_2D,1,gl::RGBA8,size.x() as i32,size.y() as i32);
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
