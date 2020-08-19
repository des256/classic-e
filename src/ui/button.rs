// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Button widget.
pub struct Button {
    ui: Rc<ui::UI>,
    text: RefCell<String>,
    font: RefCell<Rc<ui::Font>>,
    color: Cell<pixel::ARGB8>,
    back_color: Cell<pixel::ARGB8>,
    hover_back_color: Cell<pixel::ARGB8>,
    padding: Cell<Vec2<i32>>,
    inner_padding: Cell<Vec2<i32>>,
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
    pub fn new(ui: &Rc<ui::UI>,text: &str,font: &Rc<ui::Font>) -> Result<Button,SystemError> {
        Ok(Button {
            ui: Rc::clone(ui),
            text: RefCell::new(String::from(text)),
            font: RefCell::new(Rc::clone(font)),
            color: Cell::new(pixel::ARGB8::from(0xFFFFFFFF)),
            back_color: Cell::new(pixel::ARGB8::from(0xFF000000)),
            hover_back_color: Cell::new(pixel::ARGB8::from(0xFF333300)),
            padding: Cell::new(vec2!(0,0)),
            inner_padding: Cell::new(vec2!(4,2)),
            hovering: Cell::new(false),
            pressed: Cell::new(false),
        })
    }
}

ui::impl_textfont!(Button);
ui::impl_color!(Button);
ui::impl_back_color!(Button);
ui::impl_hover_back_color!(Button);
ui::impl_padding!(Button);
ui::impl_inner_padding!(Button);

impl ui::Widget for Button {

    fn measure(&self) -> Vec2<i32> {
        self.font.borrow().measure(&self.text.borrow()) + 2 * (self.padding.get() + self.inner_padding.get())
    }

    fn handle(&self,event: &Event,space: Rect<i32>) {
        match event {
            Event::MousePress(pos,mouse) => {
                match mouse {
                    Mouse::Left => {
                        if (pos.x >= space.o.x) && (pos.y >= space.o.y) && (pos.x < space.o.x + space.s.x) && (pos.y < space.o.y + space.s.y) {
                            self.pressed.set(true);
                        }
                    },
                    _ => { },
                }
            },
            Event::MouseRelease(_pos,mouse) => {
                match mouse {
                    Mouse::Left => {
                        self.pressed.set(false);
                    },
                    _ => { },
                }
            },
            Event::MouseMove(pos) => {
                if (pos.x >= space.o.x) && (pos.y >= space.o.y) && (pos.x < space.o.x + space.s.x) && (pos.y < space.o.y + space.s.y) {
                    self.hovering.set(true);
                }
                else {
                    self.hovering.set(false);
                }
            },
            _ => { },
        }
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let mut buffer: Vec<ui::UIRect> = Vec::new();
        let mut bgc = self.back_color.get();
        /*if self.pressed.get() {
            bgc = self.press_color.get();
        }
        else*/ if self.hovering.get() {
            bgc = self.hover_back_color.get();
        }
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
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
        self.font.borrow().build_text(&mut buffer,space.o + padding + inner_padding,&self.text.borrow(),0.0,u32::from(self.color.get()),u32::from(bgc));
        let vertexbuffer = gpu::VertexBuffer::new_from_vec(&self.ui.graphics,&buffer).expect("Unable to create vertexbuffer");
        self.ui.draw(canvas_size,&vertexbuffer,buffer.len());
    }
}
