// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Text widget.
pub struct Text {

    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Text rectangle.
    pub r: Cell<Rect<i32>>,

    /// Padding around the text.
    pub padding: Cell<Vec2<i32>>,

    /// The text.
    pub text: RefCell<String>,

    /// Font to use when drawing the text.
    pub font: RefCell<Rc<ui::Font>>,

    /// Color of the text.
    pub color: Cell<u32>,
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
            r: Cell::new(rect!(0,0,1,1)),
            padding: Cell::new(vec2!(0,0)),
            text: RefCell::new(String::from(text)),
            font: RefCell::new(Rc::clone(font)),
            color: Cell::new(0xFFFFFFFF),
        })
    }
}

impl ui::Widget for Text {
    
    fn measure(&self) -> Vec2<i32> {
        self.font.borrow().measure(&self.text.borrow()) + 2 * self.padding.get()
    }

    fn handle(&self,event: &Event) {
        match event {
            Event::Reconfigure(r) => {
                self.r.set(*r);
                // rebuild local vertices?
            }
        }
    }

    fn draw(&self,canvas_size: Vec2<i32>) {

        // draw the text
        let r = self.r.get();
        let padding = self.padding.get();
        let text = self.text.borrow();
        let font = self.font.borrow();
        let color = self.color.get();

        self.ui.draw_text(canvas_size,r.o + padding,&text,color,&font);
    }
}
