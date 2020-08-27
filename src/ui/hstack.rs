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

    /// Rectangle.
    pub r: Cell<Rect<i32>>,

    /// Padding around the stack.
    pub padding: Cell<Vec2<i32>>,

    /// Vertical alignment of the widgets.
    pub valign: Cell<ui::VAlignment>,

    /// The widgets.
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,

    capturing_widget: RefCell<Option<Rc<dyn ui::Widget>>>,
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
            r: Cell::new(rect!(0,0,1,1)),
            padding: Cell::new(vec2!(0,0)),
            valign: Cell::new(ui::VAlignment::Top),
            widgets: RefCell::new(widgets),
            capturing_widget: RefCell::new(None),
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

    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let padding = self.padding.get();
        let valign = self.valign.get();
        let widgets = self.widgets.borrow();
        let mut ox = r.o.x;
        for widget in widgets.iter() {
            let size = widget.measure();
            let (oy,sy) = match valign {
                ui::VAlignment::Top => { (r.o.y,size.y) },
                ui::VAlignment::Bottom => { (r.o.y + r.s.y - size.y,size.y) },
                ui::VAlignment::Center => { (r.o.y + (r.s.y - size.y) / 2,size.y / 2) },
                ui::VAlignment::Fill => { (r.o.y,r.s.y) },
            };
            widget.set_rect(rect!(ox + padding.x,oy + padding.y,size.x - 2 * padding.x,sy - 2 * padding.y));
            ox += size.x;
        }
    }

    fn draw(&self) {
        let widgets = self.widgets.borrow();
        for widget in widgets.iter() {
            widget.draw();
        }
    }

    fn mouse_press(&self,pos: Vec2<i32>,button: Mouse) -> bool {
        if let Some(widget) = &*self.capturing_widget.borrow() {
            let r = widget.get_rect();
            if widget.mouse_press(pos - r.o,button) {
                return true;
            }
            *self.capturing_widget.borrow_mut() = None;
        }
        let widgets = self.widgets.borrow();
        for widget in widgets.iter() {
            let r = widget.get_rect();
            if r.contains(&pos) {
                if widget.mouse_press(pos - r.o,button) {
                    *self.capturing_widget.borrow_mut() = Some(Rc::clone(widget));
                    return true;
                }
                return false;
            }
        }
        false
    }

    fn mouse_release(&self,pos: Vec2<i32>,button: Mouse) -> bool {
        if let Some(widget) = &*self.capturing_widget.borrow() {
            let r = widget.get_rect();
            if widget.mouse_release(pos - r.o,button) {
                return true;
            }
            *self.capturing_widget.borrow_mut() = None;
        }
        let widgets = self.widgets.borrow();
        for widget in widgets.iter() {
            let r = widget.get_rect();
            if r.contains(&pos) {
                if widget.mouse_release(pos - r.o,button) {
                    *self.capturing_widget.borrow_mut() = Some(Rc::clone(widget));
                    return true;
                }
                return false;
            }
        }
        false
    }

    fn mouse_move(&self,pos: Vec2<i32>) -> bool {
        if let Some(widget) = &*self.capturing_widget.borrow() {
            let r = widget.get_rect();
            if widget.mouse_move(pos - r.o) {
                return true;
            }
            *self.capturing_widget.borrow_mut() = None;
        }
        let widgets = self.widgets.borrow();
        for widget in widgets.iter() {
            let r = widget.get_rect();
            if r.contains(&pos) {
                if widget.mouse_move(pos - r.o) {
                    *self.capturing_widget.borrow_mut() = Some(Rc::clone(widget));
                    return true;
                }
                return false;
            }
        }
        false
    }
}
