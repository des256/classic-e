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
    core: ui::Core,
    hit: Cell<ButtonHit>,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
    pub button_color: u32,
    pub hover_button_color: u32,
    pub padding: Cell<Vec2<i32>>,  // everything cell again?
    pub inner_padding: Vec2<i32>,
}

impl Button {
    pub fn new(state: &Rc<ui::UIState>,text: &str,font: &Rc<ui::Font>) -> Button {
        Button {
            core: ui::Core::new(state),
            hit: Cell::new(ButtonHit::Outside),
            text: String::from(text),
            font: Rc::clone(font),
            color: 0xFFFFFFFF,
            button_color: 0xFF000000,
            hover_button_color: 0xFF333300,
            padding: Cell::new(vec2!(0,0)),
            inner_padding: vec2!(4,2),
        }
    }
}

impl ui::Widget for Button {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.core.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * (self.padding.get() + self.inner_padding)
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.get().o;
        let bc = if let ButtonHit::Button = self.hit.get() {
            self.hover_button_color
        }
        else {
            self.button_color
        };
        self.core.state.draw_rectangle(rect!(local_context + self.padding.get(),self.core.r.get().s - 2 * self.padding.get()),bc,gpu::BlendMode::Replace);
        self.core.state.draw_text(local_context + self.padding.get() + self.inner_padding,&self.text,self.color,&self.font);
    }

    fn handle_mouse_press(&self,b: MouseButton) -> ui::MouseResult {
        if let ButtonHit::Button = self.hit.get() {
            if let MouseButton::Left = b {
                println!("Click!");
            }
            ui::MouseResult::ProcessedCapture
        }
        else {
            ui::MouseResult::Processed
        }
    }

    fn handle_mouse_release(&self,_b: MouseButton) -> ui::MouseResult {
        if let ButtonHit::Button = self.hit.get() {
            ui::MouseResult::ProcessedCapture
        }
        else {
            ui::MouseResult::Processed
        }
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> ui::MouseResult {
        if rect!(self.padding.get(),self.core.r.get().s - 2 * self.padding.get()).contains(&p) {
            self.hit.set(ButtonHit::Button);
            ui::MouseResult::ProcessedCapture
        }
        else {
            self.hit.set(ButtonHit::Outside);
            ui::MouseResult::Processed
        }
    }
}
