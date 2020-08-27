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

    /// Rectangle.
    pub r: Cell<Rect<i32>>,

    /// Padding around the stack.
    pub padding: Cell<Vec2<i32>>,

    /// Horizontal alignment of the widgets.
    pub halign: Cell<ui::HAlignment>,

    /// The widgets.
    widgets: RefCell<Vec<Rc<dyn ui::Widget>>>,

    capturing_widget: RefCell<Option<Rc<dyn ui::Widget>>>,
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
            r: Cell::new(rect!(0,0,1,1)),
            padding: Cell::new(vec2!(0,0)),
            halign: Cell::new(ui::HAlignment::Left),
            widgets: RefCell::new(widgets),
            capturing_widget: RefCell::new(None),
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

    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let padding = self.padding.get();
        let halign = self.halign.get();
        let widgets = self.widgets.borrow();
        let mut oy = r.o.y;
        for widget in widgets.iter() {
            let size = widget.measure();
            let (ox,sx) = match halign {
                ui::HAlignment::Left => { (r.o.x,size.x) },
                ui::HAlignment::Right => { (r.o.x + r.s.x - size.x,size.x) },
                ui::HAlignment::Center => { (r.o.x + (r.s.x - size.x) / 2,size.x / 2) },
                ui::HAlignment::Fill => { (r.o.x,r.s.x) },
            };
            widget.set_rect(rect!(ox + padding.x,oy + padding.y,sx - 2 * padding.x,size.y - 2 * padding.y));
            oy += size.y;
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