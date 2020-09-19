// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Vertical stack widget.
pub struct VStack {
    core: ui::Core<Box<dyn ui::Widget>>,
    pub padding: i32x2,
    pub halign: ui::HAlignment,
}

impl VStack {
    pub fn new_from_vec(state: &Rc<ui::UIState>,widgets: Vec<Box<dyn ui::Widget>>) -> VStack {
        VStack {
            core: ui::Core::new_from_vec(state,widgets),
            padding: i32x2::zero(),
            halign: ui::HAlignment::Left,
        }
    }
}

impl ui::Widget for VStack {
    fn get_rect(&self) -> i32r {
        self.core.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.core.r.set(r);
        let mut oy = 0;
        for child in self.core.children.iter() {
            let size = child.calc_min_size();
            let (ox,sx) = match self.halign {
                ui::HAlignment::Left => { (*r.o.x(),*size.x()) },
                ui::HAlignment::Right => { (*r.o.x() + *r.s.x() - *size.x(),*size.x()) },
                ui::HAlignment::Center => { (*r.o.x() + (*r.s.x() - *size.x()) / 2,*size.x() / 2) },
                ui::HAlignment::Fill => { (*r.o.x(),*r.s.x()) },
            };
            child.set_rect(i32r::from_os(
                i32x2::from_xy(ox,oy) + self.padding,
                i32x2::from_xy(sx,*size.y()) - 2 * self.padding,
            ));
            oy += *size.y();
        }
    }

    fn calc_min_size(&self) -> i32x2 {
        let mut total_size = i32x2::zero();
        for child in self.core.children.iter() {
            let size = child.calc_min_size();
            if *size.x() > *total_size.x() {
                *total_size.x() = *size.x();
            }
            *total_size.y() += *size.y();
        }
        total_size + 2 * self.padding
    }

    fn draw(&self,context: i32x2) {
        let local_context = context + self.core.r.get().o;
        for child in self.core.children.iter() {
            child.draw(local_context);
        }
    }

    fn handle_mouse_press(&self,p: i32x2,b: MouseButton) {
        if !self.core.capturing_mouse_press(p,b) {
            self.core.other_mouse_press(p,b);
        }
    }

    fn handle_mouse_release(&self,p: i32x2,b: MouseButton) {
        if !self.core.capturing_mouse_release(p,b) {
            self.core.other_mouse_release(p,b);
        }
    }

    fn handle_mouse_move(&self,p: i32x2) -> bool {
        if !self.core.capturing_mouse_move(p) {
            self.core.other_mouse_move(p)
        }
        else {
            true
        }
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}