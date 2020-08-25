// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Horizontal stack widget.
pub struct HStack {

    /// Reference to UI context.
    _ui: Rc<ui::UI>,

    /// Padding around the stack.
    pub padding: Cell<Vec2<i32>>,

    /// Vertical alignment of the widgets.
    pub valign: Cell<ui::VAlignment>,

    /// The widgets.
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,
}

impl HStack {
    /// Create new horizontal stack from vec of widgets.
    /// ## Arguments
    /// * `ui` - UI context to create this horizontal stack widget for.
    /// * `widgets` - Widgets in the stack.
    /// ## Returns
    /// The horizontal stack widget.
    pub fn new_from_vec(ui: &Rc<ui::UI>,widgets: Vec<Rc<dyn ui::Widget>>) -> Result<HStack,SystemError> {
        Ok(HStack {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            valign: Cell::new(ui::VAlignment::Top),
            widgets: RefCell::new(widgets),
        })
    }
}

impl ui::Widget for HStack {
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

    fn handle(&self,_event: &Event,_space: Rect<i32>) {
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let mut ox = space.o.x;
        let padding = self.padding.get();
        let widgets = self.widgets.borrow();
        let valign = self.valign.get();
        for widget in widgets.iter() {
            let size = widget.measure();
            let (oy,sy) = match valign {
                ui::VAlignment::Top => { (space.o.y,size.y) },
                ui::VAlignment::Bottom => { (space.o.y + space.s.y - size.y,size.y) },
                ui::VAlignment::Center => { (space.o.y + (space.s.y - size.y) / 2,size.y / 2) },
                ui::VAlignment::Fill => { (space.o.y,space.s.y) },
            };
            widget.draw(canvas_size,rect!(ox + padding.x,oy + padding.y,size.x - 2 * padding.x,sy - 2 * padding.y));
            ox += size.x;
        }
    }
}
