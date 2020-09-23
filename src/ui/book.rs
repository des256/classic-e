// E - UI - Book
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::{
        Cell,
        RefCell,
    },
};

/// Book hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum BookHit {
    Outside,
    Tab(usize),
    Page,
}

pub struct Page {
    pub name: String,
    pub widget: Box<dyn ui::Widget>,
}

/// Tab book widget with pages.
pub struct Book {
    state: Rc<ui::UIState>,
    pub r: Cell<Rect<i32>>,
    pages: Vec<Page>,
    hit: Cell<BookHit>,
    pub current_index: Cell<usize>,
    pub padding: Cell<Vec2<i32>>,
    pub inner_padding: Cell<Vec2<i32>>,
    pub font: RefCell<Rc<ui::Font>>,
    pub color: Cell<u32>,
    pub tab_color: Cell<u32>,
    pub hover_tab_color: Cell<u32>,
    pub tab_back_color: Cell<u32>,
    pub page_capturing: Cell<bool>,
}

impl Book {
    pub fn new_from_vec(state: &Rc<ui::UIState>,pages: Vec<ui::Page>) -> Book {
        Book {
            state: Rc::clone(state),
            r: Cell::new(Rect::<i32>::zero()),
            pages: pages,
            hit: Cell::new(BookHit::Outside),
            current_index: Cell::new(0),
            padding: Cell::new(Vec2::<i32>::zero()),
            inner_padding: Cell::new(Vec2::<i32>::new(8,4)),
            font: RefCell::new(Rc::clone(&state.font)),
            color: Cell::new(0xFFFFFFFF),
            tab_color: Cell::new(0xFF001133),
            hover_tab_color: Cell::new(0xFF002266),
            tab_back_color: Cell::new(0xFF000000),
            page_capturing: Cell::new(false),
        }
    }
}

impl ui::Widget for Book {
    fn get_rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        // measure size of tab bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let font = self.font.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        let mut tab_bar_size = Vec2::<i32>::zero();
        for page in self.pages.iter() {
            let tab_size = font.measure(&page.name) + 2 * inner_padding;
            if tab_size.y() > tab_bar_size.y() {
                tab_bar_size.set_y(tab_size.y());
            }
            tab_bar_size.set_x(tab_bar_size.x() + tab_size.x());
        }

        // measure largest page
        let mut page_size = Vec2::<i32>::zero();
        for page in self.pages.iter() {
            let size = page.widget.calc_min_size();
            if size.x() > page_size.x() {
                page_size.set_x(size.x());
            }
            if size.y() > page_size.y() {
                page_size.set_y(size.y());
            }
        }

        // combine both
        let mut book_size = tab_bar_size;
        if page_size.x() > book_size.x() {
            book_size.set_x(page_size.x());
        }
        book_size.set_y(book_size.y() + page_size.y());

        book_size + 2 * padding
    }

    fn draw(&self,context: Vec2<i32>) {
        let font = self.font.borrow();
        let hit = self.hit.get();
        let color = self.color.get();
        let tab_color = self.tab_color.get();
        let hover_tab_color = self.hover_tab_color.get();
        let tab_back_color = self.tab_back_color.get();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let current_index = self.current_index.get();

        // draw tab bar
        let mut tab_rect = Rect::<i32>::zero();
        tab_rect.set_o(context + padding);
        for i in 0..self.pages.len() {
            let page = &self.pages[i];
            tab_rect.set_s(font.measure(&page.name) + 2 * inner_padding);
            let mut tc = tab_color;
            if let BookHit::Tab(n) = hit {
                if n == i {
                    tc = hover_tab_color;
                }
            }
            self.state.draw_rectangle(tab_rect,tc,gpu::BlendMode::Replace);
            self.state.draw_text(tab_rect.o() + inner_padding,&page.name,color,&font);
            tab_rect.set_ox(tab_rect.ox() + tab_rect.sx());
        }
        self.state.draw_rectangle(Rect::<i32>::new_os(
            tab_rect.o(),
            Vec2::<i32>::new(
                self.r.get().sx() - tab_rect.ox(),
                tab_rect.sy()
            )
        ),tab_back_color,gpu::BlendMode::Replace);

        // draw current page
        if (self.pages.len() > 0) && (current_index < self.pages.len()) {
            let mut pr = self.r.get();
            pr.set_oy(pr.oy() + tab_rect.sy());
            pr.set_sy(pr.sy() - tab_rect.sy());
            self.pages[current_index].widget.draw(pr.o());
        }
    }

    fn handle_mouse_press(&self,p: Vec2<i32>,b: MouseButton) {
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            page.widget.handle_mouse_press(p - r.o(),b);
        }

        // otherwise, go by hit test
        else {
            match self.hit.get() {

                // over a tab and a left-click here means setting the current index
                BookHit::Tab(i) => {
                    if let MouseButton::Left = b {
                        self.current_index.set(i);
                        return;
                    }
                },

                // over the page area, send to corresponding page
                BookHit::Page => {
                    let i = self.current_index.get();
                    if i < self.pages.len() {
                        let page = &self.pages[i];
                        let r = page.widget.get_rect();
                        page.widget.handle_mouse_press(p - r.o(),b);
                    }
                },

                _ => { },
            }    
        }
    }

    fn handle_mouse_release(&self,p: Vec2<i32>,b: MouseButton) {
        // if the page area is capturing, just pass down the mouse release
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            page.widget.handle_mouse_release(p - r.o(),b);
        }

        // otherwise, go by hit test
        else {
            match self.hit.get() {

                // over the page area, send to corresponding page
                BookHit::Page => {
                    let i = self.current_index.get();
                    if i < self.pages.len() {
                        let page = &self.pages[i];
                        let r = page.widget.get_rect();
                        page.widget.handle_mouse_release(p - r.o(),b);
                    }
                },
                _ => { },
            }
        }
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        // if the page area is capturing, just pass down the move
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            if page.widget.handle_mouse_move(p - r.o()) {
                return true;
            }
        }

        // update hit test

        // get resources
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let font = self.font.borrow();

        // if there are no pages, don't bother
        if self.pages.len() == 0 {
            self.hit.set(BookHit::Outside);
            return false;
        }

        //calculate height of the tab bar
        let tab_bar_height = (font.measure(&self.pages[0].name) + 2 * inner_padding).y();

        // inside or outside padding
        let mut r = self.r.get();
        r.set_o(r.o() + padding);
        r.set_s(r.s() - 2 * padding);
        if r.contains(&p) {

            // tab bar or page
            let mut pr = r;
            pr.set_oy(pr.oy() + tab_bar_height);
            pr.set_sy(pr.sy() - tab_bar_height);
            if pr.contains(&p) {
                // over the page, pass down to current widget
                self.hit.set(BookHit::Page);
                let i = self.current_index.get();
                let child = &self.pages[i];
                let r = child.widget.get_rect();
                return child.widget.handle_mouse_move(p - r.o());
            }

            // check each tab
            let mut tab_rect = Rect::<i32>::new_os(r.o(),Vec2::<i32>::zero());
            for i in 0..self.pages.len() {
                let page = &self.pages[i];
                tab_rect.set_s(font.measure(&page.name) + 2 * inner_padding);
                if tab_rect.contains(&p) {

                    // over tab i
                    self.hit.set(BookHit::Tab(i));
                    return true;
                }
                tab_rect.set_ox(tab_rect.sx());
            }
        }
        else {
            // outside padding
            self.hit.set(BookHit::Outside);
        }
        false
    }

    fn handle_mouse_wheel(&self,_w: MouseWheel) {
    }
}
