// E - UI - Book
// Desmond Germans, 2020

use {
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

/// Book.
pub struct Book {
    r: Cell<Rect<i32>>,
    children: Vec<Rc<dyn Widget>>,
}

impl Book {
    pub fn new(children: Vec<Rc<dyn Widget>>) -> Result<Book,SystemError> {
        Ok(Book {
            r: Cell::new(rect!(0,0,0,0)),
            children: children,
        })
    }
}

impl Widget for Book {
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