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

/// Scroller over a child widget.
pub struct Scroller {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    child: Rc<dyn Widget>,
    offset: Cell<Vec2<i32>>,
}

impl Scroller {
    pub fn new(ui: &Rc<UI>,child: Rc<dyn Widget>) -> Result<Rc<Scroller>,SystemError> {
        Ok(Rc::new(Scroller {
            ui: Rc::clone(&ui),
            r: Cell::new(rect!(0,0,0,0)),
            child: child,
            offset: Cell::new(vec2!(0,0)),
        }))
    }
}

const SCROLLER_MIN_SIZE: i32 = 10;

impl Widget for Scroller {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        vec2!(SCROLLER_MIN_SIZE,SCROLLER_MIN_SIZE)
    }

    fn draw(&self) {
        self.ui.draw.delta_offset(-self.offset.get());
        self.child.draw();
        self.ui.draw.delta_offset(self.offset.get());
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        self.child.mousepress(ui,window,p - self.rect().o + self.offset.get(),b)
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        self.child.mouserelease(ui,window,p - self.rect().o + self.offset.get(),b)
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        self.child.mousemove(ui,window,p - self.rect().o + self.offset.get())
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        self.child.mousewheel(ui,window,w)
    }
}
