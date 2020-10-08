// E - UI - List
// Desmond Germans, 2020

// A list is a vertical list of items. TBD.

use{
    crate::*,
    std::{
        cell::Cell,
    },
};

#[derive(Copy,Clone)]
pub enum ListHit {
    Nothing,
    Item(usize),
}

pub struct ListItem {
    
}

/// List.
pub struct List {
    r: Cell<Rect<i32>>,
    hit: Cell<ListHit>,
    items: Vec<ListItem>,
    // TBD current, or multiple currents
}

const DEFAULT_LIST_ITEMS: i32 = 3;

impl List {
    pub fn new() -> Result<List,SystemError> {
        Ok(List {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ListHit::Nothing),
            items: Vec::new(),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> ListHit {
        ListHit::Nothing  // Should be Item(i) once we know how Scroller works
    }
}

impl Widget for List {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        let size = styles.font.measure("Text Item");
        vec2!(size.x(),size.y() * DEFAULT_LIST_ITEMS)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw the list items
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}
