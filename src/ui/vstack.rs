// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

pub struct VStack {
    ui: Rc<UI>,
    engine: WidgetEngine,
    widgets: RefCell<Vec<Box<dyn Widget>>>,
    ca: Cell<HAlignment>,
}

impl VStack {
    pub fn new(ui: &Rc<UI>) -> VStack {
        VStack {
            ui: Rc::clone(ui),
            engine: WidgetEngine::new(),
            widgets: RefCell::new(Vec::new()),
            ca: Cell::new(HAlignment::Left),
        }
    }

    pub fn set_calign(&self,ca: HAlignment) {
        self.ca.set(ca);
    }
}

impl Widget for VStack {
    fn draw(&self,gc: &Rc<GC>,space: Rect<f32>) {
        
    }

    fn measure(&self) -> Vec2<f32> {
        vec2!(0.0,0.0)
    }
}