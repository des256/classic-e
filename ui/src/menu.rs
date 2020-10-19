// E - UI - Menu
// Desmond Germans, 2020

// A menu is a popup window containing a list of menu items. TBD.

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
enum MenuHit {
    Nothing,
    Item(usize),
}

/// Menu item.
pub enum MenuItem {
    Action(String),
    Menu(String,Rc<Menu>),
    Separator,
}

/// Menu.
pub struct Menu {
    ui: Rc<UI>,
    style: RefCell<style::Menu>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuHit>,
    items: Vec<MenuItem>,
    current_item: RefCell<Option<usize>>,
    pub popup: RefCell<Option<Rc<UIWindow>>>,
}

const MENU_SEPARATOR_HEIGHT: i32 = 10;

impl Menu {
    pub fn new(ui: &Rc<UI>,items: Vec<MenuItem>) -> Result<Rc<Menu>,SystemError> {
        let menu = Rc::new(Menu {
            ui: Rc::clone(&ui),
            style: RefCell::new(style::Menu {
                font: Rc::clone(&ui.font),
                item_text_color: 0xAAAAAA,
                item_disabled_text_color: 0x666666,
                item_color: 0x444444,
                item_hover_color: 0x224488,
                item_disabled_color: 0x333333,
                item_current_color: 0x3366CC,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(MenuHit::Nothing),
            items: items,
            current_item: RefCell::new(None),
            popup: RefCell::new(None),
        });
        let menu_popup_ref = Rc::clone(&menu);
        *menu.popup.borrow_mut() = Some(UIWindow::new_popup(&ui,rect!(0,0,1,1),menu_popup_ref as Rc<dyn Widget>)?);
        Ok(menu)

        // NOTE: in order to close all leaks, first do *menu.popup.borrow_mut() = None
    }

    fn find_hit(&self,p: Vec2<i32>) -> MenuHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,self.r.get().s.x,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuItem::Action(name) => {
                    let size = style.font.measure(&name);
                    r.s.y = size.y;
                    if r.contains(&p) {
                        return MenuHit::Item(i);
                    }
                    r.o.y += size.y;
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    r.s.y = size.y;
                    if r.contains(&p) {
                        return MenuHit::Item(i);
                    }
                    r.o.y += size.y;
                },
                MenuItem::Separator => {
                    r.o.y += MENU_SEPARATOR_HEIGHT;
                },
            }
        }
        MenuHit::Nothing
    }
}

impl Widget for Menu {
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
                MenuItem::Action(name) => {
                    let size = style.font.measure(&name);
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                    total_size += vec2!(0,size.y);
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                    total_size += vec2!(0,size.y);
                },
                MenuItem::Separator => {
                    total_size += vec2!(0,MENU_SEPARATOR_HEIGHT);
                },
            }
        }
        total_size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,self.r.get().s.x,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let mut color = style.item_color;
            if let Some(n) = *self.current_item.borrow() {
                if n == i {
                    color = style.item_current_color;
                }
            }
            if let MenuHit::Item(n) = self.hit.get() {
                if n == i {
                    color = style.item_hover_color;
                }
            }
            let text_color = style.item_text_color;
            match item {
                MenuItem::Action(name) => {
                    let size = style.font.measure(&name);
                    r.s.y = size.y;
                    self.ui.draw.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw.draw_text(r.o,&name,text_color,&style.font);
                    r.o.y += size.y;
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    r.s.y = size.y;
                    self.ui.draw.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw.draw_text(vec2!(0,0),&name,text_color,&style.font);
                    r.o.y += size.y;
                },
                MenuItem::Separator => {
                    r.s.y = MENU_SEPARATOR_HEIGHT;
                    self.ui.draw.draw_rectangle(r,style.item_color,BlendMode::Replace);
                    r.o.y += MENU_SEPARATOR_HEIGHT;
                },
            }
        }
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        match self.hit.get() {
            MenuHit::Nothing => {
                false
            },
            MenuHit::Item(_n) => {
                // TODO: update the currently open submenu
                false
            },
        }
    }

    fn mouserelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        match self.hit.get() {
            MenuHit::Nothing => {
                false
            },
            MenuHit::Item(_n) => {
                false
            },
        }
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        self.hit.set(self.find_hit(p));
        match self.hit.get() {
            MenuHit::Nothing => {
                false
            },
            MenuHit::Item(n) => {
                let current_n = *self.current_item.borrow();
                if let Some(cn) = current_n {
                    if cn != n {
                        match &self.items[n] {
                            MenuItem::Action(_name) => {
                                *self.current_item.borrow_mut() = Some(n);
                            },
                            MenuItem::Menu(_name,_menu) => {
                                // TODO: open submenu
                            },
                            MenuItem::Separator => {
                            },
                        }
                    }
                }
                false
            },
        }
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
