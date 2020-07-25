// E - UI
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;

/// UI subsystem.
pub struct UI {
    pub system: Rc<System>,
    pub gpu: Rc<gpu::GPU>,
    pub msdf_shader: gpu::Shader,
    pub font_protos: RefCell<Vec<Rc<ui::FontProto>>>,
    pub fonts: RefCell<Vec<Rc<ui::Font>>>,
}

impl UI {
    pub fn new(system: &Rc<System>,gpu: &Rc<gpu::GPU>) -> Result<UI,SystemError> {

        let vs = r#"
            #version 420 core

            uniform vec2 ppu;
            uniform vec2 size;

            layout(location = 0) in vec4 p;

            out vec2 tc;

            void main() {
                tc = vec2(p.z,1.0 - p.w);
                gl_Position = vec4(-1.0 + 2.0 * ppu.x * p.x / size.x,-1.0 + 2.0 * ppu.y * p.y / size.y,0.0,1.0);
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

        let msdf_shader = gpu::Shader::new(&gpu,vs,None,fs).expect("what?");

        Ok(UI {
            system: Rc::clone(system),
            gpu: Rc::clone(gpu),
            msdf_shader: msdf_shader,
            font_protos: RefCell::new(Vec::new()),
            fonts: RefCell::new(Vec::new()),
        })
    }

    pub fn get_font(&self,name: &str,size: Vec2<f32>,spacing: f32) -> Result<Rc<ui::Font>,SystemError> {

        // see if font already exists, and refer to that
        {
            let fonts = self.fonts.borrow();
            for font in fonts.iter() {
                if (name == font.proto.name) && (size == font.size) && (spacing == font.spacing) {
                    return Ok(Rc::clone(font));
                }
            }
        }

        // see if proto already exists, and create new font for it
        {
            let protos = self.font_protos.borrow();
            for proto in protos.iter() {
                if name == proto.name {
                    let font = Rc::new(ui::Font::new(&proto,size,spacing));
                    self.fonts.borrow_mut().push(Rc::clone(&font));
                    return Ok(font);
                }
            }
        }

        // otherwise load the font
        let mut file = match File::open(name) {
            Ok(file) => file,
            Err(_) => { return Err(SystemError::Generic); },
        };
        let mut buffer: Vec<u8> = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Err(SystemError::Generic);
        }
        let scale = (buffer[8] as u32) | ((buffer[9] as u32) << 8) | ((buffer[10] as u32) << 16) | ((buffer[11] as u32) << 24);
        let count = ((buffer[12] as u32) | ((buffer[13] as u32) << 8) | ((buffer[14] as u32) << 16) | ((buffer[15] as u32) << 24)) as usize;
        let mut characters: Vec<ui::Character> = Vec::new();
        for i in 0..count {
            let n = (buffer[16 + i * 32] as u32) | ((buffer[17 + i * 32] as u32) << 8) | ((buffer[18 + i * 32] as u32) << 16) | ((buffer[19 + i * 32] as u32) << 24);
            let rox = (buffer[20 + i * 32] as u32) | ((buffer[21 + i * 32] as u32) << 8) | ((buffer[22 + i * 32] as u32) << 16) | ((buffer[23 + i * 32] as u32) << 24);
            let roy = (buffer[24 + i * 32] as u32) | ((buffer[25 + i * 32] as u32) << 8) | ((buffer[26 + i * 32] as u32) << 16) | ((buffer[27 + i * 32] as u32) << 24);
            let rsx = (buffer[28 + i * 32] as u32) | ((buffer[29 + i * 32] as u32) << 8) | ((buffer[30 + i * 32] as u32) << 16) | ((buffer[31 + i * 32] as u32) << 24);
            let rsy = (buffer[32 + i * 32] as u32) | ((buffer[33 + i * 32] as u32) << 8) | ((buffer[34 + i * 32] as u32) << 16) | ((buffer[35 + i * 32] as u32) << 24);
            let ox = (buffer[36 + i * 32] as u32) | ((buffer[37 + i * 32] as u32) << 8) | ((buffer[38 + i * 32] as u32) << 16) | ((buffer[39 + i * 32] as u32) << 24);
            let oy = (buffer[40 + i * 32] as u32) | ((buffer[41 + i * 32] as u32) << 8) | ((buffer[42 + i * 32] as u32) << 16) | ((buffer[43 + i * 32] as u32) << 24);
            let adv = (buffer[44 + i * 32] as u32) | ((buffer[45 + i * 32] as u32) << 8) | ((buffer[46 + i * 32] as u32) << 16) | ((buffer[47 + i * 32] as u32) << 24);
            characters.push(ui::Character {
                n: n,
                r: rect!(rox as i32,roy as i32,rsx as i32,rsy as i32),
                offset: vec2!(ox as i32,oy as i32),
                advance: adv as i32,
            });
        }
        let image = image::decode::<pixel::ARGB8>(&buffer[16 + count * 32..]).expect("unable to decode");
        let texture = gpu::Texture2D::<pixel::ARGB8>::new(&self.gpu,&image).expect("unable to create font texture");

        let proto = Rc::new(ui::FontProto {
            name: name.to_string(),
            scale: scale,
            characters: characters,
            texture: texture,
        });
        self.font_protos.borrow_mut().push(Rc::clone(&proto));
        let font = Rc::new(ui::Font::new(&proto,size,spacing));
        self.fonts.borrow_mut().push(Rc::clone(&font));
        Ok(font)
    }
}