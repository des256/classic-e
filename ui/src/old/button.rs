// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Button hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum ButtonHit {
    Outside,
    Button,
}

/// Button widget.
pub struct Button {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    hit: Cell<ButtonHit>,
    pub text: String,
    pub padding: Vec2<i32>,
    pub inner_padding: Vec2<i32>,
}

impl Button {
    pub fn new(ui: &Rc<UI>,text: &str) -> Result<Button,SystemError> {
        Ok(Button {
            ui: Rc::clone(ui),
            r: Cell::new(Rect::<i32>::zero()),
            hit: Cell::new(ButtonHit::Outside),
            text: String::from(text),
            padding: Vec2::<i32>::zero(),
            inner_padding: vec2!(4,2),
        })
    }
}

impl Widget for Button {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }
    
    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let styles = self.ui.state.styles.borrow();
        styles.font.measure(&self.text) + 2 * (self.padding + self.inner_padding)
    }

    fn draw(&self) {
        let styles = self.ui.state.styles.borrow();
        let bc = if let ButtonHit::Button = self.hit.get() {
            styles.button_hover_color
        }
        else {
            styles.button_color
        };
        self.ui.state.draw_rectangle(rect!(self.padding,self.r.get().s() - 2 * self.padding),bc,BlendMode::Replace);
        self.ui.state.draw_text(self.padding + self.inner_padding,&self.text,styles.button_text_color,&styles.font);
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,b: MouseButton) {
        if let ButtonHit::Button = self.hit.get() {
            if let MouseButton::Left = b {
                println!("Click!");
            }
        }
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        if Rect::<i32>::new_os(self.padding,self.r.get().s() - 2 * self.padding).contains(&p) {
            self.hit.set(ButtonHit::Button);
            true
        }
        else {
            if let ButtonHit::Outside = self.hit.get() {
            }
            else {
                self.hit.set(ButtonHit::Outside);
            }
            false
        }
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
