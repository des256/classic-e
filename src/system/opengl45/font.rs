// E - OpenGL - Font
// Desmond Germans, 2020

// let font = graphics.load_font("hello.fnt",vec2!(hsize,vsize),spacing);
// graphics.draw_text(vec2!(x,y),"Hello, World!",&font);
// let size = font.measure("Hello, World!");

use crate::Graphics;
use crate::UIError;
use crate::Texture2D;
use crate::ARGB8;
use std::fs::File;
use std::io::prelude::*;
use crate::decode;
use crate::Vec2;
use crate::Vec4;
use crate::prelude::*;
use crate::Rect;
use std::rc::Rc;

const FONT: Vec2<f32> = Vec2 { x: 0.065,y: 0.065, };  // manually found by comparing chrome and html font-size: 24 --> draw_text font size should be similar

pub struct Character {
    n: u32,
    r: Rect<i32>,
    offset: Vec2<i32>,
    advance: i32,
}

pub struct FontProto {
    pub name: String,
    pub scale: u32,
    pub characters: Vec<Character>,
    pub texture: Texture2D<ARGB8>,
}

impl FontProto {
    fn find(&self,c: char) -> Option<&Character> {
        let n = c as u32;
        for ch in &self.characters {
            if ch.n == n {
                return Some(ch);
            }
        }
        None
    }
}

pub struct Font {
    pub proto: Rc<FontProto>,
    pub size: Vec2<f32>,
    pub spacing: f32,
}

impl Font {
    pub fn new(proto: Rc<FontProto>,size: Vec2<f32>,spacing: f32) -> Font {
        Font {
            proto: proto,
            size: size,
            spacing: spacing,
        }
    }

    pub fn measure(&self,text: &str) -> Vec2<f32> {
        let mut lp: Vec2<f32> = vec2!(0.0,0.0);
        let mut min: Vec2<f32> = vec2!(0.0,0.0);
        let mut max: Vec2<f32> = vec2!(0.0,0.0);
        for c in text.chars() {
            if let Some(ch) = self.proto.find(c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    // bottom-left of the character, in GU
                    let ox = lp.x - FONT.x * self.size.x * (ch.offset.x as f32) / (self.proto.scale as f32);
                    let oy = lp.y - FONT.y * self.size.y * (ch.offset.y as f32) / (self.proto.scale as f32);

                    // size of the character, in GU
                    let sx = FONT.x * self.size.x * (ch.r.s.x as f32) / (self.proto.scale as f32);
                    let sy = FONT.y * self.size.y * (ch.r.s.y as f32) / (self.proto.scale as f32);

                    // adjust min and max
                    if ox < min.x {
                        min.x = ox;
                    }
                    if ox + sx > max.x {
                        max.x = ox + sx;
                    }
                    if oy < min.y {
                        min.y = oy;
                    }
                    if oy + sy > max.y {
                        max.y = oy + sy;
                    }

                    // advance
                    lp.x += FONT.x * self.size.x * (ch.advance as f32) / (self.proto.scale as f32) + FONT.x * self.size.x * self.spacing;
                }
                else {
                    // only advance
                    lp.x += 2.0 * FONT.x * self.size.x * (ch.advance as f32) / (self.proto.scale as f32) + FONT.x * self.size.x * self.spacing;  // the choice for double spacing is arbitrary
                }
            }
        }
        max - min
    }
}

impl Graphics {
    pub fn get_font(&self,name: &str,size: Vec2<f32>,spacing: f32) -> Result<Rc<Font>,UIError> {

        // see if font already exists, and refer to that
        {
            let fonts = self.fonts.borrow();
            for font in &*fonts {
                if (name == font.proto.name) && (size == font.size) && (spacing == font.spacing) {
                    return Ok(Rc::clone(&font));
                }
            }
        }

        // see if proto already exists, and create new font for it
        {
            let protos = self.fontprotos.borrow();
            for proto in &*protos {
                if name == proto.name {
                    let font = Rc::new(Font::new(Rc::clone(&proto),size,spacing));
                    self.fonts.borrow_mut().push(Rc::clone(&font));
                    return Ok(font);
                }
            }
        }

        // otherwise load the font
        let mut file = match File::open(name) {
            Ok(file) => file,
            Err(_) => { return Err(UIError::Generic); },
        };
        let mut buffer: Vec<u8> = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Err(UIError::Generic);
        }
        let scale = (buffer[8] as u32) | ((buffer[9] as u32) << 8) | ((buffer[10] as u32) << 16) | ((buffer[11] as u32) << 24);
        let count = ((buffer[12] as u32) | ((buffer[13] as u32) << 8) | ((buffer[14] as u32) << 16) | ((buffer[15] as u32) << 24)) as usize;
        let mut characters: Vec<Character> = Vec::new();
        for i in 0..count {
            let n = (buffer[16 + i * 32] as u32) | ((buffer[17 + i * 32] as u32) << 8) | ((buffer[18 + i * 32] as u32) << 16) | ((buffer[19 + i * 32] as u32) << 24);
            let rox = (buffer[20 + i * 32] as u32) | ((buffer[21 + i * 32] as u32) << 8) | ((buffer[22 + i * 32] as u32) << 16) | ((buffer[23 + i * 32] as u32) << 24);
            let roy = (buffer[24 + i * 32] as u32) | ((buffer[25 + i * 32] as u32) << 8) | ((buffer[26 + i * 32] as u32) << 16) | ((buffer[27 + i * 32] as u32) << 24);
            let rsx = (buffer[28 + i * 32] as u32) | ((buffer[29 + i * 32] as u32) << 8) | ((buffer[30 + i * 32] as u32) << 16) | ((buffer[31 + i * 32] as u32) << 24);
            let rsy = (buffer[32 + i * 32] as u32) | ((buffer[33 + i * 32] as u32) << 8) | ((buffer[34 + i * 32] as u32) << 16) | ((buffer[35 + i * 32] as u32) << 24);
            let ox = (buffer[36 + i * 32] as u32) | ((buffer[37 + i * 32] as u32) << 8) | ((buffer[38 + i * 32] as u32) << 16) | ((buffer[39 + i * 32] as u32) << 24);
            let oy = (buffer[40 + i * 32] as u32) | ((buffer[41 + i * 32] as u32) << 8) | ((buffer[42 + i * 32] as u32) << 16) | ((buffer[43 + i * 32] as u32) << 24);
            let adv = (buffer[44 + i * 32] as u32) | ((buffer[45 + i * 32] as u32) << 8) | ((buffer[46 + i * 32] as u32) << 16) | ((buffer[47 + i * 32] as u32) << 24);
            characters.push(Character {
                n: n,
                r: rect!(rox as i32,roy as i32,rsx as i32,rsy as i32),
                offset: vec2!(ox as i32,oy as i32),
                advance: adv as i32,
            });
        }
        let image = decode::<ARGB8>(&buffer[16 + count * 32..]).expect("unable to decode");
        let texture = self.create_texture2d::<ARGB8>(&image).expect("unable to create font texture");

        let proto = Rc::new(FontProto {
            name: name.to_string(),
            scale: scale,
            characters: characters,
            texture: texture,
        });
        self.fontprotos.borrow_mut().push(Rc::clone(&proto));
        let font = Rc::new(Font::new(proto,size,spacing));
        self.fonts.borrow_mut().push(Rc::clone(&font));
        Ok(font)
    }

    pub fn draw_text(&self,p: Vec2<f32>,text: &str,font: &Rc<Font>) {
        let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut lp = vec2!(p.x as f32,p.y as f32);
        let mut count = 0;
        for c in text.chars() {
            if let Some(ch) = font.proto.find(c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    // bottom-left of the character, in GU
                    let ox = lp.x - FONT.x * font.size.x * (ch.offset.x as f32) / (font.proto.scale as f32);
                    let oy = lp.y - FONT.y * font.size.y * (ch.offset.y as f32) / (font.proto.scale as f32);

                    // size of the character, in GU
                    let sx = FONT.x * font.size.x * (ch.r.s.x as f32) / (font.proto.scale as f32);
                    let sy = FONT.y * font.size.y * (ch.r.s.y as f32) / (font.proto.scale as f32);

                    // texture divisor
                    let tdx = 1.0 / (font.proto.texture.size.x as f32);
                    let tdy = 1.0 / (font.proto.texture.size.y as f32);

                    // texture coordinates
                    let tox = tdx * (ch.r.o.x as f32);
                    let toy = tdy * (ch.r.o.y as f32);
                    let tsx = tdx * (ch.r.s.x as f32);
                    let tsy = tdy * (ch.r.s.y as f32);

                    // add quad
                    vertices.push(vec4!(ox,oy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy,tox + tsx,toy));
                    vertices.push(vec4!(ox + sx,oy + sy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy,tox,toy));
                    vertices.push(vec4!(ox + sx,oy + sy,tox + tsx,toy + tsy));
                    vertices.push(vec4!(ox,oy + sy,tox,toy + tsy));

                    // advance
                    lp.x += FONT.x * font.size.x * (ch.advance as f32) / (font.proto.scale as f32) + FONT.x * font.size.x * font.spacing;

                    count += 1;
                }
                else {
                    // only advance
                    lp.x += 2.0 * FONT.x * font.size.x * (ch.advance as f32) / (font.proto.scale as f32) + FONT.x * font.size.x * font.spacing;  // the choice for double spacing is arbitrary
                }
            }
        }

        let vertexbuffer = self.create_vertexbuffer(vertices).expect("what?");
        self.bind_vertexbuffer(&vertexbuffer);
        self.bind_msdf_shader();
        self.bind_texture2d(0,&font.proto.texture);
        let scale = self.scale.get();
        let size = self.size.get();
        let color = self.color.get();
        self.set_uniform("scale",vec2!(scale.x / (size.x as f32),scale.y / (size.y as f32)));
        self.set_uniform("font_texture",0);
        self.set_uniform("color",Vec4::<f32>::from(color));
        self.draw_triangles(6 * count);
        self.unbind_vertexbuffer();
        self.unbind_shader();
        self.unbind_texture2d(0);
    }
}
