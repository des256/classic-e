// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Text widget.
pub struct Text {
    ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    text: RefCell<String>,
    font_size: Cell<u32>,
    color: Cell<pixel::ARGB8>,
    back_color: Cell<pixel::ARGB8>,
}

impl Text {

    /// Create new text widget.
    /// # Arguments
    /// * `ui` - UI context for this widget.
    /// * `text` - Text representation.
    /// # Returns
    /// * `Ok(text)` - The text widget.
    /// * `Err(_)` - The text widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,text: &str,font_size: u32) -> Result<Text,SystemError> {
        Ok(Text {
            ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            text: RefCell::new(String::from(text)),
            font_size: Cell::new(font_size),
            color: Cell::new(pixel::ARGB8::from(0xFFFFFFFF)),
            back_color: Cell::new(pixel::ARGB8::from(0xFF001133)),
        })
    }
}

ui::impl_color!(Text);
ui::impl_back_color!(Text);
ui::impl_padding!(Text);

impl ui::Widget for Text {
    
    fn measure(&self) -> Vec2<i32> {
        self.ui.font.measure(&self.text.borrow(),self.font_size.get()) + 2 * self.padding.get()
    }

    fn handle(&self,event: &Event,_space: Rect<i32>) -> ui::HandleResult {
        match event {
            _ => { ui::HandleResult::Unhandled },
        }
    }

    fn build(&self,buffer: &mut Vec<ui::UIRect>,space: Rect<i32>) {
        let padding = self.padding.get();
        let color = u32::from(self.color.get());
        let back_color = u32::from(self.back_color.get());
        let font_size = self.font_size.get();
        self.ui.font.build_text(buffer,space.o + padding,&self.text.borrow(),0.0,font_size,color,back_color);
    }
}
