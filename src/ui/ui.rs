// E - UI
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

use gl::types::{
    GLuint,
    GLvoid,
};

/// UI subsystem.
pub struct UI {
    pub system: Rc<System>,
    pub graphics: Rc<gpu::Graphics>,
    pub uber_shader: gpu::Shader,
    pub fonts: RefCell<Vec<Rc<ui::Font>>>,
    pub quad_vertexbuffer: gpu::VertexBuffer<Vec2<f32>>,
}

static QUAD: [Vec2<f32>; 4] = [
    vec2!(0.0,0.0),
    vec2!(1.0,0.0),
    vec2!(1.0,1.0),
    vec2!(0.0,1.0),
];

#[derive(Copy,Clone)]
#[repr(C)]
pub struct Vertex {
    pub(crate) pt: Vec4<f32>,    // should become p: Vec2<u16>, t: Vec2<f32>,
    pub(crate) a: Vec4<f32>,     // should become a: u32,
    pub(crate) b: Vec4<f32>,     // should become b: u32,
    pub(crate) mlfq: Vec4<u32>,  // should become m: u8, l: u8, f: u16,
    // m = mode:
    // 0x01: true if texture source is ARGB, false if texture is alpha, depending on bits 0x06
    // 0x06: which channel dictates the full texture alpha
}

// TODO: this currently means that only OpenGL is accepted; solve this later
impl gpu::GLVertex for Vertex {
    fn bind() -> Vec<GLuint> {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,64,0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,4,gl::FLOAT,gl::FALSE,64,16 as *const GLvoid);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,4,gl::FLOAT,gl::FALSE,64,32 as *const GLvoid);
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribIPointer(3,4,gl::UNSIGNED_INT,64,48 as *const GLvoid);
        }
        vec![0]
    }

    fn len() -> isize {
        64
    }
}

impl UI {
    pub fn new(system: &Rc<System>,graphics: &Rc<gpu::Graphics>) -> Result<UI,SystemError> {
        let uber_vs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            
            layout(location = 0) in vec4 ipt;
            layout(location = 1) in vec4 ia;
            layout(location = 2) in vec4 ib;
            layout(location = 3) in uvec4 imlfq;

            out Vertex {
                vec2 t;
                vec4 a;
                vec4 b;
                flat uvec2 ml;
            } v;

            void main() {
                gl_Position = vec4(
                    -1.0 + 2.0 * ipt.x / canvas_size.x,
                    1.0 - 2.0 * ipt.y / canvas_size.y,
                    0.0,
                    1.0
                );
                v.t = vec2(ipt.z,ipt.w);
                v.a = ia;
                v.b = ib;
                v.ml = uvec2(imlfq.x,imlfq.y);
            }
        "#;
        let uber_fs = r#"
            #version 420 core

            //uniform sampler2DArray textures;
            uniform sampler2D textures;

            in Vertex {
                vec2 t;
                vec4 a;
                vec4 b;
                flat uvec2 ml;
            } v;

            out vec4 o;

            void main() {
                //vec4 t = texture2DArray(textures,vec3(v.t.x,v.t.y,v.ml.y));
                vec4 t = texture2D(textures,vec2(v.t.x,v.t.y));
                if((v.ml.x & 0x01) == 0) {
                    switch(v.ml.x & 0x06) {
                        case 0x00: t = t.xxxx; break;
                        case 0x02: t = t.yyyy; break;
                        case 0x04: t = t.zzzz; break;
                        case 0x06: t = t.wwww; break;
                    }
                }
                float ta = t.w;
                o = (1.0 - ta) * v.a + v.b * t;
            }
        "#;
        let uber_shader = gpu::Shader::new(&graphics,uber_vs,None,uber_fs).expect("what?");

        let quad_vertexbuffer = match gpu::VertexBuffer::new(&graphics,QUAD.to_vec()) {
            Ok(vertexbuffer) => vertexbuffer,
            Err(_) => { return Err(SystemError::Generic); },
        };

        Ok(UI {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            uber_shader: uber_shader,
            fonts: RefCell::new(Vec::new()),
            quad_vertexbuffer: quad_vertexbuffer,
        })
    }

    pub fn get_font(&self,filename: &str) -> Result<Rc<ui::Font>,SystemError> {

        // see if font already exists, and refer to that
        {
            let fonts = self.fonts.borrow();
            for font in fonts.iter() {
                if filename == font.filename {
                    return Ok(Rc::clone(font));
                }
            }
        }

        // otherwise load the font
        let font = Rc::new(ui::Font::new(self,filename).expect("unable to load font"));
        self.fonts.borrow_mut().push(Rc::clone(&font));
        Ok(font)
    }
}