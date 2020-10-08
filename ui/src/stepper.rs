// E - UI - Stepper
// Desmond Germans, 2020

// A stepper is an up/down control to precisely specify a numerical value.

use{
    crate::*,
    std::cell::Cell,
};

/// Stepper.
pub struct Stepper {
    r: Cell<Rect<i32>>,
}

impl Stepper {
    pub fn new() -> Result<Stepper,SystemError> {
        Ok(Stepper {
            r: Cell::new(rect!(0,0,0,0)),
        })
    }
}

impl Widget for Stepper {
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
