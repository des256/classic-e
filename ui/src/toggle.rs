// E - UI - Toggle
// Desmond Germans, 2020

// A toggle allows the user to select or unselect a control.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum ToggleHit {
    Nothing,
    PageLeft,
    Toggle,
    PageRight,
}

/// Toggle.
pub struct Toggle {
    r: Cell<Rect<i32>>,
    hit: Cell<ToggleHit>,
    enabled: Cell<bool>,
    current: Cell<bool>,
}

const TOGGLE_SIZE_X: i32 = 40;
const TOGGLE_SIZE_Y: i32 = 20;

impl Toggle {
    pub fn new() -> Result<Toggle,SystemError> {
        Ok(Toggle {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ToggleHit::Nothing),
            enabled: Cell::new(true),
            current: Cell::new(false),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> ToggleHit {
        if p.x() < TOGGLE_SIZE_X / 2 {
            if self.current.get() {
                ToggleHit::PageLeft
            }
            else {
                ToggleHit::Toggle
            }
        }
        else {
            if self.current.get() {
                ToggleHit::Toggle
            }
            else {
                ToggleHit::PageRight
            }
        }
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
        vec2!(TOGGLE_SIZE_X,TOGGLE_SIZE_Y)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw left maybe
        // TODO: draw toggle
        // TODO: draw right maybe
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
