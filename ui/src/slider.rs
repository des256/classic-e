// E - UI - Slider
// Desmond Germans, 2020

// A slider is a horizontal or vertical control to indicate a value.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum SliderHit {
    Nothing,
    PageDown,
    Slider,
    PageUp,
}

/// Slider.
pub struct Slider {
    r: Cell<Rect<i32>>,
    hit: Cell<SliderHit>,
    orientation: Orientation,
    min: Cell<f32>,
    max: Cell<f32>,
    value: Cell<f32>,
}

impl Slider {
    pub fn new_horizontal() -> Result<Slider,SystemError> {
        Ok(Slider {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SliderHit::Nothing),
            orientation: Orientation::Horizontal,
            min: Cell::new(0.0),
            max: Cell::new(1.0),
            value: Cell::new(0.5),
        })
    }

    pub fn new_vertical() -> Result<Slider,SystemError> {
        Ok(Slider {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SliderHit::Nothing),
            orientation: Orientation::Vertical,
            min: Cell::new(0.0),
            max: Cell::new(1.0),
            value: Cell::new(0.5),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> SliderHit {
        SliderHit::Nothing  // should be PageDown, Slider or PageUp once we know where the slider is
    }
}

const SLIDER_SIZE: i32 = 20;

impl Widget for Slider {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        // TODO: calculate all the things
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                vec2!(SLIDER_SIZE * 2,SLIDER_SIZE)
            },
            Orientation::Vertical => {
                vec2!(SLIDER_SIZE,SLIDER_SIZE * 2)
            },
        }
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw up/left page
        // TODO: draw slider
        // TODO: draw down/right page
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}
