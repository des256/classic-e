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
/// MenuBar widget.
pub struct MenuBar {
    state: Rc<UIState>,
    r: Cell<Rect<i32>>,
    hit: Cell<MenuBarHit>,
    items: Vec<String>,
    pub padding: Vec2<i32>,
}

impl MenuBar {
    pub fn new(state: &Rc<UIState>,items: Vec<String>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            state: Rc::clone(state),
            r: Cell::new(Rect::<i32>::zero()),
            hit: Cell::new(MenuBarHit::Outside),
            items: items,
            padding: Vec2::<i32>::zero(),
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
        let styles = self.state.styles.borrow();
        styles.font.measure("TODO: menuitems") + 2 * self.padding
    }

    fn draw(&self) {
        let styles = self.state.styles.borrow();
        self.state.draw_text(self.padding,"TODO: menuitems",styles.menubar_text_color,&styles.font);
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {
    }

    fn handle_mouse_move(&self,_p: Vec2<i32>) {
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