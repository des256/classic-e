// E - UI - ScrollBar
// Desmond Germans, 2020

// A scroll bar is a horizontal or vertical control around a scroller that
// pans the scroller around on a virtually much larger area.

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

#[derive(Copy,Clone,Debug)]
pub enum ScrollBarHit {
    Nothing,
    StepLess,
    PageLess,
    Tab,
    PageMore,
    StepMore,
}

/// Horizontal or vertical scroll bar.
pub struct ScrollBar {
    ui: Rc<UI>,
    orientation: Orientation,
    style: RefCell<style::ScrollBar>,
    r: Cell<Rect<i32>>,
    hit: Cell<ScrollBarHit>,
    capturing: Cell<bool>,
    _enabled: Cell<bool>,
    full: Cell<f32>,  // parameters: full range
    page: Cell<f32>,  // parameters: page size
    step: Cell<f32>,  // parameters: step size
    value: Cell<f32>,  // current value of the scrollbar
    start_pos: Cell<i32>,  // dragging system
    start_p: Cell<Vec2<i32>>,  // dragging system
}

const SCROLLBAR_SIZE: i32 = 20;

impl ScrollBar {
    pub fn new_horizontal(ui: &Rc<UI>,full: f32,page: f32,step: f32) -> Result<Rc<ScrollBar>,SystemError> {
        Ok(Rc::new(ScrollBar {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            style: RefCell::new(style::ScrollBar {
                step_color: 0x888888,
                step_hover_color: 0x224488,
                page_color: 0x777777,
                page_hover_color: 0x224488,
                tab_color: 0x888888,
                tab_hover_color: 0x224488,    
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ScrollBarHit::Nothing),
            capturing: Cell::new(false),
            _enabled: Cell::new(true),
            full: Cell::new(full),
            page: Cell::new(page),
            step: Cell::new(step),
            value: Cell::new(0.0),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
        }))
    }

    pub fn new_vertical(ui: &Rc<UI>,full: f32,page: f32,step: f32) -> Result<Rc<ScrollBar>,SystemError> {
        Ok(Rc::new(ScrollBar {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            style: RefCell::new(style::ScrollBar {
                step_color: 0x888888,
                step_hover_color: 0x224488,
                page_color: 0x777777,
                page_hover_color: 0x224488,
                tab_color: 0x888888,
                tab_hover_color: 0x224488,    
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ScrollBarHit::Nothing),
            capturing: Cell::new(false),
            _enabled: Cell::new(true),
            full: Cell::new(full),
            page: Cell::new(page),
            step: Cell::new(step),
            value: Cell::new(0.0),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
        }))
    }

    fn find_hit(&self,p: Vec2<i32>) -> ScrollBarHit {
        if rect!(vec2!(0,0),self.r.get().s).contains(&p) {
            match self.orientation {
                Orientation::Horizontal => {
                    let size = (self.page.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                    let pos = (self.value.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32;
                    if p.x < SCROLLBAR_SIZE {
                        ScrollBarHit::StepLess
                    }
                    else if p.x < SCROLLBAR_SIZE + pos {
                        ScrollBarHit::PageLess
                    }
                    else if p.x < SCROLLBAR_SIZE + pos + size {
                        ScrollBarHit::Tab
                    }
                    else if p.x < self.r.get().s.x - SCROLLBAR_SIZE {
                        ScrollBarHit::PageMore
                    }
                    else {
                        ScrollBarHit::StepMore
                    }
                },
                Orientation::Vertical => {
                    let size = (self.page.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                    let pos = (self.value.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32;
                    if p.y < SCROLLBAR_SIZE {
                        ScrollBarHit::StepLess
                    }
                    else if p.y < SCROLLBAR_SIZE + pos {
                        ScrollBarHit::PageLess
                    }
                    else if p.y < SCROLLBAR_SIZE + pos + size {
                        ScrollBarHit::Tab
                    }
                    else if p.y < self.r.get().s.y - SCROLLBAR_SIZE {
                        ScrollBarHit::PageMore
                    }
                    else {
                        ScrollBarHit::StepMore
                    }
                },
            }
        }
        else {
            ScrollBarHit::Nothing
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

impl Widget for ScrollBar {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                vec2!(SCROLLBAR_SIZE * 4,SCROLLBAR_SIZE)
            },
            Orientation::Vertical => {
                vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE * 4)
            },
        }
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut stepless_color = style.step_color;
        let mut pageless_color = style.page_color;
        let mut tab_color = style.tab_color;
        let mut pagemore_color = style.page_color;
        let mut stepmore_color = style.step_color;
        match self.hit.get() {
            ScrollBarHit::Nothing => { },
            ScrollBarHit::StepLess => {
                stepless_color = style.step_hover_color;
            },
            ScrollBarHit::PageLess => {
                pageless_color = style.page_hover_color;
            },
            ScrollBarHit::Tab => {
                tab_color = style.tab_hover_color;
            },
            ScrollBarHit::PageMore => {
                pagemore_color = style.page_hover_color;
            },
            ScrollBarHit::StepMore => {
                stepmore_color = style.step_hover_color;
            },
        }
        match self.orientation {
            Orientation::Horizontal => {
                let size = (self.page.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                let pos = (self.value.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32;
                self.ui.draw.draw_rectangle(rect!(vec2!(0,0),vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE)),stepless_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(SCROLLBAR_SIZE,0),vec2!(pos,SCROLLBAR_SIZE)),pageless_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(SCROLLBAR_SIZE + pos,0),vec2!(size,SCROLLBAR_SIZE)),tab_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(SCROLLBAR_SIZE + pos + size,0),vec2!(self.r.get().s.x - 2 * SCROLLBAR_SIZE - pos - size,SCROLLBAR_SIZE)),pagemore_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(self.r.get().s.x - SCROLLBAR_SIZE,0),vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE)),stepmore_color,BlendMode::Replace);
            },
            Orientation::Vertical => {
                let size = (self.page.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                let pos = (self.value.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32;
                self.ui.draw.draw_rectangle(rect!(vec2!(0,0),vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE)),stepless_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(0,SCROLLBAR_SIZE),vec2!(SCROLLBAR_SIZE,pos)),pageless_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(0,SCROLLBAR_SIZE + pos),vec2!(SCROLLBAR_SIZE,size)),tab_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(0,SCROLLBAR_SIZE + pos + size),vec2!(SCROLLBAR_SIZE,self.r.get().s.y - 2 * SCROLLBAR_SIZE - pos - size)),pagemore_color,BlendMode::Replace);
                self.ui.draw.draw_rectangle(rect!(vec2!(0,self.r.get().s.y - SCROLLBAR_SIZE),vec2!(SCROLLBAR_SIZE,SCROLLBAR_SIZE)),stepmore_color,BlendMode::Replace);
            },
        }
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,p: Vec2<i32>,_b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ScrollBarHit::StepLess => {
                    true
                },
                ScrollBarHit::PageLess => {
                    true
                },
                ScrollBarHit::Tab => {
                    true
                },
                ScrollBarHit::PageMore => {
                    true
                },
                ScrollBarHit::StepMore => {
                    true
                }
            }
        }
        else {
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    false
                },
                ScrollBarHit::StepLess => {
                    println!("ScrollBar: start clicking StepLess");
                    self.capturing.set(true);
                    true
                },
                ScrollBarHit::PageLess => {
                    println!("ScrollBar: start clicking PageLess");
                    self.capturing.set(true);
                    true
                },
                ScrollBarHit::Tab => {
                    println!("ScrollBar: start dragging Tab");
                    let pos = match self.orientation {
                        Orientation::Horizontal => {
                            let size = (self.page.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                            (self.value.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32
                        },
                        Orientation::Vertical => {
                            let size = (self.page.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                            (self.value.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE - size) as f32) / self.full.get()) as i32
                        },
                    };
                    self.start_pos.set(pos);
                    self.start_p.set(p);
                    self.capturing.set(true);
                    true
                },
                ScrollBarHit::PageMore => {
                    println!("ScrollBar: start clicking PageMore");
                    self.capturing.set(true);
                    true
                },
                ScrollBarHit::StepMore => {
                    println!("ScrollBar: start clicking StepMore");
                    self.capturing.set(true);
                    true
                }
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,_b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ScrollBarHit::StepLess => {
                    println!("ScrollBar: stop clicking StepLess");
                    self.set_value(self.value.get() - self.step.get());
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                ScrollBarHit::PageLess => {
                    println!("ScrollBar: stop clicking PageLess");
                    self.set_value(self.value.get() - self.page.get());
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                ScrollBarHit::Tab => {
                    println!("ScrollBar: stop dragging tab");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                ScrollBarHit::PageMore => {
                    println!("ScrollBar: stop clicking PageMore");
                    self.set_value(self.value.get() + self.page.get());
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                ScrollBarHit::StepMore => {
                    println!("ScrollBar: stop clicking StepMore");
                    self.set_value(self.value.get() + self.step.get());
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                }
            }
        }
        else {
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    false
                },
                ScrollBarHit::StepLess => {
                    false
                },
                ScrollBarHit::PageLess => {
                    false
                },
                ScrollBarHit::Tab => {
                    false
                },
                ScrollBarHit::PageMore => {
                    false
                },
                ScrollBarHit::StepMore => {
                    false
                },
            }
        }
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ScrollBarHit::StepLess => {
                    println!("ScrollBar: still clicking StepLess");
                    true
                },
                ScrollBarHit::PageLess => {
                    println!("ScrollBar: still clicking PageLess");
                    true
                },
                ScrollBarHit::Tab => {
                    match self.orientation {
                        Orientation::Horizontal => {
                            let size = (self.page.get() * ((self.r.get().s.x - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                            let wanted_pos = self.start_pos.get() + p.x - self.start_p.get().x;
                            let wanted_value = (wanted_pos as f32) * self.full.get() / ((self.r.get().s.x - 2 * SCROLLBAR_SIZE - size) as f32);
                            self.set_value(wanted_value);
                        },
                        Orientation::Vertical => {
                            let size = (self.page.get() * ((self.r.get().s.y - 2 * SCROLLBAR_SIZE) as f32) / self.full.get()) as i32;
                            let wanted_pos = self.start_pos.get() + p.y - self.start_p.get().y;
                            let wanted_value = (wanted_pos as f32) * self.full.get() / ((self.r.get().s.y - 2 * SCROLLBAR_SIZE - size) as f32);
                            self.set_value(wanted_value);
                        },
                    }
                    println!("ScrollBar: dragging tab");
                    true
                },
                ScrollBarHit::PageMore => {
                    println!("ScrollBar: still clicking PageMore");
                    true
                },
                ScrollBarHit::StepMore => {
                    println!("ScrollBar: still clicking StepMore");
                    true
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                ScrollBarHit::Nothing => {
                    false
                },
                ScrollBarHit::StepLess => {
                    true
                },
                ScrollBarHit::PageLess => {
                    true
                },
                ScrollBarHit::Tab => {
                    true
                },
                ScrollBarHit::PageMore => {
                    true
                },
                ScrollBarHit::StepMore => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
