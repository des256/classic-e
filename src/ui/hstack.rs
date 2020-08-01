// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Horizontal stack widget.
pub struct HStack {
    _ui: Rc<ui::UI>,
    padding: Cell<Vec2<i32>>,
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,
    ca: Cell<ui::VAlignment>,
}

impl HStack {
    /// Create new horizontal stack widget.
    /// ## Arguments
    /// * `ui` - UI context to create this horizontal stack widget for.
    /// * `widgets` - Widgets in the stack.
    /// ## Returns
    /// The horizontal stack widget.
    pub fn new(ui: &Rc<ui::UI>,widgets: Vec<Rc<dyn ui::Widget>>) -> HStack {
        HStack {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            widgets: RefCell::new(widgets),
            ca: Cell::new(ui::VAlignment::Top),
        }
    }

    /// Set widget padding.
    /// ## Arguments
    /// * `padding` - New padding specification.
    pub fn set_padding(&self,padding: Vec2<i32>) {
        self.padding.set(padding);
    }

    /// Set widget alignment.
    /// ## Arguments
    /// * `ca` - Vertical alignment of the widgets in the stack.
    pub fn set_calign(&self,ca: ui::VAlignment) {
        self.ca.set(ca);
    }
}

impl ui::Widget for HStack {
    fn draw(&self,dc: &Rc<ui::DC>,space: Rect<i32>) {
        let mut ox = space.o.x;
        let padding = self.padding.get();
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure();
            let (oy,sy) = match self.ca.get() {
                ui::VAlignment::Top => { (space.o.y,size.y) },
                ui::VAlignment::Bottom => { (space.o.y + space.s.y - size.y,size.y) },
                ui::VAlignment::Center => { (space.o.y + (space.s.y - size.y) / 2,size.y / 2) },
                ui::VAlignment::Fill => { (space.o.y,space.s.y) },
            };
            widget.draw(dc,rect!(ox + padding.x,oy + padding.y,size.x - 2 * padding.x,sy - 2 * padding.y));
            ox += size.x;
        }
    }

    fn measure(&self) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure();
            total_size.x += size.x;
            if size.y > total_size.y {
                total_size.y = size.y;
            }
        }
        total_size + 2 * self.padding.get()
    }
}