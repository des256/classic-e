// E - UI - Button
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Button hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum ButtonHit {
    Outside,
    Button,
}

/// Button widget.
pub struct Button {
    core: ui::Core,
    hit: ButtonHit,
    pub text: String,
    pub font: Rc<ui::Font>,
    pub color: u32,
    pub button_color: u32,
    pub hover_button_color: u32,
    pub padding: Vec2<i32>,
    pub inner_padding: Vec2<i32>,
}

impl Button {
    pub fn new(anchor: &Rc<ui::UIAnchor>,text: &str,font: &Rc<ui::Font>) -> Button {
        Button {
            core: ui::Core::new(anchor),
            hit: ButtonHit::Outside,
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
        self.core.r
    }

    fn set_rect(&mut self,r: Rect<i32>) {
        self.core.r = r;
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.font.measure(&self.text) + 2 * (self.padding + self.inner_padding)
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.o;
        let bc = if let ButtonHit::Button = self.hit {
            self.hover_button_color
        }
        else {
            self.button_color
        };
        self.core.anchor.draw_rectangle(rect!(local_context + self.padding,self.core.r.s - 2 * self.padding),bc,gpu::BlendMode::Replace);
        self.core.anchor.draw_text(local_context + self.padding + self.inner_padding,&self.text,self.color,&self.font);
    }

    fn handle_mouse_press(&mut self,b: MouseButton) -> ui::MouseResult {
        if let ButtonHit::Button = self.hit {
            if let MouseButton::Left = b {
                println!("Click!");
            }
            ui::MouseResult::ProcessedCapture
        }
        else {
            ui::MouseResult::Processed
        }
    }

    fn handle_mouse_release(&mut self,_b: MouseButton) -> ui::MouseResult {
        if let ButtonHit::Button = self.hit {
            ui::MouseResult::ProcessedCapture
        }
        else {
            ui::MouseResult::Processed
        }
    }

    fn handle_mouse_move(&mut self,p: Vec2<i32>) -> ui::MouseResult {
        if rect!(self.padding,self.core.r.s - 2 * self.padding).contains(&p) {
            self.hit = ButtonHit::Button;
            ui::MouseResult::ProcessedCapture
        }
        else {
            self.hit = ButtonHit::Outside;
            ui::MouseResult::Processed
        }
    }
}
