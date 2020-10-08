// E - UI - Menu
// Desmond Germans, 2020

// A menu is a popup window containing a list of menu items. TBD.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum MenuHit {
    Nothing,
    Item(usize),
}

pub enum MenuItem {
    Action(String),
    Menu(String,Menu),
    Separator,
}

/// Menu.
pub struct Menu {
    r: Cell<Rect<i32>>,
    hit: Cell<MenuHit>,
    items: Vec<MenuItem>,
    current: Cell<Option<usize>>,
}

const MENU_SEPARATOR_HEIGHT: i32 = 10;

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Result<Menu,SystemError> {
        Ok(Menu {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(MenuHit::Nothing),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> MenuHit {
        let styles = draw.styles.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuItem::Action(name) => {
                    let size = styles.font.measure(&name);
                    r.set_s(size);
                    if r.contains(&p) {
                        return MenuHit::Item(i);
                    }
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
                    r.set_s(size);
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

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            match item {
                MenuItem::Action(name) => {
                    let size = styles.font.measure(&name);
                    if size.x() > total_size.x() {
                        total_size.set_x(size.x());
                    }
                    total_size += vec2!(0,size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
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

    fn draw(&self,draw: &Draw) {
        let styles = draw.styles.borrow();
        let mut r = rect!(0i32,0i32,self.r.get().sx(),0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let color = if let Some(n) = self.current.get() {
                if n == i {
                    styles.menu_item_current_color
                }
                else {
                    styles.menu_item_color  // TBD if menu item is enabled or not
                }
            }
            else if let MenuHit::Item(n) = self.hit.get() {
                if n == i {
                    styles.menu_item_hover_color
                }
                else {
                    styles.menu_item_color // TBD if menu item is enabled or not
                }
            }
            else {
                styles.menu_item_color // TBD if menu item is enabled or not
            };
            let text_color = styles.menu_item_text_color;
            match item {
                MenuItem::Action(name) => {
                    let size = styles.font.measure(&name);
                    r.set_sy(size.y());
                    draw.draw_rectangle(r,color,BlendMode::Replace);
                    draw.draw_text(r.o(),&name,text_color,&styles.font);
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
                    r.set_sy(size.y());
                    draw.draw_rectangle(r,color,BlendMode::Replace);
                    draw.draw_text(vec2!(0,0),&name,text_color,&styles.font);
                    r.set_oy(r.oy() + size.y());
                },
                MenuItem::Separator => {
                    r.set_sy(MENU_SEPARATOR_HEIGHT);
                    draw.draw_rectangle(r,styles.menu_item_color,BlendMode::Replace);
                    r.set_oy(r.oy() + MENU_SEPARATOR_HEIGHT);
                },
            }
        }
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}
