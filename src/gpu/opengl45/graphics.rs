// E - GPU (OpenGL 4.5) - Graphics
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
    ffi::CString,
    //ptr::null_mut,
};
use gl::types::{
    GLuint,
    GLfloat,
    GLchar,
    GLenum,
};
#[cfg(target_os="linux")]
use x11::{
    xlib::XID,
    glx::{
        glXMakeCurrent,
        glXSwapBuffers,
    },
};
#[cfg(target_os="windows")]
use {
    std::ptr::null_mut,
    winapi::{
        um::wingdi::{
            wglMakeCurrent,
            SwapBuffers,
        },
        shared::windef::HDC,
    },
};

/// Graphics context.
pub struct Graphics {
    system: Rc<System>,
    pub(crate) sp: Cell<GLuint>,
    pub(crate) index_type: Cell<GLenum>,
    pub(crate) target_is_framebuffer: Cell<bool>,
#[cfg(target_os="linux")]
    pub(crate) target_id: Cell<XID>,
#[cfg(target_os="windows")]
    pub(crate) target_hdc: Cell<HDC>,
}

#[doc(hidden)]
pub trait OpenGLUniform {
    fn set_uniform(location: i32,value: Self);
}

impl OpenGLUniform for Vec2<f32> {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform2fv(location,1,&value as *const Self as *const GLfloat) };
    }
}

impl OpenGLUniform for Vec3<f32> {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform3fv(location,1,&value as *const Self as *const GLfloat) };
    }
}

impl OpenGLUniform for Vec4<f32> {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform4fv(location,1,&value as *const Self as *const GLfloat) };
    }
}

impl OpenGLUniform for Vec2<u32> {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform2uiv(location,1,&value as *const Self as *const GLuint) };
    }
}

impl OpenGLUniform for i32 {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform1i(location,value) };
    }
}

impl OpenGLUniform for u32 {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform1ui(location,value) };
    }
}

#[doc(hidden)]
pub trait BindTarget {
    fn do_bind(&self,graphics: &Graphics);
}

impl BindTarget for Rc<gpu::Framebuffer> {
    fn do_bind(&self,graphics: &Graphics) {
        unsafe {
#[cfg(target_os="linux")]
            glXMakeCurrent(graphics.system.connection.get_raw_dpy(),graphics.system.hidden_window,graphics.system.context);
#[cfg(target_os="windows")]
            wglMakeCurrent(graphics.system.hidden_hdc,graphics.system.hglrc);
            gl::BindFramebuffer(gl::FRAMEBUFFER,self.fbo);
            gl::Viewport(0,0,self.size.x as i32,self.size.y as i32);
            gl::Scissor(0,0,self.size.x as i32,self.size.y as i32);
        }
        graphics.target_is_framebuffer.set(true);
    }
}

impl BindTarget for Rc<Window> {
    fn do_bind(&self,graphics: &Graphics) {
        let size = self.size.get();
        unsafe {
#[cfg(target_os="linux")]
            glXMakeCurrent(graphics.system.connection.get_raw_dpy(),self.id,graphics.system.context);
#[cfg(target_os="windows")]
            wglMakeCurrent(self.hdc,self.system.hglrc);
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
            gl::Viewport(0,0,size.x as i32,size.y as i32);
            gl::Scissor(0,0,size.x as i32,size.y as i32);
        }
        graphics.target_is_framebuffer.set(false);
#[cfg(target_os="linux")]
        graphics.target_id.set(self.id);
#[cfg(target_os="windows")]
        graphics.target_hdc.set(self.hdc);
    }
}

#[doc(hidden)]
pub trait BindTexture {
    fn do_bind(&self,graphics: &Graphics,stage: usize);
}

impl BindTexture for gpu::Framebuffer {
    fn do_bind(&self,_graphics: &Graphics,stage: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + stage as u32);
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
        }
    }
}

impl<T: gpu::GLFormat> BindTexture for gpu::Texture1D<T> {
    fn do_bind(&self,_graphics: &Graphics,stage: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + stage as u32);
            gl::BindTexture(gl::TEXTURE_1D,self.tex);
        }
    }
}

impl<T: gpu::GLFormat> BindTexture for gpu::Texture2D<T> {
    fn do_bind(&self,_graphics: &Graphics,stage: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + stage as u32);
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
        }
    }
}

impl<T: gpu::GLFormat> BindTexture for gpu::Texture3D<T> {
    fn do_bind(&self,_graphics: &Graphics,stage: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + stage as u32);
            gl::BindTexture(gl::TEXTURE_3D,self.tex);
        }
    }
}

impl<T: gpu::GLFormat> BindTexture for gpu::TextureCube<T> {
    fn do_bind(&self,_graphics: &Graphics,stage: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + stage as u32);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP,self.tex);
        }
    }
}

impl Graphics {
    /// Create new graphics context.
    /// # Arguments
    /// * `system` - System to create the graphics context for.
    /// # Returns
    /// * `Ok(GPU)` - The created graphics context.
    /// * `Err(SystemError)` - The graphics context could not be created.
    pub fn new(system: &Rc<System>) -> Result<Graphics,SystemError> {
        Ok(Graphics {
            system: Rc::clone(system),
            sp: Cell::new(0),
            index_type: Cell::new(gl::UNSIGNED_INT),
            target_is_framebuffer: Cell::new(false),
#[cfg(target_os="linux")]
            target_id: Cell::new(0),
#[cfg(target_os="windows")]
            target_hdc: Cell::new(null_mut()),
        })
    }

    /// (temporary) Bind current target.
    /// # Arguments
    /// * `target` - Either `Framebuffer` or `Window`.
    pub fn bind_target<T: BindTarget>(&self,target: &T) {
        target.do_bind(&self);
    }

    /// (temporary) Present target.
    pub fn present(&self) {
        unsafe {
            gl::Flush();
            if !self.target_is_framebuffer.get() {
#[cfg(target_os="linux")]
                glXSwapBuffers(self.system.connection.get_raw_dpy(),self.target_id.get());
#[cfg(target_os="windows")]
                SwapBuffers(self.target_hdc.get());
            }
        }
    }

    /// (temporary) Clear current target.
    /// # Arguments
    /// * `color` - Color to clear with.
    pub fn clear<T>(&self,color: T) where Vec4<f32>: From<T> {
        let color = Vec4::<f32>::from(color);
        unsafe {
            gl::ClearColor(color.x,color.y,color.z,color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// (temporary) Draw triangle fan.
    /// # Arguments
    /// * `n` - Number of vertices.
    pub fn draw_triangle_fan(&self,n: i32) {
        unsafe { gl::DrawArrays(gl::TRIANGLE_FAN,0,n) };
    }

    /// (temporary) Draw triangles.
    /// # Arguments
    /// * `n` - Number of vertices.
    pub fn draw_triangles(&self,n: i32) {
        unsafe { gl::DrawArrays(gl::TRIANGLES,0,n) };
    }

    /// (temporary) Draw indexed triangle fan.
    /// # Arguments
    /// * `n` - Number of vertices.
    pub fn draw_indexed_triangle_fan(&self,n: i32) {
        unsafe { gl::DrawElements(gl::TRIANGLE_FAN,n,self.index_type.get(),null_mut()) };
    }

    /// (temporary) Draw triangles.
    /// # Arguments
    /// * `n` - Number of vertices.
    pub fn draw_indexed_triangles(&self,n: i32) {
        unsafe { gl::DrawElements(gl::TRIANGLES,n,self.index_type.get(),null_mut()) };
    }

    /// (temporary) Set blending mode.
    /// # Arguments
    /// * `mode` - Blending mode.
    pub fn set_blend(&self,mode: gpu::BlendMode) {
        match mode {
            gpu::BlendMode::Replace => unsafe { gl::Disable(gl::BLEND); },
            _ => unsafe { gl::Enable(gl::BLEND); },
        }
        match mode {
            gpu::BlendMode::Over => unsafe { gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA); },
            _ => { },
        }
    }

    /// (temporary) Bind texture or framebuffer to a texture stage.
    /// # Arguments
    /// * `stage` - Texture stage to bind to.
    /// * `texture` - Texture or framebuffer to bind.
    pub fn bind_texture<T: BindTexture>(&self,stage: usize,texture: &T) {
        texture.do_bind(&self,stage);
    }

    /// (temporary) Bind current shader program.
    /// # Arguments
    /// * `shader` - Shader program.
    pub fn bind_shader(&self,shader: &gpu::Shader) {
        unsafe { gl::UseProgram(shader.sp); }
        self.sp.set(shader.sp);
    }

    /// (temporary) Set uniform value for current shader program.
    /// # Arguments
    /// * `name` - Variable name referenced in the shader program.
    /// * `value` - Value of the uniform.
    pub fn set_uniform<T: OpenGLUniform>(&self,name: &str,value: T) {
        let cname = CString::new(name).unwrap();
        let res = unsafe { gl::GetUniformLocation(self.sp.get(),cname.as_ptr() as *const GLchar) };
        T::set_uniform(res,value);
    }

    /// (temporary) Bind current vertex buffer.
    /// # Arguments
    /// * `vertexbuffer` - Vertexbuffer to bind.
    pub fn bind_vertexbuffer<T: gpu::GLVertex>(&self,vertexbuffer: &gpu::VertexBuffer<T>) {
        unsafe { gl::BindVertexArray(vertexbuffer.vao) };
    }

    /// (temporary) Bind current index buffer.
    /// # Arguments
    /// * `indexbuffer` - Indexbuffer to bind.
    pub fn bind_indexbuffer<T: gpu::GLIndex>(&self,indexbuffer: &gpu::IndexBuffer<T>) {
        self.index_type.set(T::gl_type());
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,indexbuffer.ibo) };
    }
}
