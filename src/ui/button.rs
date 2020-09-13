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
    state: Rc<ui::UIState>,
    pub r: Cell<Rect<i32>>,
    hit: Cell<ButtonHit>,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
    pub button_color: u32,
    pub hover_button_color: u32,
    pub padding: Vec2<i32>,
    pub inner_padding: Vec2<i32>,
}

impl Button {
    pub fn new(state: &Rc<ui::UIState>,text: &str,font: &Rc<ui::Font>) -> Button {
        Button {
            state: Rc::clone(state),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ButtonHit::Outside),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
            button_color: 0xFF000000,
            hover_button_color: 0xFF333300,
            padding: vec2!(0,0),
            inner_padding: vec2!(4,2),
        }
    }
}

impl ui::Widget for Button {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * (self.padding + self.inner_padding)
    }

    fn draw(&self,context: Vec2<i32>) {
        let bc = if let ButtonHit::Button = self.hit.get() {
            self.hover_button_color
        }
        else {
            self.button_color
        };
        self.state.draw_rectangle(rect!(context + self.padding,self.r.get().s - 2 * self.padding),bc,gpu::BlendMode::Replace);
        self.state.draw_text(context + self.padding + self.inner_padding,&self.text,self.color,&self.font);
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
        if rect!(self.padding,self.r.get().s - 2 * self.padding).contains(&p) {
            if let ButtonHit::Button = self.hit.get() {
            }
            else {
                self.hit.set(ButtonHit::Button);
                self.state.invalidate();
            }
            true
        }
        else {
            if let ButtonHit::Outside = self.hit.get() {                
            }
            else {
                self.hit.set(ButtonHit::Outside);
                self.state.invalidate();
            }
            false
        }
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
