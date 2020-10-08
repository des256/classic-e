// E - UI - Scroller
// Desmond Germans, 2020

// A scroller is a window onto a virtually much larger area.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    }
};

/// Scroller.
pub struct Scroller {
    r: Cell<Rect<i32>>,
    child: Rc<dyn Widget>,
    // TBD offset
}

impl Scroller {
    pub fn new(child: Rc<dyn Widget>) -> Result<Scroller,SystemError> {
        Ok(Scroller {
            r: Cell::new(rect!(0,0,0,0)),
            child: child,
        })
    }
}

impl Widget for Scroller {
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
        // TODO: draw child at offset
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {
                // TODO: pass down to child
            },
            Event::MouseRelease(p,b) => {
                // TODO: pass down to child
            },
            Event::MouseMove(p) => {
                // TODO: pass down to child
            },
            _ => { },
        }
    }
}
