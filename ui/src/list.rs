// E - UI - List
// Desmond Germans, 2020

// A list is a vertical list of items. TBD.

use{
    crate::*,
    std::{
        cell::Cell,
    },
};

pub struct ListItem {
    
}

/// List.
pub struct List {
    r: Cell<Rect<i32>>,
    items: Vec<ListItem>,
}

impl List {
    pub fn new() -> Result<List,SystemError> {
        Ok(List {
            r: Cell::new(rect!(0,0,0,0)),
            items: Vec::new(),
        })
    }
}

impl Widget for List {
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
