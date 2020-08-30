// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
};

/// Text widget.
pub struct Text {
    core: ui::Core,
    pub padding: Vec2<i32>,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
}

impl Text {
    pub fn new(ui: &Rc<ui::UI>,text: &str,font: &Rc<ui::Font>) -> Text {
        Text {
            core: ui::Core::new(ui),
            padding: vec2!(0,0),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
        }
    }
}

impl ui::Widget for Text {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r
    }

    fn set_rect(&mut self,r: Rect<i32>) {
        self.core.r = r;
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.o;
        self.core.ui.draw_text(local_context + self.padding,&self.text,self.color,&self.font);
    }

    fn handle_mouse_press(&mut self,_b: MouseButton) -> ui::MouseResult {
        ui::MouseResult::Unprocessed
    }

    fn handle_mouse_release(&mut self,_b: MouseButton) -> ui::MouseResult {
        ui::MouseResult::Unprocessed
    }

    fn handle_mouse_move(&mut self,_p: Vec2<i32>) -> ui::MouseResult {
        ui::MouseResult::Unprocessed
    }
}
