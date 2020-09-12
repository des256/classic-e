// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
};

/// Text widget.
pub struct Text {
    core: ui::Core<Box<dyn ui::Widget>>,
    pub padding: Vec2<i32>,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
}

impl Text {
    pub fn new(state: &Rc<ui::UIState>,text: &str,font: &Rc<ui::Font>) -> Text {
        Text {
            core: ui::Core::new(state),
            padding: vec2!(0,0),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
        }
    }
}

impl ui::Widget for Text {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.core.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.get().o;
        self.core.state.draw_text(local_context + self.padding,&self.text,self.color,&self.font);
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
