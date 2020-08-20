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
    font: RefCell<Rc<ui::Font>>,
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
    pub fn new(ui: &Rc<ui::UI>,text: &str,font: &Rc<ui::Font>) -> Result<Text,SystemError> {
        Ok(Text {
            ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            text: RefCell::new(String::from(text)),
            font: RefCell::new(Rc::clone(font)),
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
        self.font.borrow().measure(&self.text.borrow()) + 2 * self.padding.get()
    }

    fn handle(&self,_event: &Event,_space: Rect<i32>) {
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let mut buffer: Vec<ui::UIRect> = Vec::new();
        let padding = self.padding.get();
        let color = u32::from(self.color.get());
        let back_color = u32::from(self.back_color.get());
        self.font.borrow().build_text(&mut buffer,space.o + padding,&self.text.borrow(),0.0,color,back_color);
        let vertexbuffer = gpu::VertexBuffer::new_from_vec(&self.ui.graphics,&buffer).expect("Unable to create vertexbuffer");
        self.ui.draw(canvas_size,&vertexbuffer,buffer.len());
    }
}
