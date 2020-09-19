// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Horizontal stack widget.
pub struct HStack {
    core: ui::Core<Box<dyn ui::Widget>>,
    pub padding: i32x2,
    pub valign: ui::VAlignment,
}

impl HStack {
    pub fn new_from_vec(state: &Rc<ui::UIState>,widgets: Vec<Box<dyn ui::Widget>>) -> HStack {
        HStack {
            core: ui::Core::new_from_vec(state,widgets),
            padding: i32x2::zero(),
            valign: ui::VAlignment::Top,
        }
    }
}

impl ui::Widget for HStack {
    fn get_rect(&self) -> i32r {
        self.core.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.core.r.set(r);
        let mut ox = 0;
        for child in self.core.children.iter() {
            let size = child.calc_min_size();
            let (oy,sy) = match self.valign {
                ui::VAlignment::Top => { (*r.o.y(),*size.y()) },
                ui::VAlignment::Bottom => { (*r.o.y() + *r.s.y() - *size.y(),*size.y()) },
                ui::VAlignment::Center => { (*r.o.y() + (*r.s.y() - *size.y()) / 2,*size.y() / 2) },
                ui::VAlignment::Fill => { (*r.o.y(),*r.s.y()) },
            };
            child.set_rect(i32r::from_os(
                i32x2::from_xy(ox,oy) + self.padding,
                i32x2::from_xy(*size.x(),sy) - 2 * self.padding
            ));
            ox += *size.x();
        }
    }

    fn calc_min_size(&self) -> i32x2 {
        let mut total_size = i32x2::zero();
        for child in self.core.children.iter() {
            let size = child.calc_min_size();
            *total_size.x() += *size.x();
            if *size.y() > *total_size.y() {
                *total_size.y() = *size.y();
            }
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
