// E - UI - Page
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

pub(crate) const FONT: Vec2<f32> = Vec2 { x: 0.035,y: 0.035, };

#[doc(hidden)]
pub struct Character {
    pub(crate) n: u32,
    pub(crate) r: Rect<i32>,
    pub(crate) offset: Vec2<i32>,
    pub(crate) advance: i32,
}

#[doc(hidden)]
pub struct FontProto {
    pub name: String,
    pub scale: u32,
    pub characters: Vec<Character>,
    pub texture: gpu::Texture2D<pixel::ARGB8>,
}

impl FontProto {
    pub(crate) fn find(&self,c: char) -> Option<&Character> {
        let n = c as u32;
        for ch in &self.characters {
            if ch.n == n {
                return Some(ch);
            }
        }
        None
    }
}

/// Text font representation.
/// 
/// # File Format
/// The file starts with a header of the following 8 bytes:
/// `45 46 4E 84 30 30 31 00`, followed by the scale (u32) and the number of
/// characters (u32).
/// 
/// Then for each character, the following structure is stored:
/// ```
/// n: u32,
/// x0: i32,
/// y0: i32,
/// width: i32,
/// height: i32,
/// ofsx: i32,
/// ofsy: i32,
/// advance: i32,
/// ```
/// Here `n` is the unicode code point, `x0` and `y0` are the lower-left
/// coordinates of the character (as measured from the lower-left corner of
/// the atlas), `width` is the width of the character, `height` is the height
/// of the character, `ofsx`,`ofsy` are the offset of the character to the
/// base line, `advance` is the advance to the next character.
/// 
/// Finally, the file contains a BMP, PNG, etc. of the MSDF atlas.
pub struct Font {
    pub proto: Rc<FontProto>,
    pub size: Vec2<f32>,
    pub spacing: f32,
}

impl Font {
    pub fn new(proto: &Rc<FontProto>,size: Vec2<f32>,spacing: f32) -> Font {
        Font {
            proto: Rc::clone(proto),
            size: size,
            spacing: spacing,
        }
    }

    pub fn measure(&self,text: &str) -> (Vec2<f32>,Vec2<f32>) {
        let mut lp = 0f32;
        let mut min: Vec2<f32> = vec2!(0.0,0.0);
        let mut max: Vec2<f32> = vec2!(0.0,0.0);
        for c in text.chars() {
            if let Some(ch) = self.proto.find(c) {
                if (ch.r.s.x > 0) && (ch.r.s.y > 0) {
                    let sx = FONT.x * self.size.x * (ch.r.s.x as f32) / (self.proto.scale as f32);
                    let ox = lp - FONT.x * self.size.x * (ch.offset.x as f32) / (self.proto.scale as f32);
                    if ox < min.x {
                        min.x = ox;
                    }
                    if ox + sx > max.x {
                        max.x = ox + sx;
                    }
                }
                lp += FONT.x * self.size.x * (ch.advance as f32) / (self.proto.scale as f32) + FONT.x * self.size.x * self.spacing;
            }
        }

        // only measure the width of the string
        let xsize = max.x - min.x;
        let xofs = -min.x;

        // use standard height
        let ysize = 40.0 * FONT.y * self.size.y;
        let yofs = 0.75 * ysize;
        (vec2!(xsize,ysize),vec2!(xofs,yofs))
    }
}
