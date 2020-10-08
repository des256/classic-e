// E - UI - Button
// Desmond Germans, 2020

// A button is a rectangle with a text, image or action reference that can be
// clicked.

use{
    crate::*,
    std::cell::Cell,
};

/// Button.
pub struct Button {
    r: Cell<Rect<i32>>,
    name: String,
}

impl Button {
    pub fn new(name: &str) -> Result<Button,SystemError> {
        Ok(Button {
            r: Cell::new(rect!(0,0,0,0)),
            name: name.to_string(),
        })
    }
}

impl Widget for Button {
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