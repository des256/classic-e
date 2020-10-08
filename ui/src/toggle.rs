// E - UI - Toggle
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Toggle.
pub struct Toggle {
    r: Cell<Rect<i32>>,
}

impl Toggle {
    pub fn new() -> Result<Toggle,SystemError> {
        Ok(Toggle {
            r: Cell::new(rect!(0,0,0,0)),
        })
    }
}

impl Widget for Toggle {
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
