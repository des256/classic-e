// E - UI - Page
// Desmond Germans, 2020

// A font manages the font texture atlas.

use crate::*;
use std::{
    rc::Rc,
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

/// Text font prototype.
#[doc(hidden)]
pub struct FontProto {
    pub(crate) _filename: String,
    pub(crate) sets: Vec<CharacterSet>,
    pub(crate) texture: Texture2D<pixel::R8>,
}

#[doc(hidden)]
impl FontProto {
    pub fn new(graphics: &Rc<Graphics>,filename: &str) -> Result<FontProto,SystemError> {
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
        let texture = Texture2D::new_from_mat(&graphics,mat)?;
        Ok(FontProto {
            _filename: filename.to_string(),
            sets: sets,
            texture: texture,
        })
    }
}

/// Text font.
pub struct Font {
    pub(crate) proto: Rc<FontProto>,
    pub(crate) font_size: u32,
    pub(crate) ratio: f32,
}

impl Font {
    pub fn new(proto: &Rc<FontProto>,font_size: u32) -> Result<Font,SystemError> {
        let mut actual_size = proto.sets[proto.sets.len() - 1].font_size;
        for s in proto.sets.iter() {
            if s.font_size >= font_size {
                actual_size = s.font_size;
                break;
            }
        }
        let ratio = (font_size as f32) / (actual_size as f32);
        Ok(Font {
            proto: Rc::clone(proto),
            font_size: actual_size,
            ratio: ratio,
        })
    }

    /// Measure string in this font.
    /// ## Arguments
    /// * `text` - String to measure.
    /// ## Returns
    /// The size of the string when rendered.
    pub fn measure(&self,text: &str) -> Vec2<i32> {
        let mut x = 0i32;
        let mut height = 0i32;
        for s in self.proto.sets.iter() {
            if s.font_size == self.font_size {
                for c in text.chars() {
                    let code = c as u32;
                    for ch in s.characters.iter() {
                        if ch.n == code {
                            x += (self.ratio * (ch.advance as f32)) as i32;
                            break;
                        }
                    }
                }
                height = (self.ratio * (s.height as f32)) as i32;
                break;
            }
        }
        vec2!(x,height)
    }
}