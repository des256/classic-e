// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Vertical stack widget.
pub struct VStack {
    ui: Rc<ui::UI>,
    engine: ui::WidgetEngine,
    widgets: RefCell<Vec<Box<dyn ui::Widget>>>,
    ca: Cell<ui::HAlignment>,
}

impl VStack {
    pub fn new(ui: &Rc<ui::UI>) -> VStack {
        VStack {
            ui: Rc::clone(ui),
            engine: ui::WidgetEngine::new(),
            widgets: RefCell::new(Vec::new()),
            ca: Cell::new(ui::HAlignment::Left),
        }
    }

    pub fn set_calign(&self,ca: ui::HAlignment) {
        self.ca.set(ca);
    }
}

impl ui::Widget for VStack {
    fn draw(&self,gc: &Rc<ui::GC>,space: Rect<f32>) {
        
    }

    fn measure(&self) -> Vec2<f32> {
        vec2!(0.0,0.0)
    }
}