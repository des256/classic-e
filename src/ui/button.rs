// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Button hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum ButtonHit {

    /// Mouse is somewhere else.
    Outside,

    /// Mouse is over the button.
    Button,
}

/// Button widget.
pub struct Button {
    
    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Rectangle.
    r: Cell<Rect<i32>>,

    /// Hit state.
    hit: Cell<ButtonHit>,

    /// Text on the button.
    pub text: RefCell<String>,

    /// Font to use for button text.
    pub font: RefCell<Rc<ui::Font>>,

    /// Color of the button text.
    pub color: Cell<u32>,

    /// Color of the button face.
    pub button_color: Cell<u32>,

    /// Color of the button face when the mouse hovers over it.
    pub hover_button_color: Cell<u32>,

    /// Padding around the button.
    pub padding: Cell<Vec2<i32>>,

    /// Padding around the text on the button.
    pub inner_padding: Cell<Vec2<i32>>,
}

impl Button {

    /// Create new button widget.
    /// ## Arguments
    /// * `ui` - UI context for this widget.
    /// * `text` - Text representation.
    /// ## Returns
    /// * `Ok(Button)` - The button widget.
    /// * `Err(SystemError)` - The button widget could not be created.
    pub fn new(ui: &Rc<ui::UI>,text: &str,font: &Rc<ui::Font>) -> Result<Button,SystemError> {
        Ok(Button {
            ui: Rc::clone(ui),
            r: Cell::new(rect!(0,0,1,1)),
            hit: Cell::new(ButtonHit::Outside),
            text: RefCell::new(String::from(text)),
            font: RefCell::new(Rc::clone(font)),
            color: Cell::new(0xFFFFFFFF),
            button_color: Cell::new(0xFF000000),
            hover_button_color: Cell::new(0xFF333300),
            padding: Cell::new(vec2!(0,0)),
            inner_padding: Cell::new(vec2!(4,2)),
        })
    }

    fn test_hit(&self,pos: Vec2<i32>) -> ButtonHit {
        let padding = self.padding.get();
        let r = rect!(padding,self.r.get().s - 2 * padding);
        let mut hit = ButtonHit::Outside;
        if r.contains(&pos) {
            hit = ButtonHit::Button;
        }
        hit
    }
}

impl ui::Widget for Button {

    fn measure(&self) -> Vec2<i32> {

        let font = self.font.borrow();
        let text = self.text.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        font.measure(&text) + 2 * (padding + inner_padding)
    }

    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn draw(&self) {

        let r = self.r.get();
        let hit = self.hit.get();
        let text = self.text.borrow();
        let font = self.font.borrow();
        let color = self.color.get();
        let button_color = if let ButtonHit::Button = hit {
            self.hover_button_color.get()
        } else {
            self.button_color.get()
        };
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        self.ui.draw_rectangle(rect!(r.o + padding,r.s - 2 * padding),button_color,gpu::BlendMode::Replace);
        self.ui.draw_text(r.o + padding + inner_padding,&text,color,&font);
    }

    fn mouse_press(&self,_pos: Vec2<i32>,button: Mouse) -> bool {
        let hit = self.hit.get();
        match button {
            Mouse::Left => {
                if let ButtonHit::Button = hit {
                    // call some sort of closure maybe
                }
            },

            _ => { },
        }
        if let ButtonHit::Button = hit {
            true
        }
        else {
            false
        }
    }

    fn mouse_move(&self,pos: Vec2<i32>) -> bool {
        let hit = self.test_hit(pos);
        self.hit.set(hit);
        if let ButtonHit::Button = hit {
            true
        }
        else {
            false
        }
    }
}
