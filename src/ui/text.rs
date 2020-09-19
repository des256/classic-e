// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Text widget.
pub struct Text {
    core: ui::Core<Box<dyn ui::Widget>>,
    pub padding: i32x2,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
}

impl Text {
    pub fn new(state: &Rc<ui::UIState>,text: &str,font: &Rc<ui::Font>) -> Text {
        Text {
            core: ui::Core::new(state),
            padding: i32x2::zero(),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
        }
    }
}

impl ui::Widget for Text {
    fn get_rect(&self) -> i32r {
        self.core.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.core.r.set(r);
    }

    fn calc_min_size(&self) -> i32x2 {
        self.font.measure(&self.text) + 2 * self.padding
    }

    fn draw(&self,context: i32x2) {
        let local_context = context + self.core.r.get().o;
        self.core.state.draw_text(local_context + self.padding,&self.text,self.color,&self.font);
    }

    fn handle_mouse_press(&self,_p: i32x2,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: i32x2,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: i32x2) -> bool {
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
