// E - UI - MenuBar
// Desmond Germans, 2020

// A menu bar is a horizontal bar with drop down menus.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum MenuBarHit {
    Nothing,
    Item(usize),
}

pub enum MenuBarItem {
    Menu(String,Menu),
    Separator,
}

/// Menu bar.
pub struct MenuBar {
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    items: Vec<MenuBarItem>,
    current: Cell<Option<usize>>,
}

const MENUBAR_SEPARATOR_WIDTH: i32 = 10;

impl MenuBar {
    pub fn new(items: Vec<MenuBarItem>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(MenuBarHit::Nothing),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> MenuBarHit {
        let styles = draw.styles.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            match item {
                MenuBarItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
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

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            match item {
                MenuBarItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
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

    fn draw(&self,draw: &Draw) {
        let styles = draw.styles.borrow();
        let mut r = rect!(0i32,0i32,0i32,self.r.get().sy());
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let color = if let Some(n) = self.current.get() {
                if n == i {
                    styles.menubar_item_current_color
                }
                else {
                    styles.menubar_item_color
                }
            }
            else if let MenuBarHit::Item(n) = self.hit.get() {
                if n == i {
                    styles.menubar_item_hover_color
                }
                else {
                    styles.menubar_item_color
                }
            }
            else {
                styles.menubar_item_color
            };
            let text_color = styles.menubar_item_text_color;
            match item {
                MenuBarItem::Menu(name,_) => {
                    let size = styles.font.measure(&name);
                    r.set_sx(size.x());
                    draw.draw_rectangle(r,color,BlendMode::Replace);
                    draw.draw_text(r.o(),&name,text_color,&styles.font);
                    r.set_ox(r.ox() + size.x());
                },
                MenuBarItem::Separator => {
                    r.set_sx(MENUBAR_SEPARATOR_WIDTH);
                    draw.draw_rectangle(r,styles.menubar_item_color,BlendMode::Replace);
                    r.set_ox(r.ox() + MENUBAR_SEPARATOR_WIDTH);
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
