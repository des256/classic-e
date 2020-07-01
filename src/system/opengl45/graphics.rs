// E - OpenGL - Graphics
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::Shader;
use crate::Vec2;
use crate::ARGB8;
use crate::Pixel;
use std::cell::Cell;
use crate::prelude::*;

pub struct Graphics {
    pub(crate) sp: Cell<GLuint>,
    pub(crate) vaas: Cell<Vec<GLuint>>,
    pub(crate) msdf_shader: Shader,
    pub(crate) size: Cell<Vec2<usize>>,
    pub(crate) scale: Cell<Vec2<f32>>,
    pub(crate) color: Cell<ARGB8>,
}

const SCREEN: Vec2<f32> = Vec2 { x: 2.0,y: 2.0, };  // pixels per GU

pub enum BlendMode {
    Replace,
    Over,
}

impl Graphics {
    pub(crate) fn new() -> Graphics {

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }

        let vs = r#"
            #version 420 core

            uniform vec2 scale;

            layout(location = 0) in vec4 p;

            out vec2 tc;

            void main() {
                tc = vec2(p.z,1.0 - p.w);
                gl_Position = vec4(-1.0 + scale.x * p.x,-1.0 + scale.y * p.y,0.0,1.0);
            }
        "#;

        let fs = r#"
            #version 420 core

            uniform sampler2D font_texture;
            uniform vec4 color;

            in vec2 tc;

            out vec4 frag_color;

            float median(float r,float g,float b) {
                return max(min(r,g),min(max(r,g),b));
            }

            void main() {
                vec3 t = texture(font_texture,tc).rgb;
                vec2 unit = (4.0 / textureSize(font_texture,0)).xy;
                float dist = median(t.r,t.g,t.b) - 0.5;
                dist *= dot(unit,0.5 / fwidth(tc));
                float cov = clamp(dist + 0.5,0.0,1.0);
                frag_color = vec4(color.xyz,color.w * cov);
            }
        "#;

        let msdf_shader = Graphics::_create_shader(vs,None,fs).expect("what?");

        Graphics {
            sp: Cell::new(0),
            vaas: Cell::new(Vec::new()),
            msdf_shader: msdf_shader,
            size: Cell::new(vec2!(1,1)),
            scale: Cell::new(SCREEN),
            color: Cell::new(ARGB8::new_rgb(255,0,0)),
        }
    }

    pub fn set_scale(&self,scale: Vec2<f32>) {
        self.scale.set(vec2!(scale.x * SCREEN.x,scale.y * SCREEN.y));
    }

    pub fn set_window_size(&self,size: Vec2<usize>) {
        self.size.set(size);
    }

    pub fn bind_msdf_shader(&self) {
        unsafe { gl::UseProgram(self.msdf_shader.sp); }
        self.sp.set(self.msdf_shader.sp);
    }

    pub fn clear(&self,r: f32,g: f32,b: f32,a: f32) {
        unsafe {
            gl::ClearColor(r,g,b,a);
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

    pub fn set_color(&self,color: ARGB8) {
        self.color.set(color);
    }
}