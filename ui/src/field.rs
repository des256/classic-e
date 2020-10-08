// E - UI - Field
// Desmond Germans, 2020

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
    },
};

/// Field.
pub struct Field {
    r: Cell<Rect<i32>>,
    text: RefCell<String>,
}

impl Field {
    pub fn new() -> Result<Field,SystemError> {
        Ok(Field {
            r: Cell::new(rect!(0,0,0,0)),
            text: RefCell::new(String::new()),
        })
    }
}

impl Widget for Field {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,_draw: &Draw) -> Vec2<i32> {
        vec2!(0,0)
    }

    fn draw(&self,_draw: &Draw) {
    }

    fn handle(&self,_ui: &UI,_window: &Window,_event: Event) {
    }
}
