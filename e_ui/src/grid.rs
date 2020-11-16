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
    _rows: RefCell<Vec<RowSpec>>,
    _columns: RefCell<Vec<ColumnSpec>>,
}

impl Grid {
    pub fn new() -> Result<Rc<Grid>,SystemError> {
        Ok(Rc::new(Grid {
            r: Cell::new(rect!(0,0,0,0)),
            _rows: RefCell::new(Vec::new()),
            _columns: RefCell::new(Vec::new()),
        }))
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

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
