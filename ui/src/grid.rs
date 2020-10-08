// E - UI - Grid
// Desmond Germans, 2020

// A grid is an organized rectangle of widgets.

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
    },
};

pub struct RowSpec {

}

pub struct ColumnSpec {
    
}

/// Grid.
pub struct Grid {
    r: Cell<Rect<i32>>,
    rows: RefCell<Vec<RowSpec>>,
    columns: RefCell<Vec<ColumnSpec>>,
}

impl Grid {
    pub fn new() -> Result<Grid,SystemError> {
        Ok(Grid {
            r: Cell::new(rect!(0,0,0,0)),
            rows: RefCell::new(Vec::new()),
            columns: RefCell::new(Vec::new()),
        })
    }
}

impl Widget for Grid {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        // TODO: add all the item sizes together
        vec2!(0,0)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw grid cells
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {

            },
            _ => { },
        }
    }
}
