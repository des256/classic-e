// E - UI - Field
// Desmond Germans, 2020

// A field is one line of text input.

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
    },
};

#[derive(Copy,Clone)]
pub enum FieldHit {
    Nothing,
    LeftIcon,
    Text(usize),
    RightIcon,
}

/// Field.
pub struct Field {
    r: Cell<Rect<i32>>,
    hit: Cell<FieldHit>,
    text: RefCell<String>,
}

impl Field {
    pub fn new() -> Result<Field,SystemError> {
        Ok(Field {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(FieldHit::Nothing),
            text: RefCell::new(String::new()),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> FieldHit {
        FieldHit::Nothing  // Should be LeftIcon, Text(i) or RightIcon once we know how the text management works
    }
}

impl Widget for Field {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        // get the styles
        let styles = draw.styles.borrow();

        // take the size of a default text
        let size = styles.font.measure("Field Text");

        size
    }

    fn draw(&self,draw: &Draw) {
        // TODO: left icon, if any
        // TODO: text from offset
        // TODO: right icon, if any
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
