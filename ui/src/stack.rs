// E - UI - Stack
// Desmond Germans, 2020

// A stack is a horizontal or vertical bunch of widgets.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

/// Stack.
pub struct Stack {
    r: Cell<Rect<i32>>,
    orientation: Orientation,
    children: Vec<Rc<dyn Widget>>,
}

impl Stack {
    pub fn new_horizontal(children: Vec<Rc<dyn Widget>>) -> Result<Stack,SystemError> {
        Ok(Stack {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Horizontal,
            children: children,
        })
    }

    pub fn new_vertical(children: Vec<Rc<dyn Widget>>) -> Result<Stack,SystemError> {
        Ok(Stack {
            r: Cell::new(rect!(0,0,0,0)),
            orientation: Orientation::Vertical,
            children: children,
        })
    }
}

impl Widget for Stack {
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
