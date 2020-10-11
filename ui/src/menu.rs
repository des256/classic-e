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

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum MenuHit {
    Nothing,
    Item(usize),
}

/// Menu style.
pub struct MenuStyle {
    pub font: Rc<Font>,
    pub item_text_color: u32,
    pub item_disabled_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
    pub item_disabled_color: u32,
    pub item_current_color: u32,
}

/// Menu item.
pub enum MenuItem {
    Action(String),
    Menu(String,Menu),
    Separator,
}

/// Menu.
pub struct Menu {
    ui: Rc<UI>,
    style: RefCell<MenuStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuHit>,
    capturing: Cell<bool>,
    items: Vec<MenuItem>,
    current: Cell<Option<usize>>,
}

const MENU_SEPARATOR_HEIGHT: i32 = 10;

impl Menu {
    pub fn new(ui: &Rc<UI>,items: Vec<MenuItem>) -> Result<Menu,SystemError> {
        Ok(Menu {
            ui: Rc::clone(&ui),
            style: RefCell::new(MenuStyle {
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
            capturing: Cell::new(false),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> MenuHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,self.r.get().sx(),0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuItem::Action(name) => {
                    let size = style.font.measure(&name);
                    r.set_sy(size.y());
                    if r.contains(&p) {
                        return MenuHit::Item(i);
                    }
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    r.set_sy(size.y());
                    if r.contains(&p) {
                        return MenuHit::Item(i);
                    }
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Separator => {
                    r.set_oy(r.oy() + MENU_SEPARATOR_HEIGHT);
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
                    if size.x() > total_size.x() {
                        total_size.set_x(size.x());
                    }
                    total_size += vec2!(0,size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    if size.x() > total_size.x() {
                        total_size.set_x(size.x());
                    }
                    total_size += vec2!(0,size.y());
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
        let mut r = rect!(0i32,0i32,self.r.get().sx(),0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let mut color = style.item_color;
            if let MenuHit::Item(n) = self.hit.get() {
                if n == i {
                    color = style.item_hover_color;
                }
            }
            if let Some(n) = self.current.get() {
                if n == i {
                    color = style.item_current_color;
                }
            }
            let text_color = style.item_text_color;
            match item {
                MenuItem::Action(name) => {
                    let size = style.font.measure(&name);
                    r.set_sy(size.y());
                    self.ui.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw_text(r.o(),&name,text_color,&style.font);
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = style.font.measure(&name);
                    r.set_sy(size.y());
                    self.ui.draw_rectangle(r,color,BlendMode::Replace);
                    self.ui.draw_text(vec2!(0,0),&name,text_color,&style.font);
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Separator => {
                    r.set_sy(MENU_SEPARATOR_HEIGHT);
                    self.ui.draw_rectangle(r,style.item_color,BlendMode::Replace);
                    r.set_oy(r.oy() + MENU_SEPARATOR_HEIGHT);
                },
            }
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
        // TODO: if capturing, no change, otherwise:
        self.hit.set(self.find_hit(p));
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
