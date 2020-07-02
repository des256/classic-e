// E - OpenGL - Graphics
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::Shader;
use crate::Vec2;
use std::cell::Cell;
use crate::prelude::*;
use std::rc::Rc;
use crate::Font;
use crate::FontProto;
use std::cell::RefCell;
use crate::Vec4;
use crate::Rect;

pub struct Graphics {
    pub(crate) sp: Cell<GLuint>,
    pub(crate) vaas: Cell<Vec<GLuint>>,
    pub(crate) msdf_shader: Shader,
    pub(crate) color_shader: Shader,
    pub(crate) size: Cell<Vec2<usize>>,
    pub(crate) scale: Cell<Vec2<f32>>,
    pub(crate) color: Cell<Vec4<f32>>,
    pub(crate) fontprotos: RefCell<Vec<Rc<FontProto>>>,
    pub(crate) fonts: RefCell<Vec<Rc<Font>>>,
}

const SCREEN: Vec2<f32> = Vec2 { x: 1.0,y: 1.0, };  // pixels per GU

pub enum BlendMode {
    Replace,
    Over,
}

fn build_msdf_shader() -> Shader {
    let vs = r#"
        #version 420 core

        uniform vec2 scale;

        layout(location = 0) in vec4 p;

        out vec2 tc;

        void main() {
            tc = vec2(p.z,1.0 - p.w);
            gl_Position = vec4(-1.0 + 2.0 * scale.x * p.x,-1.0 + 2.0 * scale.y * p.y,0.0,1.0);
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

    Graphics::_create_shader(vs,None,fs).expect("what?")
}

fn build_color_shader() -> Shader {
    let vs = r#"
        #version 420 core

        uniform vec2 scale;

        layout(location = 0) in vec4 p;

        void main() {
            gl_Position = vec4(-1.0 + 2.0 * scale.x * p.x,-1.0 + 2.0 * scale.y * p.y,0.0,1.0);
        }
    "#;

    let fs = r#"
        #version 420 core

        uniform vec4 color;

        out vec4 frag_color;

        void main() {
            frag_color = color;
        }
    "#;

    Graphics::_create_shader(vs,None,fs).expect("what?")
}

impl Graphics {
    pub(crate) fn new() -> Graphics {

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }

        let msdf_shader = build_msdf_shader();
        let color_shader = build_color_shader();

        // TODO: load default fonts

        Graphics {
            sp: Cell::new(0),
            vaas: Cell::new(Vec::new()),
            msdf_shader: msdf_shader,
            color_shader: color_shader,
            size: Cell::new(vec2!(1,1)),
            scale: Cell::new(SCREEN),
            color: Cell::new(vec4!(1.0,1.0,0.0,1.0)),
            fontprotos: RefCell::new(Vec::new()),
            fonts: RefCell::new(Vec::new()),
        }
    }

    pub fn set_scale(&self,scale: Vec2<f32>) {  // set pixels per GU
        self.scale.set(vec2!(scale.x * SCREEN.x,scale.y * SCREEN.y));
    }

    pub fn get_scale(&self) -> Vec2<f32> {
        self.scale.get()
    }

    pub fn set_window_size(&self,size: Vec2<usize>) {  // set size of the window in pixels
        self.size.set(size);
    }

    pub fn get_space(&self) -> Vec2<f32> {  // get maximum GU coordinates
        let scale = self.scale.get();
        let size = self.size.get();
        vec2!((size.x as f32) / scale.x,(size.y as f32) / scale.y)
    }

    pub fn bind_msdf_shader(&self) {
        unsafe { gl::UseProgram(self.msdf_shader.sp); }
        self.sp.set(self.msdf_shader.sp);
    }

    pub fn bind_color_shader(&self) {
        unsafe { gl::UseProgram(self.color_shader.sp); }
        self.sp.set(self.color_shader.sp);
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

    pub fn draw_rect<T>(&self,r: Rect<f32>,color: T) where Vec4<f32>: From<T> {
        let mut vertices: Vec<Vec2<f32>> = Vec::new();
        vertices.push(vec2!(r.o.x,r.o.y));
        vertices.push(vec2!(r.o.x + r.s.x,r.o.y));
        vertices.push(vec2!(r.o.x + r.s.x,r.o.y + r.s.y));
        vertices.push(vec2!(r.o.x,r.o.y));
        vertices.push(vec2!(r.o.x + r.s.x,r.o.y + r.s.y));
        vertices.push(vec2!(r.o.x,r.o.y + r.s.y));
        let vertexbuffer = self.create_vertexbuffer(vertices).expect("what?");
        self.bind_vertexbuffer(&vertexbuffer);
        self.bind_color_shader();
        let scale = self.scale.get();
        let size = self.size.get();
        self.set_uniform("scale",vec2!(scale.x / (size.x as f32),scale.y / (size.y as f32)));
        self.set_uniform("color",Vec4::<f32>::from(color));
        self.draw_triangles(6);
        self.unbind_vertexbuffer();
        self.unbind_shader();
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

    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }
}