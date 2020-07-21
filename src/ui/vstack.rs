// E - UI - VStack
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

pub struct VStack<'a> {
    ui: &'a UI<'a>,
    engine: WidgetEngine,
    widgets: RefCell<Vec<Box<dyn Widget>>>,
    ca: Cell<HAlignment>,
}

impl<'a> VStack<'a> {
    pub fn new(ui: &'a UI<'a>) -> VStack<'a> {
        VStack {
            ui: ui,
            engine: WidgetEngine::new(),
            widgets: RefCell::new(Vec::new()),
            ca: Cell::new(HAlignment::Left),
        }
    }

    pub fn set_calign(&self,ca: HAlignment) {
        self.ca.set(ca);
    }
}

impl<'a> Widget for VStack<'a> {
    fn draw(&self,gc: &GC,space: Rect<f32>) {
        
    }

    fn measure(&self) -> Vec2<f32> {
        vec2!(0.0,0.0)
    }
}