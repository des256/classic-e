// E - UI - MenuBar
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// MenuBar hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum MenuBarHit {

    /// Mouse is somewhere else.
    Outside,

    /// Mouse is over an item.
    Item(usize),
}

pub struct Menu {
}

impl Widget for Menu {
    fn rect(&self) -> Rect<i32> {
        rect!(0i32,0i32,0i32,0i32)
    }

    fn set_rect(&self,_r: Rect<i32>) {
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        vec2!(0i32,0i32)
    }

    fn draw(&self) {
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) -> bool {
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}

pub struct MenuItem {
    pub name: String,
    pub menu: Rc<Menu>,
}

/// MenuBar widget.
pub struct MenuBar {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    items: Vec<MenuItem>,
    open_menu: Cell<Option<(u64,usize)>>,
    pub padding: Vec2<i32>,
    pub inner_padding: Vec2<i32>,

}

impl MenuBar {
    pub fn new(ui: &Rc<UI>,items: Vec<MenuItem>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            ui: Rc::clone(ui),
            r: Cell::new(Rect::<i32>::zero()),
            hit: Cell::new(MenuBarHit::Outside),
            items: items,
            open_menu: Cell::new(None),
            padding: vec2!(0i32,0i32),
            inner_padding: vec2!(8i32,2i32),
        })
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
        let styles = self.ui.state.styles.borrow();
        let mut total = vec2!(0i32,0i32);
        for item in self.items.iter() {
            let item_size = styles.font.measure(&item.name) + 2 * self.inner_padding;
            total.set_x(total.x() + item_size.x());
            if item_size.y() > total.y() {
                total.set_y(item_size.y());
            }
        }
        total + 2 * self.padding
    }

    fn draw(&self) {
        let styles = self.ui.state.styles.borrow();
        let mut item_rect = rect!(self.padding,vec2!(0i32,0i32));
        for i in 0..self.items.len() {
            let item = &self.items[i];
            item_rect.set_s(styles.font.measure(&item.name) + 2 * self.inner_padding);
            let mut item_color = styles.menubar_item_color;
            if let MenuBarHit::Item(n) = self.hit.get() {
                if n == i {
                    item_color = styles.menubar_item_hover_color;
                }
            }
            self.ui.state.draw_rectangle(item_rect,item_color,BlendMode::Replace);
            self.ui.state.draw_text(item_rect.o() + self.inner_padding,&item.name,styles.menubar_text_color,&styles.font);
            item_rect.set_ox(item_rect.ox() + item_rect.sx());
        }
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,b: MouseButton) {
        let hit = self.hit.get();
        println!("press, hit = {:?}",hit);
        match hit {
            MenuBarHit::Outside => {
                if let Some((id,_i)) = self.open_menu.get() {
                    self.ui.close(id);
                    self.open_menu.set(None);
                }
            },
            MenuBarHit::Item(i) => {
                if let MouseButton::Left = b {
                    if let Some((id,_k)) = self.open_menu.get() {
                        self.ui.close(id);
                        self.open_menu.set(None);
                    }
                    let styles = self.ui.state.styles.borrow();
                    let mut item_rect = rect!(self.padding,vec2!(0i32,0i32));
                    for k in 0..self.items.len() {
                        let item = &self.items[k];
                        item_rect.set_s(styles.font.measure(&item.name) + 2 * self.inner_padding);
                        if i == k {
                            break;
                        }
                        item_rect.set_ox(item_rect.ox() + item_rect.sx());
                    }
                    println!("opening window from {}",item_rect);
                    let id = self.ui.open_popup(rect!(item_rect.ox(),item_rect.oy() + item_rect.sy(),100,100),&self.items[i].menu);
                    self.open_menu.set(Some((id,i)));
                    return;
                }
            },
        }
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        let styles = self.ui.state.styles.borrow();
        let mut item_rect = rect!(self.padding,vec2!(0i32,0i32));
        for i in 0..self.items.len() {
            let item = &self.items[i];
            item_rect.set_s(styles.font.measure(&item.name) + 2 * self.inner_padding);
            if item_rect.contains(&p) {
                self.hit.set(MenuBarHit::Item(i));
                return true;
            }
            item_rect.set_ox(item_rect.ox() + item_rect.sx());
        }
        self.hit.set(MenuBarHit::Outside);
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}


// E - UI - Book
// Desmond Germans, 2020

/*use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;


/// MenuBar widget.
pub struct MenuBar {

    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Hit state.
    hit: Cell<MenuBarHit>,

    /// The items.
    items: RefCell<Vec<ui::MenuItem>>,

    /// Padding around the menubar items.
    pub inner_padding: Cell<i32x2>,

    /// Font for menu item text.
    pub font: RefCell<Rc<ui::Font>>,

    /// Menu item text color.
    pub color: Cell<u32>,

    /// Menu item background color.
    pub item_color: Cell<u32>,

    /// Menu item background color when mouse hovers over it.
    pub hover_item_color: Cell<u32>,
}

impl MenuBar {
    pub fn new(ui: &Rc<ui::UI>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            ui: Rc::clone(ui),
            r: Cell::new(rect!(0,0,1,1)),
            hit: Cell::new(MenuBarHit::Outside),
            items: RefCell::new(Vec::new()),
            inner_padding: Cell::new(vec2!(8,4)),
            font: RefCell::new(Rc::clone(&ui.font)),
            color: Cell::new(0xFF000000),
            item_color: Cell::new(0xFFFFFFFF),
            hover_item_color: Cell::new(0xFFAACCFF),
        })
    }

    fn test_hit(&self,pos: i32x2) -> MenuBarHit {

        let items = self.items.borrow();
        let inner_padding = self.inner_padding.get();
        let font = self.font.borrow();

        let r = rect!(vec2!(0,0),self.r.get().s);
        if r.contains(&pos) {
            let mut item_rect = rect!(0i32,0i32,0i32,0i32);
            let mut i = 0usize;
            for item in items.iter() {
                item_rect.s = font.measure(&item.text) + 2 * inner_padding;
                if item_rect.contains(&pos) {
                    return MenuBarHit::Item(i);
                }
                item_rect.o.x += item_rect.s.x;
                i += 1;
            }
        }

        MenuBarHit::Outside
    }
}

impl ui::Widget for MenuBar {
    fn measure(&self) -> i32x2 {
        // measure size of menu bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let font = self.font.borrow();
        let inner_padding = self.inner_padding.get();

        let mut total_size = vec2!(0i32,0i32);
        let items = self.items.borrow();
        for item in items.iter() {
            let item_size = font.measure(&item.text) + 2 * inner_padding;
            if item_size.y > total_size.y {
                total_size.y = item_size.y;
            }
            total_size.x += item_size.x;
        }

        total_size
    }

    fn get_rect(&self) -> i32r {
        self.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.r.set(r);
    }

    fn draw(&self) {

        let r = self.r.get();
        let items = self.items.borrow();
        let font = self.font.borrow();
        let hit = self.hit.get();
        let color = self.color.get();
        let item_color = self.item_color.get();
        let hover_item_color = self.hover_item_color.get();
        let inner_padding = self.inner_padding.get();

        let mut item_rect = rect!(0i32,0i32,0i32,0i32);
        item_rect.o = r.o;
        let mut i = 0usize;
        for item in items.iter() {
            item_rect.s = font.measure(&item.text) + 2 * inner_padding;
            let mut tc = item_color;
            if let MenuBarHit::Item(n) = hit {
                if n == i {
                    tc = hover_item_color;
                }
            }
            self.ui.draw_rectangle(item_rect,tc,gpu::BlendMode::Replace);
            self.ui.draw_text(item_rect.o + inner_padding,&item.text,color,&font);
            item_rect.o.x += item_rect.s.x;
            i += 1;
        }
        self.ui.draw_rectangle(rect!(item_rect.o,vec2!(r.s.x - item_rect.o.x,item_rect.s.y)),item_color,gpu::BlendMode::Replace);
    }
}
*/