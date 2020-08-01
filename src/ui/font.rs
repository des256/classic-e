// E - UI - Page
// Desmond Germans, 2020

use crate::*;
use std::{
    //rc::Rc,
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
    pub(crate) filename: String,
    pub(crate) height: i32,
    pub(crate) y_bearing: i32,
    pub(crate) characters: Vec<Character>,
    pub(crate) texture: gpu::Texture2D<pixel::ARGB8>,
}

impl Font {
    #[doc(hidden)]
    pub fn new(ui: &ui::UI,filename: &str) -> Result<Font,SystemError> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => { return Err(SystemError::Generic); },
        };
        let mut buffer: Vec<u8> = Vec::new();
        if let Err(_) = file.read_to_end(&mut buffer) {
            return Err(SystemError::Generic);
        }
        let font_height = get_i32(&buffer[8..]);
        let font_y_bearing = get_i32(&buffer[12..]);
        let texture_size_x = get_u32(&buffer[16..]);
        let texture_size_y = get_u32(&buffer[20..]);
        let count = get_u32(&buffer[24..]);
        let mut characters: Vec<ui::Character> = Vec::new();
        let bref = &buffer[28..];
        for i in 0..count {
            let n = get_u32(&bref[(i * 32) as usize..]);
            let rox = get_i32(&bref[(i * 32 + 4) as usize..]);
            let roy = get_i32(&bref[(i * 32 + 8) as usize..]);
            let rsx = get_i32(&bref[(i * 32 + 12) as usize..]);
            let rsy = get_i32(&bref[(i * 32 + 16) as usize..]);
            let bx = get_i32(&bref[(i * 32 + 20) as usize..]);
            let by = get_i32(&bref[(i * 32 + 24) as usize..]);
            let a = get_i32(&bref[(i * 32 + 28) as usize..]);
            characters.push(ui::Character {
                n: n,
                r: rect!(rox,roy,rsx,rsy),
                bearing: vec2!(bx,by),
                advance: a,
            });
        }
        let bref = &buffer[(28 + (count as usize) * 32)..];
        let mut mat = Mat::<pixel::ARGB8>::new(vec2!(texture_size_x as usize,texture_size_y as usize));
        for y in 0..texture_size_y as usize {
            for x in 0..texture_size_x as usize {
                let b = bref[y * (texture_size_x as usize) + x] as u32;
                let d = (b << 24) | (b << 16);
                mat.set(vec2!(x,y),pixel::ARGB8::from(d));
            }
        }
        Ok(Font {
            filename: filename.to_string(),
            height: font_height,
            y_bearing: font_y_bearing,
            characters: characters,
            texture: gpu::Texture2D::<pixel::ARGB8>::new_from_mat(&ui.graphics,&mat).expect("Unable to create font texture."),
        })
    }

    pub(crate) fn find<'a>(&'a self,c: char) -> Option<&'a Character> {
        let code = c as u32;
        for ch in self.characters.iter() {
            if ch.n == code {
                return Some(ch);
            }
        }
        None
    }

    /// Measure string in this font.
    /// ## Arguments
    /// * `text` - String to measure.
    /// ## Returns
    /// `Vec2<u16>` - The size of the string when rendered.
    pub fn measure(&self,text: &str) -> Vec2<i32> {
        let mut x = 0i32;
        for c in text.chars() {
            if let Some(ch) = self.find(c) {
                x += ch.advance;
            }
        }
        vec2!(x,self.height)
    }
}
