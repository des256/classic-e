// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Horizontal stack widget.
pub struct HStack {
    core: ui::Core,
    pub padding: Vec2<i32>,
    pub valign: ui::VAlignment,
}

impl HStack {
    pub fn new_from_vec(anchor: &Rc<ui::UIAnchor>,widgets: Vec<Box<dyn ui::Widget>>) -> HStack {
        HStack {
            core: ui::Core::new_from_vec(anchor,widgets),
            padding: vec2!(0,0),
            valign: ui::VAlignment::Top,
        }
    }
}

impl ui::Widget for HStack {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r
    }

    fn set_rect(&mut self,r: Rect<i32>) {
        self.core.r = r;
        let mut ox = 0;
        for child in self.core.children.iter_mut() {
            let size = child.calc_min_size();
            let (oy,sy) = match self.valign {
                ui::VAlignment::Top => { (r.o.y,size.y) },
                ui::VAlignment::Bottom => { (r.o.y + r.s.y - size.y,size.y) },
                ui::VAlignment::Center => { (r.o.y + (r.s.y - size.y) / 2,size.y / 2) },
                ui::VAlignment::Fill => { (r.o.y,r.s.y) },
            };
            child.set_rect(rect!(
                ox + self.padding.x,
                oy + self.padding.y,
                size.x - 2 * self.padding.x,
                sy - 2 * self.padding.y
            ));
            ox += size.x;
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for child in self.core.children.iter() {
            let size = child.calc_min_size();
            total_size.x += size.x;
            if size.y > total_size.y {
                total_size.y = size.y;
            }
        }
        total_size + 2 * self.padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let local_context = context + self.core.r.o;
        for child in self.core.children.iter() {
            child.draw(local_context);
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
