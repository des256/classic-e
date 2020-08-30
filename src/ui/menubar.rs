// E - UI - Book
// Desmond Germans, 2020

/*use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

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

    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Hit state.
    hit: Cell<MenuBarHit>,

    /// The items.
    items: RefCell<Vec<ui::MenuItem>>,

    /// Padding around the menubar items.
    pub inner_padding: Cell<Vec2<i32>>,

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

    fn test_hit(&self,pos: Vec2<i32>) -> MenuBarHit {

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
    fn measure(&self) -> Vec2<i32> {
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

    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
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