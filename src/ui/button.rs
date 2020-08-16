// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Button widget.
pub struct Button {
    ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    inner_padding: Cell<Vec2<i32>>,
    text: RefCell<String>,
    text_color: Cell<pixel::ARGB8>,
    button_color: Cell<pixel::ARGB8>,
    hover_color: Cell<pixel::ARGB8>,
    press_color: Cell<pixel::ARGB8>,
    hovering: Cell<bool>,
    pressed: Cell<bool>,
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
            ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            inner_padding: Cell::new(vec2!(4,2)),
            text: RefCell::new(String::from(text)),
            text_color: Cell::new(pixel::ARGB8::from(0xFFFFFFFF)),
            button_color: Cell::new(pixel::ARGB8::from(0xFF000000)),
            hover_color: Cell::new(pixel::ARGB8::from(0xFF333300)),
            press_color: Cell::new(pixel::ARGB8::from(0xFF777700)),
            hovering: Cell::new(false),
            pressed: Cell::new(false),
        })
    }
}

ui::impl_padding!(Button);

impl ui::Widget for Button {

    fn measure(&self) -> Vec2<i32> {
        self.ui.font.measure(&self.text.borrow(),40) + 2 * (self.padding.get() + self.inner_padding.get())
    }

    fn handle(&self,event: &Event,space: Rect<i32>) -> ui::HandleResult {
        match event {
            Event::MousePress(pos,mouse) => {
                match mouse {
                    Mouse::Left => {
                        if (pos.x >= space.o.x) && (pos.y >= space.o.y) && (pos.x < space.o.x + space.s.x) && (pos.y < space.o.y + space.s.y) {
                            self.pressed.set(true);
                            ui::HandleResult::HandledRebuild
                        }
                        else {
                            ui::HandleResult::Unhandled
                        }
                    },
                    _ => { ui::HandleResult::Unhandled },
                }
            },
            Event::MouseRelease(_pos,mouse) => {
                match mouse {
                    Mouse::Left => {
                        self.pressed.set(false);
                        ui::HandleResult::HandledRebuild
                    },
                    _ => { ui::HandleResult::Unhandled },
                }
            },
            Event::MouseMove(pos) => {
                if (pos.x >= space.o.x) && (pos.y >= space.o.y) && (pos.x < space.o.x + space.s.x) && (pos.y < space.o.y + space.s.y) {
                    self.hovering.set(true);
                    ui::HandleResult::HandledRebuild
                }
                else {
                    self.hovering.set(false);
                    ui::HandleResult::HandledRebuild
                }
            },
            _ => { ui::HandleResult::Unhandled },
        }
    }

    fn build(&self,buffer: &mut Vec<ui::UIRect>,space: Rect<i32>) {
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let mut bgc = self.button_color.get();
        if self.pressed.get() {
            bgc = self.press_color.get();
        }
        else if self.hovering.get() {
            bgc = self.hover_color.get();
        }
        buffer.push(ui::UIRect {
            r: vec4!(
                (space.o.x + padding.x) as f32,
                (space.o.y + padding.y) as f32,
                (space.s.x - 2 * padding.x) as f32,
                (space.s.y - 2 * padding.y) as f32
            ),
            t: vec4!(0.0,0.0,0.0,0.0),
            fbdq: vec4!(u32::from(bgc),u32::from(bgc),0,0x00000000),
        });
        self.ui.font.build_text(buffer,space.o + padding + inner_padding,&self.text.borrow(),0.0,40,u32::from(self.text_color.get()),u32::from(bgc));
    }
}
