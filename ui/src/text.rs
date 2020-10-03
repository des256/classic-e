// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Text widget.
pub struct Text {
    state: Rc<UIState>,
    r: Cell<Rect<i32>>,
    pub text: String,
    pub padding: Vec2<i32>,
}

impl Text {
    pub fn new(state: &Rc<UIState>,text: &str) -> Result<Text,SystemError> {
        Ok(Text {
            state: Rc::clone(state),
            r: Cell::new(Rect::<i32>::zero()),
            text: String::from(text),
            padding: Vec2::<i32>::zero(),
        })
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
        let styles = self.state.styles.borrow();
        styles.font.measure(&self.text) + 2 * self.padding
    }

    fn draw(&self) {
        let styles = self.state.styles.borrow();
        self.state.draw_text(self.padding,&self.text,styles.text_color,&styles.font);
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) {
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
