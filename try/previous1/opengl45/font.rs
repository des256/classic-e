// e::video::Font: MSDF font
// by Desmond Germans, 2019

use std::{ffi::c_void,fs::File,io::Read};
#[doc(no_inline)]
extern crate gl;
use gl::types::GLuint;
use crate::Canvas;

pub struct Char {
    pub id: i32,
    pub index: i32,
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: i32,
    pub x: i32,
    pub y: i32,
}

pub struct Kerning {
    pub a: i32,
    pub b: i32,
    pub n: i32,
}

pub struct Font<'a> {
    pub canvas: &'a Canvas,
    pub tex: GLuint,
    pub line_height: i32,
    pub base: i32,
    pub scale_w: i32,
    pub scale_h: i32,
    pub chars: Vec<Char>,
    pub kernings: Vec<Kerning>,
}

fn get_u32(slice: &[u8]) -> u32 {
    (slice[0] as u32) | ((slice[1] as u32) << 8) | ((slice[2] as u32) << 16) | ((slice[3] as u32) << 24)
}

impl<'a> Font<'a> {
    pub fn new(canvas: &'a Canvas,name: &str) -> Font<'a> {
        let mut file = File::open(name).unwrap();
        let mut header: [u8;32] = [0;32];
        file.read(&mut header).unwrap();
        let _tag = get_u32(&header[0..4]);
        let _version = get_u32(&header[4..8]);
        let line_height = get_u32(&header[8..12]);
        let base = get_u32(&header[12..16]);
        let scale_w = get_u32(&header[16..20]);
        let scale_h = get_u32(&header[20..24]);
        let num_chars = get_u32(&header[24..28]);
        let num_kernings = get_u32(&header[28..32]);
        let mut chars: Vec<Char> = Vec::new();
        for _i in 0..num_chars {
            let mut buffer: [u8;36] = [0;36];
            file.read(&mut buffer).unwrap();
            let id = get_u32(&buffer[0..4]) as i32;
            let index = get_u32(&buffer[4..8]) as i32;
            let width = get_u32(&buffer[8..12]) as i32;
            let height = get_u32(&buffer[12..16]) as i32;
            let xoffset = get_u32(&buffer[16..20]) as i32;
            let yoffset = get_u32(&buffer[20..24]) as i32;
            let xadvance = get_u32(&buffer[24..28]) as i32;
            let x = get_u32(&buffer[28..32]) as i32;
            let y = get_u32(&buffer[32..36]) as i32;
            chars.push(Char {
                id: id,
                index: index,
                width: width,
                height: height,
                xoffset: xoffset,
                yoffset: yoffset,
                xadvance: xadvance,
                x: x,
                y: y,
            });
        }
        let mut kernings: Vec<Kerning> = Vec::new();
        for _i in 0..num_kernings {
            let mut buffer: [u8;12] = [0;12];
            file.read(&mut buffer).unwrap();
            let a = get_u32(&buffer[0..4]) as i32;
            let b = get_u32(&buffer[4..8]) as i32;
            let n = get_u32(&buffer[8..12]) as i32;
            kernings.push(Kerning {
                a: a,
                b: b,
                n: n,
            });
        }
        let mut pixels: Vec<u8> = vec![0; (scale_w * scale_h * 4) as usize];
        file.read(&mut pixels).unwrap();
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32,scale_w as i32,scale_h as i32,0,gl::RGBA,gl::UNSIGNED_BYTE,&pixels[0] as *const u8 as *const c_void);
            //gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Font {
            canvas: canvas,
            tex: tex,
            line_height: line_height as i32,
            base: base as i32,
            scale_w: scale_w as i32,
            scale_h: scale_h as i32,
            chars: chars,
            kernings: kernings,
        }
    }

    pub fn height(&self,sz: i32) -> f32 {
        let q = (sz as f32) / (self.base as f32);
        q * (self.line_height as f32)
    }

    pub fn advance(&self, text: &str,sz: i32) -> f32 {
        let mut prev_ch: Option<&Char> = None;
        let mut total: i32 = 0;
        for c in text.chars() {
            let ch = match self.chars.iter().find(|&x| x.id == c as i32) {
                Some(c) => { c },
                None => { continue; },
            };
            total += ch.xadvance;
            match prev_ch {
                Some(pch) => {
                    match self.kernings.iter().find(|&x| (x.a == pch.id) && (x.b == ch.id)) {
                        Some(k) => { total += k.n; },
                        None => { },
                    };
                },
                None => { },
            }
            prev_ch = Some(ch);
        }
        let q = (sz as f32) / (self.base as f32);
        q * (total as f32)
    }
}
