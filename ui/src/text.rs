// E - UI - Text
// Desmond Germans, 2020

// Text is exactly that, text.

use crate::*;
use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

/// Text style.
pub struct TextStyle {
    pub font: Rc<Font>,
    pub color: u32,
    pub text_color: u32,
}

/// Text.
pub struct Text {
    ui: Rc<UI>,
    style: RefCell<TextStyle>,
    r: Cell<Rect<i32>>,
    text: String,
}

impl Text {
    pub fn new(ui: &Rc<UI>,text: &str) -> Result<Rc<Text>,SystemError> {
        Ok(Rc::new(Text {
            ui: Rc::clone(&ui),
            style: RefCell::new(TextStyle {
                font: Rc::clone(&ui.font),
                color: 0x444444,
                text_color: 0xAAAAAA,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            text: text.to_string(),
        }))
    }
}

impl Widget for Text {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        style.font.measure(self.text.as_str())
    }

    fn draw(&self) {
        let style = self.style.borrow();
        self.ui.draw_rectangle(rect!(vec2!(0,0),self.r.get().s),style.color,BlendMode::Replace);
        self.ui.draw_text(vec2!(0,0),&self.text,style.text_color,&style.font);
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}