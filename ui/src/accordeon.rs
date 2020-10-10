// E - UI - Accordeon
// Desmond Germans, 2020

// An Accordeon is a group of horizontal or vertical tabs of which one is
// open, showing the widget contents. Very similar to a Book.

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
pub enum AccordeonHit {
    Nothing,
    Tab(usize),
    Page,
}

/// Accordeon style.
pub struct AccordeonStyle {
    pub font: Rc<Font>,
    pub tab_text_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
    pub tab_current_color: u32,
}

/// Accordeon item.
pub struct AccordeonItem {
    pub name: String,
    pub child: Rc<dyn Widget>,
}

/// Accordeon.
pub struct Accordeon {
    ui: Rc<UI>,
    style: RefCell<AccordeonStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<AccordeonHit>,
    capturing: Cell<bool>,
    items: Vec<AccordeonItem>,
    current: Cell<Option<usize>>,
}

impl Accordeon {
    pub fn new(ui: &Rc<UI>,items: Vec<AccordeonItem>) -> Result<Accordeon,SystemError> {
        Ok(Accordeon {
            ui: Rc::clone(&ui),
            style: RefCell::new(AccordeonStyle {
                font: Rc::clone(&ui.font),
                tab_text_color: 0xAAAAAA,
                tab_color: 0x444444,
                tab_hover_color: 0x224488,
                tab_current_color: 0x193366,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(AccordeonHit::Nothing),
            capturing: Cell::new(false),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> AccordeonHit {
        AccordeonHit::Nothing  // Should be Tab(i) or Page, once we know which page is open
    }
}

impl Widget for Accordeon {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        // TODO: recalculate all accordeon items; make the closed items 0 size, but the one open item maximum size
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            let size = style.font.measure(&item.name);
            if size.x() > total_size.x() {
                total_size.set_x(size.x());
            }
            total_size += vec2!(0,size.y());
        }
        let mut page_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            let size = item.child.calc_min_size();
            if size.x() > page_size.x() {
                page_size.set_x(size.x());
            }
            if size.y() > page_size.y() {
                page_size.set_y(size.y());
            }
        }
        if page_size.x() > total_size.x() {
            total_size.set_x(page_size.x());
        }
        total_size += vec2!(0,page_size.y());
        total_size
    }

    fn draw(&self) {
        let styles = self.style.borrow();
        let widget_size = self.r.get().s();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let size = styles.font.measure(&item.name);
            let mut color = if let AccordeonHit::Tab(n) = self.hit.get() {
                if n == i {
                    styles.tab_hover_color
                }
                else {
                    styles.tab_color
                }
            } else {
                if let Some(n) = self.current.get() {
                    if n == i {
                        styles.tab_current_color
                    }
                    else {
                        styles.tab_color
                    }
                }
                else {
                    styles.tab_color
                }
            };
            self.ui.draw_rectangle(r,color,BlendMode::Replace);
            self.ui.draw_text(r.o(),&item.name,styles.tab_text_color,&styles.font);
            r.set_oy(r.oy() + size.y());
            if let Some(n) = self.current.get() {
                if n == i {
                    self.ui.delta_offset(r.o());
                    self.items[i].child.draw();
                    self.ui.delta_offset(-r.o());
                }
            }
            r.set_oy(r.oy() + self.items[i].child.rect().sy());
        }
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                AccordeonHit::Tab(n) => {
                    true
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mousepress(ui,window,p - self.items[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mousepress: currentless Page");
                        self.capturing.set(false);
                        false
                    }
                },
            }
        }
        else {
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    false
                },
                AccordeonHit::Tab(n) => {
                    println!("Accordeon: start clicking on tab {}",n);
                    self.capturing.set(true);
                    true
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mousepress(ui,window,p - self.items[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mousepress: currentless Page");
                        false
                    }
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                AccordeonHit::Tab(n) => {
                    println!("Accordeon: stop clicking on tab {}",n);
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mouserelease(ui,window,p - self.items[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mouserelease: currentless Page");
                        self.capturing.set(false);
                        false
                    }
                },
            }
        }
        else {
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    false
                },
                AccordeonHit::Tab(n) => {
                    false
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mouserelease(ui,window,p - self.items[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mouserelease: currentless Page");
                        false
                    }
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                AccordeonHit::Tab(n) => {
                    println!("Accordeon: still clicking on tab {}",n);
                    true
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mousemove(ui,window,p - self.items[n].child.rect().o());
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mousemove: currentless Page");
                        self.capturing.set(false);
                        false
                    }
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                AccordeonHit::Nothing => {
                    false
                },
                AccordeonHit::Tab(n) => {
                    true
                },
                AccordeonHit::Page => {
                    if let Some(n) = self.current.get() {
                        let result = self.items[n].child.mousemove(ui,window,p - self.items[n].child.rect().o());
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Accordeon: mousemove: currentless Page");
                        false
                    }
                },
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}