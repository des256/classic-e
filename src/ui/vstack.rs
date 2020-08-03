// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Vertical stack widget.
pub struct VStack {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,
    ca: Cell<ui::HAlignment>,
}

impl VStack {
    pub fn new(ui: &Rc<ui::UI>,widgets: Vec<Rc<dyn ui::Widget>>) -> VStack {
        VStack {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            widgets: RefCell::new(widgets),
            ca: Cell::new(ui::HAlignment::Left),
        }
    }

    pub fn set_padding(&self,padding: Vec2<i32>) {
        self.padding.set(padding);
    }

    pub fn set_calign(&self,ca: ui::HAlignment) {
        self.ca.set(ca);
    }
}

impl ui::Widget for VStack {
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<i32>) {
        let mut oy = space.o.y;
        let padding = self.padding.get();
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure(dc);
            let (ox,sx) = match self.ca.get() {
                ui::HAlignment::Left => { (space.o.x,size.x) },
                ui::HAlignment::Right => { (space.o.x + space.s.x - size.x,size.x) },
                ui::HAlignment::Center => { (space.o.x + (space.s.x - size.x) / 2,size.x / 2) },
                ui::HAlignment::Fill => { (space.o.x,space.s.x) },
            };
            widget.draw(dc,rect!(ox + padding.x,oy + padding.y,sx - 2 * padding.x,size.y - 2 * padding.y));
            oy += size.y;
        }
    }

    fn measure(&self,dc: &Rc<ui::DC>) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure(dc);
            if size.x > total_size.x {
                total_size.x = size.x;
            }
            total_size.y += size.y;
        }
        total_size + 2 * self.padding.get()
    }
}
