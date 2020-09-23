// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Text widget.
pub struct Text {
    state: Rc<ui::UIState>,
    pub r: Cell<Rect<i32>>,
    pub padding: Vec2<i32>,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
}

impl Text {
    pub fn new(state: &Rc<ui::UIState>,text: &str,font: &Rc<ui::Font>) -> Text {
        Text {
            state: Rc::clone(state),
            r: Cell::new(Rect::<i32>::zero()),
            padding: Vec2::<i32>::zero(),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
        }
    }
}

impl ui::Widget for Text {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.r.get().o();
        self.state.draw_text(local_context + self.padding,&self.text,self.color,&self.font);
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) -> bool {
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
