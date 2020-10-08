// E - UI - Splitter
// Desmond Germans, 2020

// A splitter is a horizontal or vertical arrangement of two widgets where the
// bar between can be moved to give the widgets different space.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

#[derive(Copy,Clone)]
pub enum SplitterHit {
    Nothing,
    TopLeft,
    Separator,
    BottomRight,
}

/// Splitter.
pub struct Splitter {
    r: Cell<Rect<i32>>,
    hit: Cell<SplitterHit>,
    orientation: Orientation,
    topleft: Rc<dyn Widget>,
    bottomright: Rc<dyn Widget>,
    pos: Cell<i32>,
}

impl Splitter {
    pub fn new_horizontal(left: Rc<dyn Widget>,right: Rc<dyn Widget>) -> Result<Splitter,SystemError> {
        Ok(Splitter {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SplitterHit::Nothing),
            orientation: Orientation::Horizontal,
            topleft: left,
            bottomright: right,
            pos: Cell::new(0),
        })
    }

    pub fn new_vertical(top: Rc<dyn Widget>,bottom: Rc<dyn Widget>) -> Result<Splitter,SystemError> {
        Ok(Splitter {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SplitterHit::Nothing),
            orientation: Orientation::Vertical,
            topleft: top,
            bottomright: bottom,
            pos: Cell::new(0),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> SplitterHit {
        SplitterHit::Nothing  // Should be TopLeft, Separator or BottomRight when the separator is known
    }
}

const SPLITTER_SEPARATOR_SIZE: i32 = 5;

impl Widget for Splitter {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                let mut total_size = vec2!(SPLITTER_SEPARATOR_SIZE,0);
                let size = self.topleft.calc_min_size(draw);
                total_size += vec2!(size.x(),0);
                if size.y() > total_size.y() {
                    total_size.set_y(size.y());
                }
                let size = self.bottomright.calc_min_size(draw);
                total_size += vec2!(size.x(),0);
                if size.y() > total_size.y() {
                    total_size.set_y(size.y());
                }
                total_size
            },
            Orientation::Vertical => {
                let mut total_size = vec2!(0,SPLITTER_SEPARATOR_SIZE);
                let size = self.topleft.calc_min_size(draw);
                if size.x() > total_size.x() {
                    total_size.set_x(size.x());
                }
                total_size += vec2!(0,size.y());
                let size = self.bottomright.calc_min_size(draw);
                if size.x() > total_size.x() {
                    total_size.set_x(size.x());
                }
                total_size += vec2!(0,size.y());
                total_size
            },
        }
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw up/left child
        // TODO: draw separator
        // TODO: draw down/right child
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
