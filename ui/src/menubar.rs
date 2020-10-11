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
    Menu(String,Rc<dyn Widget>),
    Separator,
}

pub struct CurrentMenu {
    n: usize,
    window: Rc<UIWindow>,
}

/// Menu bar.
pub struct MenuBar {
    ui: Rc<UI>,
    style: RefCell<MenuBarStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    items: Vec<MenuBarItem>,
    current_menu: RefCell<Option<CurrentMenu>>,
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
            items: items,
            current_menu: RefCell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> MenuBarHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    r.set_sx(size.x());
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

    pub fn open_menu(&self,ui: &UI,window: &Rc<UIWindow>,n: usize) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name,menu) => {
                    let size = style.font.measure(&name);
                    r.set_sx(size.x());
                    if i == n {
                        let mut current_menu = self.current_menu.borrow_mut();
                        if let Some(cm) = &*current_menu {
                            cm.window.close();
                            *current_menu = None;
                        }
                        let size = menu.calc_min_size();
                        let global_offset = window.window.r.get().o() + ui.offset.get();
                        let mr = rect!(global_offset + vec2!(r.ox(),r.oy() + r.sy()),size);
                        if let Ok(window) = UIWindow::new_popup(&self.ui,mr,window,menu) {
                            *current_menu = Some(CurrentMenu { n: n,window: window, });
                            break;
                        }
                        else {
                            break;
                        }
                    }
                    r.set_ox(r.ox() + size.x());
                },
                MenuBarItem::Separator => {
                    r.set_sx(MENUBAR_SEPARATOR_WIDTH);
                    r.set_ox(r.ox() + MENUBAR_SEPARATOR_WIDTH);
                },
            }                
        }
    }

    pub fn close_menu(&self,ui: &UI,window: &Rc<UIWindow>) {
        let mut current_menu = self.current_menu.borrow_mut();
        if let Some(cm) = &*current_menu {
            cm.window.close();
            *current_menu = None;
        }
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
            let mut color = style.item_color;
            if let Some(cm) = &*self.current_menu.borrow() {
                if cm.n == i {
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

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        match self.hit.get() {
            MenuBarHit::Nothing => {
                false
            },
            MenuBarHit::Item(n) => {
                let current_n = if let Some(cm) = &*self.current_menu.borrow() { Some(cm.n) } else { None };
                if let Some(cmn) = current_n {
                    if cmn == n {
                        self.close_menu(ui,window);
                    }
                    else {
                        self.open_menu(ui,window,n);
                    }
                }
                else {
                    self.open_menu(ui,window,n);
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
                let current_n = if let Some(cm) = &*self.current_menu.borrow() { Some(cm.n) } else { None };
                if let Some(cmn) = current_n {
                    if cmn != n {
                        self.open_menu(ui,window,n);
                    }
                }
                false
            },
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
