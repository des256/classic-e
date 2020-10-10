// E - UI - Slider
// Desmond Germans, 2020

// A slider is a horizontal or vertical control to indicate a value.

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

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum SliderHit {
    Nothing,
    PageLess,
    Tab,
    PageMore,
}

/// Horizontal or vertical slider style.
pub struct SliderStyle {
    pub color: u32,
    pub empty_color: u32,
    pub full_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
}

/// Horizontal or vertical slider.
pub struct Slider {
    ui: Rc<UI>,
    orientation: Orientation,
    style: RefCell<SliderStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<SliderHit>,
    capturing: Cell<bool>,
    full: Cell<f32>,
    value: Cell<f32>,
    start_pos: Cell<i32>,
    start_p: Cell<Vec2<i32>>,
}

const SLIDER_SIZE: i32 = 20;
const SLIDER_GUTTER_SIZE: i32 = 10;

impl Slider {
    pub fn new_horizontal(ui: &Rc<UI>) -> Result<Slider,SystemError> {
        Ok(Slider {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            style: RefCell::new(SliderStyle {
                color: 0x444444,
                empty_color: 0x222222,
                full_color: 0xCC6633,
                tab_color: 0xAAAAAA,
                tab_hover_color: 0x3366CC,    
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SliderHit::Nothing),
            capturing: Cell::new(false),
            full: Cell::new(1.0),
            value: Cell::new(0.5),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
        })
    }

    pub fn new_vertical(ui: &Rc<UI>) -> Result<Slider,SystemError> {
        Ok(Slider {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            style: RefCell::new(SliderStyle {
                color: 0x444444,
                empty_color: 0x222222,
                full_color: 0xCC6633,
                tab_color: 0xAAAAAA,
                tab_hover_color: 0x3366CC,    
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SliderHit::Nothing),
            capturing: Cell::new(false),
            full: Cell::new(1.0),
            value: Cell::new(0.5),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> SliderHit {
        if rect!(vec2!(0,0),self.r.get().s()).contains(&p) {
            match self.orientation {
                Orientation::Horizontal => {
                    let pos = (self.value.get() * ((self.r.get().sx() - SLIDER_SIZE) as f32) / self.full.get()) as i32;
                    if p.x() < pos {
                        SliderHit::PageLess
                    }
                    else if p.x() < pos + SLIDER_SIZE {
                        SliderHit::Tab
                    }
                    else {
                        SliderHit::PageMore
                    }
                },
                Orientation::Vertical => {
                    // invert position
                    let pos = ((self.full.get() - self.value.get()) * ((self.r.get().sy() - SLIDER_SIZE) as f32) / self.full.get()) as i32;
                    if p.y() < pos {
                        SliderHit::PageLess
                    }
                    else if p.y() < pos + SLIDER_SIZE {
                        SliderHit::Tab
                    }
                    else {
                        SliderHit::PageMore
                    }
                },
            }
        }
        else {
            SliderHit::Nothing
        }
    }

    pub fn set_value(&self,value: f32) {
        let mut new_value = value;
        if new_value < 0.0 {
            new_value = 0.0;
        }
        if new_value > self.full.get() {
            new_value = self.full.get();
        }
        self.value.set(new_value);
    }
}

impl Widget for Slider {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                vec2!(SLIDER_SIZE * 2,SLIDER_SIZE)
            },
            Orientation::Vertical => {
                vec2!(SLIDER_SIZE,SLIDER_SIZE * 2)
            },
        }
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let tab_color = if let SliderHit::Tab = self.hit.get() {
            style.tab_hover_color
        }
        else {
            style.tab_color
        };
        let ridge = (SLIDER_SIZE - SLIDER_GUTTER_SIZE) / 2;
        match self.orientation {
            Orientation::Horizontal => {
                let pos = (self.value.get() * ((self.r.get().sx() - SLIDER_SIZE) as f32) / self.full.get()) as i32;
                if pos > 0 {
                    self.ui.draw_rectangle(rect!(vec2!(0,0),vec2!(ridge,SLIDER_SIZE)),style.color,BlendMode::Replace);
                    if pos > ridge {
                        self.ui.draw_rectangle(rect!(vec2!(ridge,0),vec2!(pos - ridge,ridge)),style.color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge,ridge),vec2!(pos - ridge,SLIDER_GUTTER_SIZE)),style.full_color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge,ridge + SLIDER_GUTTER_SIZE),vec2!(pos - ridge,ridge)),style.color,BlendMode::Replace);
                    }
                }
                self.ui.draw_rectangle(rect!(vec2!(pos,0),vec2!(SLIDER_SIZE,SLIDER_SIZE)),tab_color,BlendMode::Replace);
                if pos < self.r.get().sx() - SLIDER_SIZE {
                    if pos < self.r.get().sx() - SLIDER_SIZE - ridge {
                        self.ui.draw_rectangle(rect!(vec2!(pos + SLIDER_SIZE,0),vec2!(self.r.get().sx() - ridge - pos - SLIDER_SIZE,ridge)),style.color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(pos + SLIDER_SIZE,ridge),vec2!(self.r.get().sx() - ridge - pos - SLIDER_SIZE,SLIDER_GUTTER_SIZE)),style.empty_color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(pos + SLIDER_SIZE,ridge + SLIDER_GUTTER_SIZE),vec2!(self.r.get().sx() - ridge - pos - SLIDER_SIZE,ridge)),style.color,BlendMode::Replace);
                    }
                    self.ui.draw_rectangle(rect!(vec2!(self.r.get().sx() - ridge,0),vec2!(ridge,SLIDER_SIZE)),style.color,BlendMode::Replace);
                }
            },
            Orientation::Vertical => {
                // invert position
                let pos = ((self.full.get() - self.value.get()) * ((self.r.get().sy() - SLIDER_SIZE) as f32) / self.full.get()) as i32;
                if pos > 0 {
                    self.ui.draw_rectangle(rect!(vec2!(0,0),vec2!(SLIDER_SIZE,ridge)),style.color,BlendMode::Replace);
                    if pos > ridge {
                        self.ui.draw_rectangle(rect!(vec2!(0,ridge),vec2!(ridge,pos - ridge)),style.color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge,ridge),vec2!(SLIDER_GUTTER_SIZE,pos - ridge)),style.empty_color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge + SLIDER_GUTTER_SIZE,ridge),vec2!(ridge,pos - ridge)),style.color,BlendMode::Replace);
                    }
                }
                self.ui.draw_rectangle(rect!(vec2!(0,pos),vec2!(SLIDER_SIZE,SLIDER_SIZE)),tab_color,BlendMode::Replace);
                if pos < self.r.get().sy() - SLIDER_SIZE {
                    if pos < self.r.get().sy() - SLIDER_SIZE - ridge {
                        self.ui.draw_rectangle(rect!(vec2!(0,pos + SLIDER_SIZE),vec2!(ridge,self.r.get().sy() - ridge - pos - SLIDER_SIZE)),style.color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge,pos + SLIDER_SIZE),vec2!(SLIDER_GUTTER_SIZE,self.r.get().sy() - ridge - pos - SLIDER_SIZE)),style.full_color,BlendMode::Replace);
                        self.ui.draw_rectangle(rect!(vec2!(ridge + SLIDER_GUTTER_SIZE,pos + SLIDER_SIZE),vec2!(ridge,self.r.get().sy() - ridge - pos - SLIDER_SIZE)),style.color,BlendMode::Replace);
                    }
                    self.ui.draw_rectangle(rect!(vec2!(0,self.r.get().sy() - ridge),vec2!(SLIDER_SIZE,ridge)),style.color,BlendMode::Replace);
                }
            },
        }
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SliderHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SliderHit::PageLess => {
                    true
                },
                SliderHit::Tab => {
                    true
                },
                SliderHit::PageMore => {
                    true
                },
            }
        }
        else {
            match self.hit.get() {
                SliderHit::Nothing => {
                    false
                },
                SliderHit::PageLess => {
                    println!("Slider: start clicking PageLess");
                    self.capturing.set(true);
                    true
                },
                SliderHit::Tab => {
                    println!("Slider: start dragging Tab");
                    let pos = match self.orientation {
                        Orientation::Horizontal => {
                            (self.value.get() * ((self.r.get().sx() - SLIDER_SIZE) as f32) / self.full.get()) as i32
                        },
                        Orientation::Vertical => {
                            // invert pos
                            ((self.full.get() - self.value.get()) * ((self.r.get().sy() - SLIDER_SIZE) as f32) / self.full.get()) as i32
                        },
                    };
                    self.start_pos.set(pos);
                    self.start_p.set(p);
                    self.capturing.set(true);
                    true
                },
                SliderHit::PageMore => {
                    println!("Slider: start clicking PageMore");
                    self.capturing.set(true);
                    true
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SliderHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SliderHit::PageLess => {
                    println!("Slider: stop clicking PageLess");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                SliderHit::Tab => {
                    println!("Slider: stop dragging Tab");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                SliderHit::PageMore => {
                    println!("Slider: stop clicking PageMore");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
            }
        }
        else {
            match self.hit.get() {
                SliderHit::Nothing => {
                    false
                },
                SliderHit::PageLess => {
                    false
                },
                SliderHit::Tab => {
                    false
                },
                SliderHit::PageMore => {
                    false
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SliderHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SliderHit::PageLess => {
                    println!("Slider: still clicking PageLess");
                    true
                },
                SliderHit::Tab => {
                    match self.orientation {
                        Orientation::Horizontal => {
                            let wanted_pos = self.start_pos.get() + p.x() - self.start_p.get().x();
                            let wanted_value = (wanted_pos as f32) * self.full.get() / ((self.r.get().sx() - SLIDER_SIZE) as f32);
                            self.set_value(wanted_value);
                        },
                        Orientation::Vertical => {
                            // invert position
                            let wanted_pos = self.start_pos.get() + p.y() - self.start_p.get().y();
                            let wanted_value = self.full.get() - ((wanted_pos as f32) * self.full.get() / ((self.r.get().sy() - SLIDER_SIZE) as f32));
                            self.set_value(wanted_value);
                        },
                    }
                    println!("Slider: dragging tab");
                    true
                },
                SliderHit::PageMore => {
                    println!("Slider: still clicking PageMore");
                    true
                }
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                SliderHit::Nothing => {
                    false
                },
                SliderHit::PageLess => {
                    true
                },
                SliderHit::Tab => {
                    true
                },
                SliderHit::PageMore => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}
