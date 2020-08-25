// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use crate::ui::UIRectFunctions;
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
}

impl ui::Widget for Button {

    fn measure(&self) -> Vec2<i32> {

        let font = self.font.borrow();
        let text = self.text.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        font.measure(&text) + 2 * (padding + inner_padding)
    }

    fn handle(&self,event: &Event,space: Rect<i32>) {

        // hit test
        match event {
            Event::MousePress(pos,_) | Event::MouseRelease(pos,_) | Event::MouseMove(pos) => {
                let padding = self.padding.get();
                let mut rect = space;
                rect.o += padding;
                rect.s -= 2 * padding;
                let mut hit = ButtonHit::Outside;
                if rect.contains(&pos) {
                    hit = ButtonHit::Button;
                }
                self.hit.set(hit);
            },
            _ => { },
        }

        // handle events
        let hit = self.hit.get();
        match event {
            Event::MousePress(_,mouse) => {
                match mouse {
                    Mouse::Left => {
                        if let ButtonHit::Button = hit {
                            // TODO: button is pressed, so call some closure
                        }
                    },
                    _ => { },
                }
            },
            _ => { },
        }
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {

        // begin drawing series
        let mut buffer = self.ui.begin_drawing();

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

        buffer.push_rect(rect!(space.o + padding,space.s - 2 * padding),button_color);

        // TODO: draw text always in center of button
        buffer.push_text(space.o + padding + inner_padding,&text,&font,color,button_color);

        // end drawing series
        self.ui.end_drawing(canvas_size,buffer,gpu::BlendMode::Replace);
    }
}
