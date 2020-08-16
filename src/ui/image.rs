// E - UI - Image
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Image widget.
pub struct Image {
    ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    tex: RefCell<Rc<ui::Texture2DSub<pixel::ARGB8>>>,
}

impl Image {

    /// Create new image widget.
    /// # Arguments
    /// * `ui` - UI context for this widget.
    /// * `tex` - Atlassed piece of texture to use.
    /// # Returns
    /// * `Ok(image)` - The image widget.
    /// * `Err(_)` - The image widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,tex: &Rc<ui::Texture2DSub<pixel::ARGB8>>) -> Result<Image,SystemError> {
        Ok(Image {
            ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            tex: RefCell::new(Rc::clone(tex)),
        })
    }
}

ui::impl_padding!(Image);

impl ui::Widget for Image {
    
    //fn draw(&self,dc: &Rc<ui::DC>,space: Rect<i32>) {
    //    dc.draw_texture(space.o + self.padding.get(),&self.tex.borrow());
    //}

    fn measure(&self) -> Vec2<i32> {
        let size = self.tex.borrow().r.s;
        vec2!(size.x as i32,size.y as i32)
    }

    fn handle(&self,event: &Event,_space: Rect<i32>) -> ui::HandleResult {
        match event {
            _ => { ui::HandleResult::Unhandled },
        }
    }

    fn build(&self,buffer: &mut Vec<ui::UIRect>,space: Rect<i32>) {
        let padding = self.padding.get();
        let tex = self.tex.borrow();
        buffer.push(ui::UIRect {
            r: vec4!(
                (space.o.x + padding.x) as f32,
                (space.o.y + padding.y) as f32,
                (space.s.x - 2 * padding.x) as f32,
                (space.s.y - 2 * padding.y) as f32
            ),
            t: vec4!(
                (tex.r.o.x as f32) / (self.ui.large_textures.array.size.x as f32),
                (tex.r.o.y as f32) / (self.ui.large_textures.array.size.y as f32),
                (tex.r.s.x as f32) / (self.ui.large_textures.array.size.x as f32),
                (tex.r.s.y as f32) / (self.ui.large_textures.array.size.y as f32)
            ),
            fbdq: vec4!(0xFFFFFFFF,0xFF000000,0,0x00030000),
        });        
    }
}
