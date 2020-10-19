// E - UI - Progress
// Desmond Germans, 2020

// A progress indicator is a bar that fills up to indicate the progress of
// something.

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
        rc::Rc,
    },
};

/// Progress indicator.
pub struct Progress {
    ui: Rc<UI>,
    orientation: Orientation,
    style: RefCell<style::Progress>,
    r: Cell<Rect<i32>>,
    full: Cell<f32>,
    value: Cell<f32>,
    enabled: Cell<bool>,
}

impl Progress {
    pub fn new_horizontal(ui: &Rc<UI>,full: f32) -> Result<Rc<Progress>,SystemError> {
        Ok(Rc::new(Progress {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            style: RefCell::new(style::Progress {
                full_color: 0xCC6633,
                empty_color: 0x222222,
                disabled_color: 0x888888,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            full: Cell::new(full),
            value: Cell::new(0.0),
            enabled: Cell::new(true),
        }))
    }

    pub fn new_vertical(ui: &Rc<UI>,full: f32) -> Result<Rc<Progress>,SystemError> {
        Ok(Rc::new(Progress {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            style: RefCell::new(style::Progress {
                full_color: 0xCC6633,
                empty_color: 0x222222,
                disabled_color: 0x888888,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            full: Cell::new(full),
            value: Cell::new(0.0),
            enabled: Cell::new(true),
        }))
    }

    pub fn set_value(&self,value: f32) {
        self.value.set(value);
    }
}

const PROGRESS_SIZE: i32 = 10;

impl Widget for Progress {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        self.set_value(self.value.get());
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        vec2!(PROGRESS_SIZE,PROGRESS_SIZE)
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut full_color = style.disabled_color;
        if self.enabled.get() {
            full_color = style.full_color;
        }
        match self.orientation {
            Orientation::Horizontal => {
                let pos = ((self.value.get() * (self.r.get().s.x as f32)) / self.full.get()) as i32;
                self.ui.draw.draw_rectangle(rect!(vec2!(0,0),vec2!(pos,self.r.get().s.y)),full_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(pos,0),vec2!(self.r.get().s.x - pos,self.r.get().s.y)),style.empty_color,BlendMode::Replace);
            },    
            Orientation::Vertical => {
                // invert position
                let pos = (((self.full.get() - self.value.get()) * (self.r.get().s.y as f32)) / self.full.get()) as i32;
                self.ui.draw.draw_rectangle(rect!(vec2!(0,0),vec2!(self.r.get().s.x,pos)),style.empty_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(0,pos),vec2!(self.r.get().s.x,self.r.get().s.y - pos)),full_color,BlendMode::Replace);
            },
        }
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
