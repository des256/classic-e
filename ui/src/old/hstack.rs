// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Horizontal stack widget.
pub struct HStack {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    pub widgets: Vec<Box<dyn Widget>>,
    pub valign: Cell<VAlignment>,
    pub padding: Vec2<i32>,
}

impl HStack {
    pub fn new_from_vec(ui: &Rc<UI>,widgets: Vec<Box<dyn Widget>>) -> Result<HStack,SystemError> {
        Ok(HStack {
            ui: Rc::clone(&ui),
            r: Cell::new(Rect::<i32>::zero()),
            widgets: widgets,
            valign: Cell::new(VAlignment::Top),
            padding: Vec2::<i32>::zero(),
        })
    }
}

impl Widget for HStack {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }
    
    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let mut ox = 0;
        for widget in self.widgets.iter() {
            let size = widget.calc_min_size();
            let (oy,sy) = match self.valign.get() {
                VAlignment::Top => { (r.oy(),size.y()) },
                VAlignment::Bottom => { (r.oy() + r.sy() - size.y(),size.y()) },
                VAlignment::Center => { (r.oy() + (r.sy() - size.y()) / 2,size.y() / 2) },
                VAlignment::Fill => { (r.oy(),r.sy()) },
            };
            widget.set_rect(
                rect!(
                    vec2!(ox,oy) + self.padding,
                    vec2!(size.x(),sy) - 2 * self.padding
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

    fn draw(&self) {
        for widget in self.widgets.iter() {
            self.ui.state.delta_offset(widget.rect().o());
            widget.draw();
            self.ui.state.delta_offset(-widget.rect().o());
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
