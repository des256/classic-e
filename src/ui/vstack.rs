// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Vertical stack widget.
pub struct VStack {
    
    /// Reference to UI context.
    _ui: Rc<ui::UI>,

    /// Padding around the stack.
    pub padding: Cell<Vec2<i32>>,

    /// Horizontal alignment of the widgets.
    pub halign: Cell<ui::HAlignment>,

    /// The widgets.
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,
}

impl VStack {
    /// Create new vertical stack from vec of widgets.
    /// ## Arguments
    /// * `ui` - UI context to create this vertical stack widget for.
    /// * `widgets` - Widgets in the stack.
    /// ## Returns
    /// The vertical stack widget.
    pub fn new_from_vec(ui: &Rc<ui::UI>,widgets: Vec<Rc<dyn ui::Widget>>) -> Result<VStack,SystemError> {
        Ok(VStack {
            _ui: Rc::clone(ui),
            padding: Cell::new(vec2!(0,0)),
            halign: Cell::new(ui::HAlignment::Left),
            widgets: RefCell::new(widgets),
        })
    }
}

impl ui::Widget for VStack {
    fn measure(&self) -> Vec2<i32> {
        let mut total_size = vec2!(0i32,0i32);
        for widget in self.widgets.borrow().iter() {
            let size = widget.measure();
            if size.x > total_size.x {
                total_size.x = size.x;
            }
            total_size.y += size.y;
        }
        total_size + 2 * self.padding.get()
    }

    fn handle(&self,_event: &Event,_space: Rect<i32>) {
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let mut oy = space.o.y;
        let padding = self.padding.get();
        let widgets = self.widgets.borrow();
        let halign = self.halign.get();
        for widget in widgets.iter() {
            let size = widget.measure();
            let (ox,sx) = match halign {
                ui::HAlignment::Left => { (space.o.x,size.x) },
                ui::HAlignment::Right => { (space.o.x + space.s.x - size.x,size.x) },
                ui::HAlignment::Center => { (space.o.x + (space.s.x - size.x) / 2,size.x / 2) },
                ui::HAlignment::Fill => { (space.o.x,space.s.x) },
            };
            widget.draw(canvas_size,rect!(ox + padding.x,oy + padding.y,sx - 2 * padding.x,size.y - 2 * padding.y));
            oy += size.y;
        }
    }
}