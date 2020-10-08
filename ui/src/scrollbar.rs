// E - UI - ScrollBar
// Desmond Germans, 2020

// A scroll bar is a horizontal or vertical control around a scroller that
// pans the scroller around on a virtually much larger area.

use{
    crate::*,
    std::cell::Cell,
};

/// Scroll bar.
pub struct ScrollBar {
    r: Cell<Rect<i32>>,
    orientation: Orientation,
}

impl ScrollBar {
    pub fn new_horizontal() -> Result<ScrollBar,SystemError> {
        Ok(ScrollBar {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Horizontal,
        })
    }

    pub fn new_vertical() -> Result<ScrollBar,SystemError> {
        Ok(ScrollBar {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Vertical,
        })
    }
}

impl Widget for ScrollBar {
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
