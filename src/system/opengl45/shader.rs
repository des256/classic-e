// E - OpenGL - Shader
// Desmond Germans, 2020

use std::ffi::CString;
use std::ptr::null;
use gl::types::GLint;
use std::ffi::CStr;
use gl::types::GLuint;
use std::ptr::null_mut;
use gl::types::GLchar;
use gl::types::GLfloat;
use crate::UIError;
use crate::Graphics;
use crate::f32_2;
use crate::u32_2;
use crate::f32_4;

pub struct Shader {
    pub(crate) sp: GLuint,
}

pub trait OpenGLUniform {
    fn set_uniform(location: i32,value: Self);
}

impl OpenGLUniform for f32_2 {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform2fv(location,1,&value as *const Self as *const GLfloat) };
    }
}

impl OpenGLUniform for f32_4 {
    fn set_uniform(location: i32,value: Self) {
        unsafe { gl::Uniform4fv(location,1,&value as *const Self as *const GLfloat) };
    }
}

impl OpenGLUniform for u32_2 {
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

impl Graphics {
    // API-wise, create_shader is caled from the Graphics object, but the Graphics object itself also needs to create some shaders in the constructor
    pub(crate) fn _create_shader(
        vertex_src: &str,
        geometry_src: Option<&str>,
        fragment_src: &str,
    ) -> Result<Shader,UIError> {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let vcstr = CString::new(vertex_src.as_bytes()).unwrap();
            gl::ShaderSource(vs,1,&vcstr.as_ptr(),null());
            gl::CompileShader(vs);
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vs,gl::COMPILE_STATUS,&mut success);
            gl::GetShaderInfoLog(vs,512,null_mut(),info_log.as_mut_ptr() as *mut GLchar);
            let c_str: &CStr = CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: vertex shader errors:\n{}\nvertex shader source:\n{}",str_slice,vertex_src);
            }
            if success != gl::TRUE as GLint {
                return Err(UIError::Generic);
            }

            // compile geometry shader
            let mut gs: u32 = 0;
            if let Some(geometry_src) = geometry_src {
                gs = gl::CreateShader(gl::GEOMETRY_SHADER);
                let gcstr = CString::new(geometry_src.as_bytes()).unwrap();
                gl::ShaderSource(gs,1,&gcstr.as_ptr(),null());
                gl::CompileShader(gs);
                let mut success = gl::FALSE as GLint;
                let mut info_log = Vec::with_capacity(512);
                info_log.set_len(512 - 1);
                gl::GetShaderiv(gs,gl::COMPILE_STATUS,&mut success);
                gl::GetShaderInfoLog(gs,512,null_mut(),info_log.as_mut_ptr() as *mut GLchar);
                let c_str: &CStr = CStr::from_ptr(info_log.as_ptr());
                let str_slice: &str = c_str.to_str().unwrap();
                if str_slice.len() > 0 {
                    println!("Shader: geometry shader errors:\n{}\ngeometry shader source:\n{}",str_slice,geometry_src);
                }
                if success != gl::TRUE as GLint {
                    return Err(UIError::Generic);
                }
            }

            // compile fragment shader
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fcstr = CString::new(fragment_src.as_bytes()).unwrap();
            gl::ShaderSource(fs,1,&fcstr.as_ptr(),null());
            gl::CompileShader(fs);
            gl::GetShaderiv(fs,gl::COMPILE_STATUS,&mut success);
            gl::GetShaderInfoLog(fs,512,null_mut(),info_log.as_mut_ptr() as *mut GLchar);
            let c_str: &CStr = CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: fragment shader errors:\n{}\nfragment shader source:\n{}",str_slice,fragment_src);
            }
            if success != gl::TRUE as GLint {
                return Err(UIError::Generic);
            }

            // link shaders
            let sp = gl::CreateProgram();
            gl::AttachShader(sp,vs);
            if gs != 0 {
                gl::AttachShader(sp,gs);
            }
            gl::AttachShader(sp,fs);
            gl::LinkProgram(sp);
            gl::GetProgramiv(sp,gl::LINK_STATUS,&mut success);
            gl::GetProgramInfoLog(sp,512,null_mut(),info_log.as_mut_ptr() as *mut GLchar);
            let c_str: &CStr = CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: shader program errors:\n{}", str_slice);
            }
            if success != gl::TRUE as GLint {
                return Err(UIError::Generic);
            }

            // and delete references to the separate shaders
            gl::DeleteShader(vs);
            if gs != 0 {
                gl::DeleteShader(gs);
            }
            gl::DeleteShader(fs);

            Ok(Shader {
                sp: sp,
            })
        }
    }

    pub fn create_shader(&self,
        vertex_src: &str,
        geometry_src: Option<&str>,
        fragment_src: &str,
    ) -> Result<Shader,UIError> {
        Self::_create_shader(vertex_src,geometry_src,fragment_src)
    }

    pub fn bind_shader(&mut self,shader: &Shader) {
        unsafe { gl::UseProgram(shader.sp); }
        self.sp = shader.sp;
    }

    pub fn unbind_shader(&mut self) {
        unsafe { gl::UseProgram(0); }
        self.sp = 0;
    }

    pub fn set_uniform<T: OpenGLUniform>(&mut self,name: &str,value: T) {
        let cname = CString::new(name).unwrap();
        let res = unsafe { gl::GetUniformLocation(self.sp,cname.as_ptr() as *const GLchar) };
        T::set_uniform(res,value);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.sp); }
    }
}
