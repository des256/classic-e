// E - UI - Page
// Desmond Germans, 2020

use crate::*;
use std::{
    //rc::Rc,
    //cell::RefCell,
    fs::File,
    io::prelude::*,
};

#[doc(hidden)]
pub struct Character {
    pub(crate) n: u32,
    pub(crate) r: Rect<i32>,
    pub(crate) bearing: Vec2<i32>,
    pub(crate) advance: i32,
}

#[doc(hidden)]
pub struct CharacterSet {
    pub(crate) font_size: u32,
    pub(crate) height: i32,
    pub(crate) y_bearing: u32,
    pub(crate) characters: Vec<Character>,
}

/*fn get_u16(buffer: &[u8]) -> u16 {
    (buffer[0] as u16) | ((buffer[1] as u16) << 8)
}

fn get_i16(buffer: &[u8]) -> i16 {
    get_u16(buffer) as i16
}*/

fn get_u32(buffer: &[u8]) -> u32 {
    (buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32) << 16) | ((buffer[3] as u32) << 24)
}

fn get_i32(buffer: &[u8]) -> i32 {
    get_u32(buffer) as i32
}

/// Text font representation.
pub struct Font {
    pub(crate) _filename: String,
    pub(crate) sets: Vec<CharacterSet>,
    pub(crate) mat: Mat<pixel::R8>,
}

impl Font {
    #[doc(hidden)]
    pub fn new(filename: &str) -> Result<Font,SystemError> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => { return Err(SystemError::Generic); },
        };
        let mut buffer: Vec<u8> = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Err(SystemError::Generic);
        }
        let mut bref = &buffer[8..];
        let atlas_size_x = get_u32(bref); bref = &bref[4..];
        let atlas_size_y = get_u32(bref); bref = &bref[4..];
        let num_sets = get_u32(bref); bref = &bref[4..];
        let mut sets: Vec<CharacterSet> = Vec::new();
        for _i in 0..num_sets {
            let font_size = get_u32(bref); bref = &bref[4..];
            let height = get_i32(bref); bref = &bref[4..];
            let y_bearing = get_u32(bref); bref = &bref[4..];
            let num_chars = get_u32(bref); bref = &bref[4..];
            let mut characters: Vec<Character> = Vec::new();
            for _k in 0..num_chars {
                let n = get_u32(bref); bref = &bref[4..];
                let ox = get_i32(bref); bref = &bref[4..];
                let oy = get_i32(bref); bref = &bref[4..];
                let sx = get_i32(bref); bref = &bref[4..];
                let sy = get_i32(bref); bref = &bref[4..];
                let bx = get_i32(bref); bref = &bref[4..];
                let by = get_i32(bref); bref = &bref[4..];
                let a = get_i32(bref); bref = &bref[4..];
                characters.push(Character {
                    n: n,
                    r: rect!(ox,oy,sx,sy),
                    bearing: vec2!(bx,by),
                    advance: a,
                });
            }
            sets.push(CharacterSet {
                font_size: font_size,
                height: height,
                y_bearing: y_bearing,
                characters: characters,
            });
        }
        let mut mat = Mat::<pixel::R8>::new(vec2!(atlas_size_x as usize,atlas_size_y as usize));
        for y in 0..atlas_size_y as usize {
            for x in 0..atlas_size_x as usize {
                mat.set(vec2!(x,y),pixel::R8 { d: bref[y * (atlas_size_x as usize) + x] });
            }
        }
        Ok(Font {
            _filename: filename.to_string(),
            sets: sets,
            mat: mat,
        })
    }

    /// Measure string in this font.
    /// ## Arguments
    /// * `text` - String to measure.
    /// ## Returns
    /// `Vec2<u16>` - The size of the string when rendered.
    pub fn measure(&self,text: &str,font_size: u32) -> Vec2<i32> {
        let mut x = 0i32;
        let mut height = 0i32;
        let mut actual_size = self.sets[self.sets.len() - 1].font_size;
        for s in self.sets.iter() {
            if s.font_size >= font_size {
                actual_size = s.font_size;
                break;
            }
        }
        let ratio = (font_size as f32) / (actual_size as f32);
        for s in self.sets.iter() {
            if s.font_size == actual_size {
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            x += (ratio * (ch.advance as f32)) as i32;
                            break;
                        }
                    }
                }
                height = (ratio * (s.height as f32)) as i32;
                break;
            }
        }
        vec2!(x,height)
    }

    /// Add text onto vertexbuffer.
    /// ## Arguments
    /// * `uirects` - Vector containing the UIRects for the vertexbuffer.
    /// * `p` - Position of the text.
    /// * `text` - Text to add.
    /// * `d` - Depth of the text.
    /// * `color` - Main color.
    /// * `back_color` - Background color.
    pub fn build_text<T: ColorParameter>(&self,uirects: &mut Vec<ui::UIRect>,p: Vec2<i32>,text: &str,_d: f32,font_size: u32,color: T,back_color: T) where u32: From<T> {
        let color = u32::from(color);
        let back_color = u32::from(back_color);
        let mut actual_size = self.sets[self.sets.len() - 1].font_size;
        for s in self.sets.iter() {
            if s.font_size >= font_size {
                actual_size = s.font_size;
                break;
            }
        }
        let ratio = (font_size as f32) / (actual_size as f32);
        for s in self.sets.iter() {
            if s.font_size == actual_size {
                let mut v = vec2!(p.x,p.y + (ratio * (s.y_bearing as f32)) as i32);
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            uirects.push(ui::UIRect {
                                r: vec4!(
                                    (v.x + (ratio * (ch.bearing.x as f32)) as i32) as f32,
                                    (v.y - (ratio * (ch.bearing.y as f32)) as i32) as f32,
                                    ((ratio * (ch.r.s.x as f32)) as i32) as f32,
                                    ((ratio * (ch.r.s.y as f32)) as i32) as f32
                                ),
                                t: vec4!(
                                    (ch.r.o.x as f32) / (self.mat.size.x as f32),
                                    (ch.r.o.y as f32) / (self.mat.size.y as f32),
                                    (ch.r.s.x as f32) / (self.mat.size.x as f32),
                                    (ch.r.s.y as f32) / (self.mat.size.y as f32)
                                ),
                                fbdq: vec4!(color,back_color,0,0x00000000),
                            });
                            v.x += (ratio * (ch.advance as f32)) as i32;
                            break;
                        }
                    }
                }
                break;
            }
        }
    }
}
