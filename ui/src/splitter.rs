// E - UI - Splitter
// Desmond Germans, 2020

// A splitter is a horizontal or vertical arrangement of two widgets where the
// bar between can be moved to give the widgets different space.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

/// Splitter.
pub struct Splitter {
    r: Cell<Rect<i32>>,
    orientation: Orientation,
    topleft: Rc<dyn Widget>,
    bottomright: Rc<dyn Widget>,
}

impl Splitter {
    pub fn new_horizontal(left: Rc<dyn Widget>,right: Rc<dyn Widget>) -> Result<Splitter,SystemError> {
        Ok(Splitter {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Horizontal,
            topleft: left,
            bottomright: right,
        })
    }

    pub fn new_vertical(top: Rc<dyn Widget>,bottom: Rc<dyn Widget>) -> Result<Splitter,SystemError> {
        Ok(Splitter {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Vertical,
            topleft: top,
            bottomright: bottom,
        })
    }
}

impl Widget for Splitter {
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
