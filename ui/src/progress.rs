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

/// Progress indicator style.
pub struct ProgressStyle {
    pub full_color: u32,
    pub empty_color: u32,
}

/// Progress indicator.
pub struct Progress {
    ui: Rc<UI>,
    orientation: Orientation,
    style: RefCell<ProgressStyle>,
    r: Cell<Rect<i32>>,
    full: Cell<f32>,
    value: Cell<f32>,
}

impl Progress {
    pub fn new_horizontal(ui: &Rc<UI>,full: f32) -> Result<Progress,SystemError> {
        Ok(Progress {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            style: RefCell::new(ProgressStyle {
                full_color: 0xCC6633,
                empty_color: 0x222222,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            full: Cell::new(full),
            value: Cell::new(0.0),
        })
    }

    pub fn new_vertical(ui: &Rc<UI>,full: f32) -> Result<Progress,SystemError> {
        Ok(Progress {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            style: RefCell::new(ProgressStyle {
                full_color: 0xCC6633,
                empty_color: 0x222222,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            full: Cell::new(full),
            value: Cell::new(0.0),
        })
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
        match self.orientation {
            Orientation::Horizontal => {
                let pos = ((self.value.get() * (self.r.get().sx() as f32)) / self.full.get()) as i32;
                self.ui.draw_rectangle(rect!(vec2!(0,0),vec2!(pos,self.r.get().sy())),style.full_color,BlendMode::Replace);
                self.ui.draw_rectangle(rect!(vec2!(pos,0),vec2!(self.r.get().sx() - pos,self.r.get().sy())),style.empty_color,BlendMode::Replace);
            },    
            Orientation::Vertical => {
                // invert position
                let pos = (((self.full.get() - self.value.get()) * (self.r.get().sy() as f32)) / self.full.get()) as i32;
                self.ui.draw_rectangle(rect!(vec2!(0,0),vec2!(self.r.get().sx(),pos)),style.empty_color,BlendMode::Replace);
                self.ui.draw_rectangle(rect!(vec2!(0,pos),vec2!(self.r.get().sx(),self.r.get().sy() - pos)),style.full_color,BlendMode::Replace);
            },
        }
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
