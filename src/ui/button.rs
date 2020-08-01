// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Button widget.
pub struct Button {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    inner_padding: Cell<Vec2<i32>>,
    font: RefCell<Rc<ui::Font>>,
    text: RefCell<String>,
    text_color: Cell<Vec4<f32>>,
    button_color: Cell<Vec4<f32>>,
}

impl Button {

    /// Create new button widget.
    /// ## Arguments
    /// * `ui` - UI context for this widget.
    /// * `text` - Text representation.
    /// ## Returns
    /// * `Ok(Button)` - The button widget.
    /// * `Err(SystemError)` - The button widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,text: &str) -> Result<Button,SystemError> {
        Ok(Button {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            inner_padding: Cell::new(vec2!(4,2)),
            font: RefCell::new(ui.get_font("arialn14.fnt").expect("cannot load font")),
            text: RefCell::new(String::from(text)),
            text_color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
            button_color: Cell::new(vec4!(0.5,0.5,0.5,1.0)),
        })
    }

    /// Set padding of the button widget.
    /// # Arguments
    /// * `padding` - New padding.
    pub fn set_padding(&self,padding: Vec2<i32>) {
        self.padding.set(padding);
    }

    /// Set inner text padding of the button widget.
    /// # Arguments
    /// * `padding` - New padding.
    pub fn set_inner_padding(&self,padding: Vec2<i32>) {
        self.inner_padding.set(padding);
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
        self.text_color.set(Vec4::<f32>::from(color));
    }

    /// Set button color.
    /// # Arguments
    /// * `color` - New color for the text.
    pub fn set_button_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.button_color.set(Vec4::<f32>::from(color));
    }
}

impl ui::Widget for Button {
    
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<i32>) {
        dc.set_color(self.button_color.get());
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        dc.draw_rect(rect!(space.o + padding,space.s - 2 * padding));
        dc.set_color(self.text_color.get());
        dc.draw_text(space.o + padding + inner_padding,&self.text.borrow());
    }

    fn measure(&self) -> Vec2<i32> {
        self.font.borrow().measure(&self.text.borrow()) + 2 * (self.padding.get() + self.inner_padding.get())
    }
}
