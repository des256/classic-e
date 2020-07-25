// E - UI - Page
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

pub(crate) const FONT: Vec2<f32> = Vec2 { x: 0.065,y: 0.065, };  // manually found by comparing chrome and html font-size: 24 --> draw_text font size should be similar

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
                    // bottom-left of the character, in GU
                    let ox = lp - FONT.x * self.size.x * (ch.offset.x as f32) / (self.proto.scale as f32);
                    let oy = -FONT.y * self.size.y * (ch.offset.y as f32) / (self.proto.scale as f32);

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
                    lp += FONT.x * self.size.x * (ch.advance as f32) / (self.proto.scale as f32) + FONT.x * self.size.x * self.spacing;
                }
                else {
                    // only advance
                    lp += 2.0 * FONT.x * self.size.x * (ch.advance as f32) / (self.proto.scale as f32) + FONT.x * self.size.x * self.spacing;  // the choice for double spacing is arbitrary
                }
            }
        }
        (max - min,-min)
    }
}
