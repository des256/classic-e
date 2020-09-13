// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Vertical stack widget.
pub struct VStack {
    state: Rc<ui::UIState>,
    r: Cell<Rect<i32>>,
    widgets: Vec<Box<dyn ui::Widget>>,
    pub padding: Vec2<i32>,
    pub halign: ui::HAlignment,
}

impl VStack {
    pub fn new_from_vec(state: &Rc<ui::UIState>,widgets: Vec<Box<dyn ui::Widget>>) -> VStack {
        VStack {
            state: Rc::clone(state),
            r: Cell::new(rect!(0,0,0,0)),
            widgets: widgets,
            padding: vec2!(0,0),
            halign: ui::HAlignment::Left,
        }
    }
}

impl ui::Widget for VStack {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let mut oy = 0;
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            let (ox,sx) = match self.halign {
                ui::HAlignment::Left => { (r.o.x,size.x) },
                ui::HAlignment::Right => { (r.o.x + r.s.x - size.x,size.x) },
                ui::HAlignment::Center => { (r.o.x + (r.s.x - size.x) / 2,size.x / 2) },
                ui::HAlignment::Fill => { (r.o.x,r.s.x) },
            };
            widget.set_rect(rect!(
                ox + self.padding.x,
                oy + self.padding.y,
                sx - 2 * self.padding.x,
                size.y - 2 * self.padding.y
            ));
            oy += size.y;
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            if size.x > total_size.x {
                total_size.x = size.x;
            }
            total_size.y += size.y;
        }
        total_size + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.r.get().o;
        for widget in self.widgets.iter() {
            widget.draw(local_context);
        }
    }

    fn handle_mouse_press(&self,p: Vec2<i32>,b: MouseButton) {
        /*if !self.core.capturing_mouse_press(p,b) {
            self.core.other_mouse_press(p,b);
        }*/
    }

    fn handle_mouse_release(&self,p: Vec2<i32>,b: MouseButton) {
        /*if !self.core.capturing_mouse_release(p,b) {
            self.core.other_mouse_release(p,b);
        }*/
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        /*if !self.core.capturing_mouse_move(p) {
            self.core.other_mouse_move(p)
        }
        else {
            true
        }*/
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}