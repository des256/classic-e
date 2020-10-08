// E - UI - Slider
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Slider.
pub struct Slider {
    r: Cell<Rect<i32>>,
    orientation: Orientation,
}

impl Slider {
    pub fn new_horizontal() -> Result<Slider,SystemError> {
        Ok(Slider {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Horizontal,
        })
    }

    pub fn new_vertical() -> Result<Slider,SystemError> {
        Ok(Slider {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Vertical,
        })
    }
}

impl Widget for Slider {
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
