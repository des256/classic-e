// E - UI - Text
// Desmond Germans, 2020

// Text is exactly that, text.

use crate::*;
use std::cell::Cell;

/// Text.
pub struct Text {
    r: Cell<Rect<i32>>,
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Result<Text,SystemError> {
        Ok(Text {
            r: Cell::new(rect!(0,0,0,0)),
            text: text.to_string(),
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

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        styles.font.measure(self.text.as_str())
    }

    fn draw(&self,draw: &Draw) {
        let styles = draw.styles.borrow();
        draw.draw_text(vec2!(0,0),&self.text,styles.text_color,&styles.font);
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
    }
}