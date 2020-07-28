// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Horizontal stack widget.
pub struct HStack {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<f32>>,
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,
    ca: Cell<ui::VAlignment>,
}

impl HStack {
    pub fn new(ui: &Rc<ui::UI>,widgets: Vec<Rc<dyn ui::Widget>>) -> HStack {
        HStack {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0.0,0.0)),
            widgets: RefCell::new(widgets),
            ca: Cell::new(ui::VAlignment::Top),
        }
    }

    pub fn set_padding(&self,padding: Vec2<f32>) {
        self.padding.set(padding);
    }

    pub fn set_calign(&self,ca: ui::VAlignment) {
        self.ca.set(ca);
    }
}

impl ui::Widget for HStack {
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<f32>) {
        let mut ox = space.o.x;
        let padding = self.padding.get();
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure();
            let (oy,sy) = match self.ca.get() {
                ui::VAlignment::Top => { (space.o.y,size.y) },
                ui::VAlignment::Bottom => { (space.o.y + space.s.y - size.y,size.y) },
                ui::VAlignment::Center => { (space.o.y + 0.5 * (space.s.y - size.y),size.y) },
                ui::VAlignment::Fill => { (space.o.y,space.s.y) },
            };
            widget.draw(dc,rect!(ox + padding.x,oy + padding.y,size.x - 2.0 * padding.x,sy - 2.0 * padding.y));
            ox += size.x;
        }
    }

    fn measure(&self) -> Vec2<f32> {
        let mut total_size = vec2!(0.0f32,0.0f32);
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure();
            total_size.x += size.x;
            if size.y > total_size.y {
                total_size.y = size.y;
            }
        }
        total_size + 2.0 * self.padding.get()
    }
}