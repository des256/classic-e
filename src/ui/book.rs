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
    pub padding: Cell<i32x2>,
    pub inner_padding: Cell<i32x2>,
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
            r: Cell::new(rect!(0,0,0,0)),
            pages: pages,
            hit: Cell::new(BookHit::Outside),
            current_index: Cell::new(0),
            padding: Cell::new(i32x2::zero()),
            inner_padding: Cell::new(i32x2::from_xy(8,4)),
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
    fn get_rect(&self) -> i32r {
        self.core.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.core.r.set(r);
    }

    fn calc_min_size(&self) -> i32x2 {
        // measure size of tab bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let font = self.font.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        let mut tab_bar_size = i32x2::zero();
        for child in self.core.children.iter() {
            let tab_size = font.measure(&child.name) + 2 * inner_padding;
            if *tab_size.y() > *tab_bar_size.y() {
                *tab_bar_size.y() = *tab_size.y();
            }
            *tab_bar_size.x() += *tab_size.x();
        }

        // measure largest page
        let mut page_size = i32x2::zero();
        for child in self.core.children.iter() {
            let size = child.widget.calc_min_size();
            if *size.x() > *page_size.x() {
                *page_size.x() = *size.x();
            }
            if *size.y() > *page_size.y() {
                *page_size.y() = *size.y();
            }
        }

        // combine both
        let mut book_size = tab_bar_size;
        if *page_size.x() > *book_size.x() {
            *book_size.x() = *page_size.x();
        }
        *book_size.y() += *page_size.y();

        book_size + 2 * padding
    }

    fn draw(&self,context: i32x2) {
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
        let mut tab_rect = i32r::zero();
        tab_rect.o = context + padding;
        for i in 0..self.pages.len() {
            let child = &self.pages[i];
            tab_rect.s = font.measure(&child.name) + 2 * inner_padding;
            let mut tc = tab_color;
            if let BookHit::Tab(n) = hit {
                if n == i {
                    tc = hover_tab_color;
                }
            }
            self.core.state.draw_rectangle(tab_rect,tc,gpu::BlendMode::Replace);
            self.core.state.draw_text(tab_rect.o + inner_padding,&child.name,color,&font);
            *tab_rect.o.x() += *tab_rect.s.x();
        }
        self.core.state.draw_rectangle(i32r::from_os(
            tab_rect.o,
            i32x2::from_xy(
                *self.core.r.get().s.x() - *tab_rect.o.x(),
                *tab_rect.s.y()
            )
        ),tab_back_color,gpu::BlendMode::Replace);

        // draw current page
        if (self.core.children.len() > 0) && (current_index < self.core.children.len()) {
            let mut pr = self.core.r.get();
            *pr.o.y() += *tab_rect.s.y();
            *pr.s.y() -= *tab_rect.s.y();
            self.core.children[current_index].widget.draw(pr.o);
        }
    }

    fn handle_mouse_press(&self,p: i32x2,b: MouseButton) {
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            page.widget.handle_mouse_press(p - r.o,b);
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
                        page.widget.handle_mouse_press(p - r.o,b);
                    }
                },

                _ => { },
            }    
        }
    }

    fn handle_mouse_release(&self,p: i32x2,b: MouseButton) {
        // if the page area is capturing, just pass down the mouse release
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            page.widget.handle_mouse_release(p - r.o,b);
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
                        page.widget.handle_mouse_release(p - r.o,b);
                    }
                },
                _ => { },
            }
        }
    }

    fn handle_mouse_move(&self,p: i32x2) -> bool {
        // if the page area is capturing, just pass down the move
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let r = page.widget.get_rect();
            if page.widget.handle_mouse_move(p - r.o) {
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
        let tab_bar_height = *(font.measure(&self.core.children[0].name) + 2 * inner_padding).y();

        // inside or outside padding
        let mut r = self.r.get();
        r.o += padding;
        r.s -= 2 * padding;
        if r.contains(&p) {

            // tab bar or page
            let mut pr = r;
            *pr.o.y() += tab_bar_height;
            *pr.s.y() -= tab_bar_height;
            if pr.contains(&p) {
                // over the page, pass down to current widget
                self.hit.set(BookHit::Page);
                let i = self.current_index.get();
                let child = &self.pages[i];
                let r = child.widget.get_rect();
                return child.widget.handle_mouse_move(p - r.o);
            }

            // check each tab
            let mut tab_rect = i32r::from_os(r.o,i32x2::zero());
            for i in 0..self.core.children.len() {
                let child = &self.core.children[i];
                tab_rect.s = font.measure(&child.name) + 2 * inner_padding;
                if tab_rect.contains(&p) {

                    // over tab i
                    self.hit.set(BookHit::Tab(i));
                    return true;
                }
                *tab_rect.o.x() = *tab_rect.s.x();
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
