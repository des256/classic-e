// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

/// Vertical stack widget.
pub struct VStack {
    core: ui::Core,
    pub padding: Vec2<i32>,
    pub halign: ui::HAlignment,
}

impl VStack {
    pub fn new_from_vec(anchor: &Rc<ui::UIAnchor>,widgets: Vec<Rc<RefCell<dyn ui::Widget>>>) -> VStack {
        VStack {
            core: ui::Core::new_from_vec(anchor,widgets),
            padding: vec2!(0,0),
            halign: ui::HAlignment::Left,
        }
    }
}

impl ui::Widget for VStack {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r
    }

    fn set_rect(&mut self,r: Rect<i32>) {
        self.core.r = r;
        let mut oy = 0;
        for child in self.core.children.iter() {
            let size = child.borrow().calc_min_size();
            let (ox,sx) = match self.halign {
                ui::HAlignment::Left => { (r.o.x,size.x) },
                ui::HAlignment::Right => { (r.o.x + r.s.x - size.x,size.x) },
                ui::HAlignment::Center => { (r.o.x + (r.s.x - size.x) / 2,size.x / 2) },
                ui::HAlignment::Fill => { (r.o.x,r.s.x) },
            };
            child.borrow_mut().set_rect(rect!(
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
        for child in self.core.children.iter() {
            let size = child.borrow().calc_min_size();
            if size.x > total_size.x {
                total_size.x = size.x;
            }
            total_size.y += size.y;
        }
        total_size + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.o;
        for child in self.core.children.iter() {
            child.borrow().draw(local_context);
        }
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