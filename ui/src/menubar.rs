// E - UI - MenuBar
// Desmond Germans, 2020

// A menu bar is a horizontal bar with drop down menus.

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
pub enum MenuBarHit {
    Nothing,
    Item(usize),
}

/// Menu bar style.
pub struct MenuBarStyle {
    pub font: Rc<Font>,
    pub item_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
    pub item_current_color: u32,
}

/// Menu bar item.
pub enum MenuBarItem {
    Menu(String),
    Separator,
}

/// Menu bar.
pub struct MenuBar {
    ui: Rc<UI>,
    style: RefCell<MenuBarStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    capturing: Cell<bool>,
    items: Vec<MenuBarItem>,
    current: Cell<Option<usize>>,
}

const MENUBAR_SEPARATOR_WIDTH: i32 = 10;

impl MenuBar {
    pub fn new(ui: &Rc<UI>,items: Vec<MenuBarItem>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            ui: Rc::clone(&ui),
            style: RefCell::new(MenuBarStyle {
                font: Rc::clone(&ui.font),
                item_text_color: 0xAAAAAA,
                item_color: 0x444444,
                item_hover_color: 0x224488,
                item_current_color: 0x112244,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(MenuBarHit::Nothing),
            capturing: Cell::new(false),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> MenuBarHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name) => {
                    let size = style.font.measure(&name);
                    r.set_s(size);
                    if r.contains(&p) {
                        return MenuBarHit::Item(i);
                    }
                    r.set_ox(r.ox() + size.x());
                },
                MenuBarItem::Separator => {
                    r.set_ox(r.ox() + MENUBAR_SEPARATOR_WIDTH);
                },
            }
        }
        MenuBarHit::Nothing
    }
}

impl Widget for MenuBar {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            match item {
                MenuBarItem::Menu(name) => {
                    let size = style.font.measure(&name);
                    total_size += vec2!(size.x(),0);
                    if size.y() > total_size.y() {
                        total_size.set_y(size.y());
                    }
                },
                MenuBarItem::Separator => {
                    total_size += vec2!(MENUBAR_SEPARATOR_WIDTH,0);
                },
            }
        }
        total_size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let color = if let Some(n) = self.current.get() {
                if n == i {
                    style.item_current_color
                }
                else {
                    style.item_color
                }
            }
            else if let MenuBarHit::Item(n) = self.hit.get() {
                if n == i {
                    style.item_hover_color
                }
                else {
                    style.item_color
                }
            }
            else {
                style.item_color
            };
            let text_color = style.item_text_color;
            match item {
                MenuBarItem::Menu(name) => {
                    let size = style.font.measure(&name);
                    r.set_sx(size.x());
                    self.ui.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw_text(r.o(),&name,text_color,&style.font);
                    r.set_ox(r.ox() + size.x());
                },
                MenuBarItem::Separator => {
                    r.set_sx(MENUBAR_SEPARATOR_WIDTH);
                    self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
                    r.set_ox(r.ox() + MENUBAR_SEPARATOR_WIDTH);
                },
            }
        }
        r.set_sx(self.r.get().sx() - r.ox());
        if r.sx() > 0 {
            self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
        }
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                MenuBarHit::Item(n) => {
                    true
                },
            }
        }
        else {
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    false
                },
                MenuBarHit::Item(n) => {
                    println!("MenuBar: start clicking on item {}",n);
                    self.capturing.set(true);
                    true
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                MenuBarHit::Item(n) => {
                    println!("MenuBar: stop clickin on item {}",n);
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
            }
        }
        else {
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    false
                },
                MenuBarHit::Item(n) => {
                    false
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                MenuBarHit::Item(n) => {
                    println!("MenuBar: still clicking on item {}",n);
                    true
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                MenuBarHit::Nothing => {
                    false
                },
                MenuBarHit::Item(n) => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}
