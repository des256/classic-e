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
    Menu(String,Rc<Menu>),
    Separator,
}

/// Menu bar.
pub struct MenuBar {
    ui: Rc<UI>,
    style: RefCell<MenuBarStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    items: Vec<MenuBarItem>,
    current_item: Cell<Option<usize>>,
}

const MENUBAR_SEPARATOR_WIDTH: i32 = 10;

impl MenuBar {
    pub fn new(ui: &Rc<UI>,items: Vec<MenuBarItem>) -> Result<Rc<MenuBar>,SystemError> {
        Ok(Rc::new(MenuBar {
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
            items: items,
            current_item: Cell::new(None),
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> MenuBarHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().s.y);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    r.s.x = size.x;
                    if r.contains(&p) {
                        return MenuBarHit::Item(i);
                    }
                    r.o.x += size.x;
                },
                MenuBarItem::Separator => {
                    r.o.x += MENUBAR_SEPARATOR_WIDTH;
                },
            }
        }
        MenuBarHit::Nothing
    }

    pub fn set_current(&self,window: &Rc<UIWindow>,new_n: usize) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().s.y);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    r.s.x = size.x;
                    if i == new_n {
                        if let Some(n) = self.current_item.get() {
                            if n != new_n {
                                self.clear_current();
                                let gr = rect!(window.window.r.get().o + self.ui.offset.get() + r.o + vec2!(0,r.s.y),menu.calc_min_size());
                                if let Some(popup) = &*menu.popup.borrow() {
                                    popup.configure(gr);
                                    popup.show();    
                                }
                                self.current_item.set(Some(new_n));
                            }
                        }
                        else {
                            let gr = rect!(window.window.r.get().o + self.ui.offset.get() + r.o + vec2!(0,r.s.y),menu.calc_min_size());
                            if let Some(popup) = &*menu.popup.borrow() {
                                popup.configure(gr);
                                popup.show();    
                            }
                            self.current_item.set(Some(new_n));
                        }
                    }
                    r.o.x += size.x;
                },
                MenuBarItem::Separator => {
                    r.o.x += MENUBAR_SEPARATOR_WIDTH;
                },
            }
        }    
    }

    pub fn clear_current(&self) {
        if let Some(n) = self.current_item.get() {
            if let MenuBarItem::Menu(name,menu) = &self.items[n] {
                if let Some(popup) = &*menu.popup.borrow() {
                    popup.hide();
                }
            }
        }
        self.current_item.set(None);
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
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    total_size += vec2!(size.x,0);
                    if size.y > total_size.y {
                        total_size.y = size.y;
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
        let mut r = rect!(0i32,0i32,0i32,self.r.get().s.y);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let mut color = style.item_color;
            if let Some(n) = self.current_item.get() {
                if n == i {
                    color = style.item_current_color;
                }
            }
            if let MenuBarHit::Item(n) = self.hit.get() {
                if n == i {
                    color = style.item_hover_color;
                }
            }
            let text_color = style.item_text_color;
            match item {
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    r.s.x = size.x;
                    self.ui.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw_text(r.o,&name,text_color,&style.font);
                    r.o.x += size.x;
                },
                MenuBarItem::Separator => {
                    r.s.x = MENUBAR_SEPARATOR_WIDTH;
                    self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
                    r.o.x += MENUBAR_SEPARATOR_WIDTH;
                },
            }
        }
        r.s.x = self.r.get().s.x - r.o.x;
        if r.s.x > 0 {
            self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
        }
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        match self.hit.get() {
            MenuBarHit::Nothing => {
                self.clear_current();
                false
            },
            MenuBarHit::Item(n) => {
                if let Some(cn) = self.current_item.get() {
                    if cn == n {
                        self.clear_current();
                    }
                    else {
                        self.set_current(window,n);
                    }
                }
                else {
                    self.set_current(window,n);
                }
                false
            },
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        match self.hit.get() {
            MenuBarHit::Nothing => {
                false
            },
            MenuBarHit::Item(n) => {
                false
            },
        }
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        self.hit.set(self.find_hit(p));
        match self.hit.get() {
            MenuBarHit::Nothing => {
                false
            },
            MenuBarHit::Item(n) => {
                if let Some(cn) = self.current_item.get() {
                    self.set_current(window,n);
                }
                false
            },
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
