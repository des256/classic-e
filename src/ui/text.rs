// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Text widget.
pub struct Text {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    font: RefCell<Rc<ui::Font>>,
    text: RefCell<String>,
    color: Cell<Vec4<f32>>,
}

impl Text {

    /// Create new text widget.
    /// # Arguments
    /// * `ui` - UI context for this widget.
    /// * `text` - Text representation.
    /// # Returns
    /// * `Ok(text)` - The text widget.
    /// * `Err(_)` - The text widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,text: &str) -> Result<Text,SystemError> {
        Ok(Text {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            font: RefCell::new(ui.get_font("arialn14.fnt").expect("cannot load font")),
            text: RefCell::new(String::from(text)),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
        })
    }

    /// Set padding of the text widget.
    /// # Arguments
    /// * `padding` - New padding.
    pub fn set_padding(&self,padding: Vec2<i32>) {
        self.padding.set(padding);
    }

    /// Set font of the text widget.
    /// # Arguments
    /// * `font` - New font to use.
    pub fn set_font(&self,font: Rc<ui::Font>) {
        *(self.font.borrow_mut()) = font;
    }

    /// Set text color.
    /// # Arguments
    /// * `color` - New color for the text.
    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }
}

impl ui::Widget for Text {
    
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<i32>) {
        dc.set_color(self.color.get());
        dc.draw_text(space.o + self.padding.get(),&self.text.borrow());
    }

    fn measure(&self) -> Vec2<i32> {
        self.font.borrow().measure(&self.text.borrow()) + 2 * self.padding.get()
    }
}
