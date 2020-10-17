// E - UI - Splitter
// Desmond Germans, 2020

// A splitter is a horizontal or vertical arrangement of two widgets where the
// bar between can be moved to give the widgets different space.

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
pub enum SplitterHit {
    Nothing,
    TopLeft,
    Separator,
    BottomRight,
}

/// Horizontal or vertical splitter style.
pub struct SplitterStyle {
    pub color: u32,
    pub hover_color: u32,
    pub disabled_color: u32,
}

/// Horizontal or vertical splitter.
pub struct Splitter {
    ui: Rc<UI>,
    orientation: Orientation,
    style: RefCell<SplitterStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<SplitterHit>,
    capturing: Cell<bool>,
    topleft: Rc<dyn Widget>,
    bottomright: Rc<dyn Widget>,
    pos: Cell<i32>,
    start_pos: Cell<i32>,
    start_p: Cell<Vec2<i32>>,
    enabled: Cell<bool>,
}

impl Splitter {
    pub fn new_horizontal(ui: &Rc<UI>,left: Rc<dyn Widget>,right: Rc<dyn Widget>) -> Result<Rc<Splitter>,SystemError> {
        Ok(Rc::new(Splitter {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            style: RefCell::new(SplitterStyle {
                color: 0x444444,
                hover_color: 0x224488,
                disabled_color: 0x333333,   
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SplitterHit::Nothing),
            capturing: Cell::new(false),
            topleft: left,
            bottomright: right,
            pos: Cell::new(0),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
            enabled: Cell::new(true),
        }))
    }

    pub fn new_vertical(ui: &Rc<UI>,top: Rc<dyn Widget>,bottom: Rc<dyn Widget>) -> Result<Rc<Splitter>,SystemError> {
        Ok(Rc::new(Splitter {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            style: RefCell::new(SplitterStyle {
                color: 0x444444,
                hover_color: 0x224488,   
                disabled_color: 0x333333, 
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(SplitterHit::Nothing),
            capturing: Cell::new(false),
            topleft: top,
            bottomright: bottom,
            pos: Cell::new(0),
            start_pos: Cell::new(0),
            start_p: Cell::new(vec2!(0,0)),
            enabled: Cell::new(true),
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> SplitterHit {
        if !rect!(vec2!(0,0),self.r.get().s).contains(&p) {
            return SplitterHit::Nothing
        }
        let pos = self.pos.get();
        match self.orientation {
            Orientation::Horizontal => {
                if p.x < pos {
                    SplitterHit::TopLeft
                }
                else if p.x < pos + SPLITTER_SEPARATOR_SIZE {
                    SplitterHit::Separator
                }
                else {
                    SplitterHit::BottomRight
                }
            },
            Orientation::Vertical => {
                if p.y < pos {
                    SplitterHit::TopLeft
                }
                else if p.y < pos + SPLITTER_SEPARATOR_SIZE {
                    SplitterHit::Separator
                }
                else {
                    SplitterHit::BottomRight
                }
            },
        }
    }

    pub fn set_pos(&self,pos: i32) {
        let mut new_pos = pos;
        let topleft_size = self.topleft.calc_min_size();
        let bottomright_size = self.bottomright.calc_min_size();
        let (range_low,range_high) = match self.orientation {
            Orientation::Horizontal => {
                (
                    topleft_size.x,
                    self.r.get().s.x - bottomright_size.x - SPLITTER_SEPARATOR_SIZE
                )
            },
            Orientation::Vertical => {
                (
                    topleft_size.y,
                    self.r.get().s.y - bottomright_size.y - SPLITTER_SEPARATOR_SIZE
                )
            },
        };
        if new_pos < range_low {
            new_pos = range_low;
        }
        if new_pos > range_high {
            new_pos = range_high;
        }
        self.pos.set(new_pos);
        match self.orientation {
            Orientation::Horizontal => {
                self.topleft.set_rect(rect!(vec2!(0,0),vec2!(new_pos,self.r.get().s.y)));
                self.bottomright.set_rect(rect!(vec2!(new_pos + SPLITTER_SEPARATOR_SIZE,0),vec2!(self.r.get().s.x - new_pos - SPLITTER_SEPARATOR_SIZE,self.r.get().s.y)));
            },
            Orientation::Vertical => {
                self.topleft.set_rect(rect!(vec2!(0,0),vec2!(self.r.get().s.x,new_pos)));
                self.bottomright.set_rect(rect!(vec2!(0,new_pos + SPLITTER_SEPARATOR_SIZE),vec2!(self.r.get().s.x,self.r.get().s.y - new_pos - SPLITTER_SEPARATOR_SIZE)));
            },
        }
    }
}

const SPLITTER_SEPARATOR_SIZE: i32 = 5;

impl Widget for Splitter {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        self.set_pos(self.pos.get());
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                let mut total_size = vec2!(SPLITTER_SEPARATOR_SIZE,0);
                let size = self.topleft.calc_min_size();
                total_size += vec2!(size.x,0);
                if size.y > total_size.y {
                    total_size.y = size.y;
                }
                let size = self.bottomright.calc_min_size();
                total_size += vec2!(size.x,0);
                if size.y > total_size.y {
                    total_size.y = size.y;
                }
                total_size
            },
            Orientation::Vertical => {
                let mut total_size = vec2!(0,SPLITTER_SEPARATOR_SIZE);
                let size = self.topleft.calc_min_size();
                if size.x > total_size.x {
                    total_size.x = size.x;
                }
                total_size += vec2!(0,size.y);
                let size = self.bottomright.calc_min_size();
                if size.x > total_size.x {
                    total_size.x = size.x;
                }
                total_size += vec2!(0,size.y);
                total_size
            },
        }
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let color = if self.enabled.get() {
            if let SplitterHit::Separator = self.hit.get() {
                style.hover_color
            }
            else {
                style.color
            }
        }
        else {
            style.disabled_color
        };
        let offset = self.topleft.rect().o;
        self.ui.delta_offset(offset);
        self.topleft.draw();
        self.ui.delta_offset(-offset);
        let pos = self.pos.get();
        match self.orientation {
            Orientation::Horizontal => {
                self.ui.draw_rectangle(rect!(pos,0,SPLITTER_SEPARATOR_SIZE,self.r.get().s.y),color,BlendMode::Replace);
            },
            Orientation::Vertical => {
                self.ui.draw_rectangle(rect!(0,pos,self.r.get().s.x,SPLITTER_SEPARATOR_SIZE),color,BlendMode::Replace);
            },
        }
        let offset = self.bottomright.rect().o;
        self.ui.delta_offset(offset);
        self.bottomright.draw();
        self.ui.delta_offset(-offset);
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SplitterHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mousepress(ui,window,p - self.topleft.rect().o,b);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    true
                },
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mousepress(ui,window,p - self.bottomright.rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            match self.hit.get() {
                SplitterHit::Nothing => {
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mousepress(ui,window,p - self.topleft.rect().o,b);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    println!("Splitter: start dragging separator");
                    self.start_pos.set(self.pos.get());
                    self.start_p.set(p);
                    self.capturing.set(true);
                    true
                }
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mousepress(ui,window,p - self.bottomright.rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SplitterHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mouserelease(ui,window,p - self.topleft.rect().o,b);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    println!("Splitter: stop dragging separator");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mouserelease(ui,window,p - self.bottomright.rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            match self.hit.get() {
                SplitterHit::Nothing => {
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mouserelease(ui,window,p - self.topleft.rect().o,b);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    false
                },
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mouserelease(ui,window,p - self.bottomright.rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                SplitterHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mousemove(ui,window,p - self.topleft.rect().o);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    let wanted_pos = match self.orientation {
                        Orientation::Horizontal => {
                            self.start_pos.get() + p.x - self.start_p.get().x
                        },
                        Orientation::Vertical => {
                            self.start_pos.get() + p.y - self.start_p.get().y
                        },
                    };
                    self.set_pos(wanted_pos);
                    println!("Splitter: dragging separator");
                    true
                },
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mousemove(ui,window,p - self.bottomright.rect().o);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                SplitterHit::Nothing => {
                    false
                },
                SplitterHit::TopLeft => {
                    let result = self.topleft.mousemove(ui,window,p - self.topleft.rect().o);
                    self.capturing.set(result);
                    result
                },
                SplitterHit::Separator => {
                    true
                },
                SplitterHit::BottomRight => {
                    let result = self.bottomright.mousemove(ui,window,p - self.bottomright.rect().o);
                    self.capturing.set(result);
                    result
                }
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
