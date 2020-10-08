// E - UI - Grid
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

    fn calc_min_size(&self,_draw: &Draw) -> Vec2<i32> {
        vec2!(0,0)
    }

    fn draw(&self,_draw: &Draw) {
    }

    fn handle(&self,_ui: &UI,_window: &Window,_event: Event) {
    }
}
