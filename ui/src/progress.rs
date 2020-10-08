// E - UI - Progress
// Desmond Germans, 2020

// A progress indicator is a bar that fills up to indicate the progress of
// something.

use{
    crate::*,
    std::cell::Cell,
};

/// Progress indicator.
pub struct Progress {
    r: Cell<Rect<i32>>,
    full: Cell<f32>,
    value: Cell<f32>,
}

impl Progress {
    pub fn new() -> Result<Progress,SystemError> {
        Ok(Progress {
            r: Cell::new(rect!(0,0,0,0)),
            full: Cell::new(1.0),
            value: Cell::new(0.0),
        })
    }
}

impl Widget for Progress {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,_draw: &Draw) -> Vec2<i32> {
        vec2!(0,0)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw background part
        // TODO: draw progress part
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
    }
}
