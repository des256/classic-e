// E - UI - Scroller
// Desmond Germans, 2020

// A scroller is a window onto a virtually much larger area.

use{
    crate::*,
    std::cell::Cell,
};

/// Scroller.
pub struct Scroller {
    r: Cell<Rect<i32>>,
}

impl Scroller {
    pub fn new() -> Result<Scroller,SystemError> {
        Ok(Scroller {
            r: Cell::new(rect!(0,0,0,0)),
        })
    }
}

impl Widget for Scroller {
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
