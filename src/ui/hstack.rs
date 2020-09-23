// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Horizontal stack widget.
pub struct HStack {
    pub r: Cell<Rect<i32>>,
    pub widgets: Vec<Box<dyn ui::Widget>>,
    pub padding: Vec2<i32>,
    pub valign: ui::VAlignment,
}

impl HStack {
    pub fn new_from_vec(_state: &Rc<ui::UIState>,widgets: Vec<Box<dyn ui::Widget>>) -> HStack {
        HStack {
            r: Cell::new(Rect::<i32>::zero()),
            widgets: widgets,
            padding: Vec2::<i32>::zero(),
            valign: ui::VAlignment::Top,
        }
    }
}

impl ui::Widget for HStack {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let mut ox = 0;
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            let (oy,sy) = match self.valign {
                ui::VAlignment::Top => { (r.oy(),size.y()) },
                ui::VAlignment::Bottom => { (r.oy() + r.sy() - size.y(),size.y()) },
                ui::VAlignment::Center => { (r.oy() + (r.sy() - size.y()) / 2,size.y() / 2) },
                ui::VAlignment::Fill => { (r.oy(),r.sy()) },
            };
            widget.set_rect(
                rect!(i32:
                    vec2!(i32: ox,oy) + self.padding,
                    vec2!(i32: size.x(),sy) - 2 * self.padding
                )
            );
            ox += size.x();
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let mut total_size = Vec2::<i32>::zero();
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            total_size.set_x(total_size.x() + size.x());
            if size.y() > total_size.y() {
                total_size.set_y(size.y());
            }
        }
        total_size + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.r.get().o();
        for widget in self.widgets.iter() {
            widget.draw(local_context);
        }
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
        /*if !self.core.capturing_mouse_press(p,b) {
            self.core.other_mouse_press(p,b);
        }*/
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
        /*if !self.core.capturing_mouse_release(p,b) {
            self.core.other_mouse_release(p,b);
        }*/
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) -> bool {
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
