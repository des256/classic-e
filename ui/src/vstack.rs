// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Vertical stack widget.
pub struct VStack {
    pub r: Cell<Rect<i32>>,
    pub widgets: Vec<Box<dyn Widget>>,
    pub padding: Vec2<i32>,
    pub halign: HAlignment,
}

impl VStack {
    pub fn new_from_vec(_state: &Rc<UIState>,widgets: Vec<Box<dyn Widget>>) -> VStack {
        VStack {
            r: Cell::new(Rect::<i32>::zero()),
            widgets: widgets,
            padding: Vec2::<i32>::zero(),
            halign: HAlignment::Left,
        }
    }
}

impl Widget for VStack {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let mut oy = 0;
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            let (ox,sx) = match self.halign {
                HAlignment::Left => { (r.ox(),size.x()) },
                HAlignment::Right => { (r.ox() + r.sx() - size.x(),size.x()) },
                HAlignment::Center => { (r.ox() + (r.sx() - size.x()) / 2,size.x() / 2) },
                HAlignment::Fill => { (r.ox(),r.sx()) },
            };
            widget.set_rect(
                rect!(
                    vec2!(ox,oy) + self.padding,
                    vec2!(sx,size.y()) - 2 * self.padding
                )
            );
            oy += size.y();
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let mut total_size = Vec2::<i32>::zero();
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            if size.x() > total_size.x() {
                total_size.set_x(size.x());
            }
            total_size.set_y(total_size.y() + size.y());
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