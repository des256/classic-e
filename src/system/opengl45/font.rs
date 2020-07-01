// E - OpenGL - Font
// Desmond Germans, 2020

use crate::Graphics;
use crate::UIError;
use crate::Texture2D;
use crate::ARGB8;
use crate::i32_r;
use crate::i32_2;
use std::fs::File;
use std::io::prelude::*;
use crate::decode;
use crate::f32_4;
use crate::f32_2;
use crate::Pixel;

const FONT: f32_2 = f32_2 { x: 0.065,y: 0.065, };  // manually found by comparing chrome and html font-size: 24 --> draw_text font size should be similar

pub struct Character {
    n: u32,
    r: i32_r,
    offset: i32_2,
    advance: i32,
}

pub struct Font {
    pub scale: u32,
    pub characters: Vec<Character>,
    pub texture: Texture2D<ARGB8>,
}

fn find_character(font: &Font,c: char) -> Option<&Character> {
    let n = c as u32;
    for ch in &font.characters {
        if ch.n == n {
            return Some(ch);
        }
    }
    None
}

impl Graphics {
    pub fn load_font(&self,name: &str) -> Result<Font,UIError> {
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
                r: i32_r {
                    o: i32_2 { x: rox as i32,y: roy as i32, },
                    s: i32_2 { x: rsx as i32,y: rsy as i32, },
                },
                offset: i32_2 { x: ox as i32,y: oy as i32, },
                advance: adv as i32,
            });
        }
        let image = decode::<ARGB8>(&buffer[16 + count * 32..]).expect("unable to decode");
        let texture = self.create_texture2d::<ARGB8>(&image).expect("unable to create font texture");
        Ok(Font {
            scale: scale,
            characters: characters,
            texture: texture,
        })
    }

    pub fn draw_text(&mut self,p: f32_2,text: &str,font: &Font,font_size: f32_2,font_spacing: f32) {
        let mut vertices: Vec<f32_4> = Vec::new();
        let mut lp = f32_2 { x: p.x as f32,y: p.y as f32, };
        let mut count = 0;
        for c in text.chars() {
            if let Some(ch) = find_character(font,c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    // bottom-left of the character, in GU
                    let ox = lp.x - FONT.x * font_size.x * (ch.offset.x as f32) / (font.scale as f32);
                    let oy = lp.y - FONT.y * font_size.y * (ch.offset.y as f32) / (font.scale as f32);

                    // size of the character, in GU
                    let sx = FONT.x * font_size.x * (ch.r.s.x as f32) / (font.scale as f32);
                    let sy = FONT.y * font_size.y * (ch.r.s.y as f32) / (font.scale as f32);

                    // texture divisor
                    let tdx = 1.0 / (font.texture.size.x as f32);
                    let tdy = 1.0 / (font.texture.size.y as f32);

                    // texture coordinates
                    let tox = tdx * (ch.r.o.x as f32);
                    let toy = tdy * (ch.r.o.y as f32);
                    let tsx = tdx * (ch.r.s.x as f32);
                    let tsy = tdy * (ch.r.s.y as f32);

                    // add quad
                    vertices.push(f32_4 { x: ox,y: oy,z: tox,w: toy, });
                    vertices.push(f32_4 { x: ox + sx,y: oy,z: tox + tsx,w: toy, });
                    vertices.push(f32_4 { x: ox + sx,y: oy + sy,z: tox + tsx,w: toy + tsy, });
                    vertices.push(f32_4 { x: ox,y: oy,z: tox,w: toy, });
                    vertices.push(f32_4 { x: ox + sx,y: oy + sy,z: tox + tsx,w: toy + tsy, });
                    vertices.push(f32_4 { x: ox,y: oy + sy,z: tox,w: toy + tsy, });
                    lp.x += FONT.x * font_size.x * (ch.advance as f32) / (font.scale as f32) + FONT.x * font_size.x * font_spacing;
                    count += 1;
                }
                else {
                    lp.x += 2.0 * FONT.x * font_size.x * (ch.advance as f32) / (font.scale as f32) + FONT.x * font_size.x * font_spacing;  // the choice for double spacing is arbitrary
                }
            }
        }

        let vertexbuffer = self.create_vertexbuffer(vertices).expect("what?");
        self.bind_vertexbuffer(&vertexbuffer);
        self.bind_msdf_shader();
        self.bind_texture2d(0,&font.texture);
        self.set_uniform("scale",f32_2 { x: self.scale.x / (self.size.x as f32),y: self.scale.y / (self.size.y as f32), });
        self.set_uniform("font_texture",0);
        self.set_uniform("color",f32_4 {
            x: (self.color.r() as f32) / 255.0,
            y: (self.color.g() as f32) / 255.0,
            z: (self.color.b() as f32) / 255.0,
            w: (self.color.a() as f32) / 255.0,
        });
        self.draw_triangles(6 * count);
        self.unbind_vertexbuffer();
        self.unbind_shader();
        self.unbind_texture2d(0);
    }
}
