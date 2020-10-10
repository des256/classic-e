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

#[doc(hidden)]
pub struct RowSpec {

}

#[doc(hidden)]
pub struct ColumnSpec {
    
}

/// Grid of child widgets.
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

    fn calc_min_size(&self) -> Vec2<i32> {
        // TODO: add all the item sizes together
        vec2!(0,0)
    }

    fn draw(&self) {
        // TODO: draw grid cells
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}
