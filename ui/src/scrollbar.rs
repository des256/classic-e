// E - UI - ScrollBar
// Desmond Germans, 2020

// A scroll bar is a horizontal or vertical control around a scroller that
// pans the scroller around on a virtually much larger area.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum ScrollBarHit {
    Nothing,
    StepDown,
    PageDown,
    Bar,
    PageUp,
    StepUp,
}

/// Scroll bar.
pub struct ScrollBar {
    r: Cell<Rect<i32>>,
    hit: Cell<ScrollBarHit>,
    orientation: Orientation,
    enabled: Cell<bool>,
}

const SCROLLBAR_SIZE: i32 = 20;

impl ScrollBar {
    pub fn new_horizontal() -> Result<ScrollBar,SystemError> {
        Ok(ScrollBar {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ScrollBarHit::Nothing),
            orientation: Orientation::Horizontal,
            enabled: Cell::new(true),
        })
    }

    pub fn new_vertical() -> Result<ScrollBar,SystemError> {
        Ok(ScrollBar {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ScrollBarHit::Nothing),
            orientation: Orientation::Vertical,
            enabled: Cell::new(true),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> ScrollBarHit {
        ScrollBarHit::Nothing  // should be StepDown, PageDown, Bar, PageUp or StepUp once we know where the scrollbar is
    }
}

impl Widget for ScrollBar {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        // TODO: recalculate all the things
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                vec2!(SCROLLBAR_SIZE * 4,SCROLLBAR_SIZE)
            },
            Orientation::Vertical => {
                vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE * 4)
            },
        }
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw up/left arrow
        // TODO: draw up/left page
        // TODO: draw tab
        // TODO: draw down/right page
        // TODO: draw down/right arrow
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
