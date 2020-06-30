// E - OpenGL - Font
// Desmond Germans, 2020

use crate::Graphics;
use crate::UIError;
use crate::Texture2D;
use crate::ARGB8;
use crate::usize_2;
use crate::Image;

pub struct Font {
    texture: Texture2D<ARGB8>,
}

impl Graphics {
    pub fn create_font(&self) -> Result<Font,UIError> {
        let image = Image::<ARGB8>::new(usize_2 { x: 256,y: 256, });
        let texture = self.create_texture2d::<ARGB8>(&image).expect("unable to create font texture");
        Ok(Font {
            texture: texture,
        })
    }
}
