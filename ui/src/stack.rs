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
        // TODO: calculate child rects
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                let mut total_size = vec2!(0i32,0i32);
                for child in self.children.iter() {
                    let size = child.calc_min_size(draw);
                    total_size += vec2!(size.x(),0);
                    if size.y() > total_size.y() {
                        total_size.set_y(size.y());
                    }
                }
                total_size
            },
            Orientation::Vertical => {
                let mut total_size = vec2!(0i32,0i32);
                for child in self.children.iter() {
                    let size = child.calc_min_size(draw);
                    if size.x() > total_size.x() {
                        total_size.set_x(size.x());
                    }
                    total_size += vec2!(0,size.y());
                }
                total_size
            },
        }
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw all the children
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {
                // TODO: pass to child
            },
            Event::MouseRelease(p,b) => {
                // TODO: pass to child
            },
            Event::MouseMove(p) => {
                // TODO: pass to child
            },
            _ => { },
        }
    }
}
