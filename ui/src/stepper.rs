// E - UI - Stepper
// Desmond Germans, 2020

// A stepper is an up/down control to precisely specify a numerical value.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum StepperHit {
    Nothing,
    StepDown,
    StepUp,
}

/// Stepper.
pub struct Stepper {
    r: Cell<Rect<i32>>,
    hit: Cell<StepperHit>,
    min: Cell<i32>,
    max: Cell<i32>,
    value: Cell<i32>,
}

const STEPPER_SIZE_X: i32 = 10;
const STEPPER_SIZE_Y: i32 = 20;

impl Stepper {
    pub fn new() -> Result<Stepper,SystemError> {
        Ok(Stepper {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(StepperHit::Nothing),
            min: Cell::new(0),
            max: Cell::new(256),
            value: Cell::new(128),
        })
    }

    pub fn find_hit(&self,_draw: &Draw,p: Vec2<i32>) -> StepperHit {
        if p.y() < STEPPER_SIZE_Y / 2 {
            StepperHit::StepUp
        }
        else {
            StepperHit::StepDown
        }
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
        vec2!(STEPPER_SIZE_X,STEPPER_SIZE_Y)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw up
        // TODO: draw down
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
