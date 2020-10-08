// E - UI - Book
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::{
        Cell,
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
    pub widget: Box<dyn Widget>,
}

/// Tab book widget with pages.
pub struct Book {
    ui: Rc<UI>,
    r: Cell<Rect<i32>>,
    hit: Cell<BookHit>,
    pages: Vec<Page>,
    pub current_index: Cell<usize>,
    pub page_capturing: Cell<bool>,
    pub padding: Cell<Vec2<i32>>,
    pub inner_padding: Cell<Vec2<i32>>,
}

impl Book {
    pub fn new_from_vec(ui: &Rc<UI>,pages: Vec<Page>) -> Result<Book,SystemError> {
        Ok(Book {
            ui: Rc::clone(ui),
            r: Cell::new(Rect::<i32>::zero()),
            hit: Cell::new(BookHit::Outside),
            pages: pages,
            current_index: Cell::new(0),
            page_capturing: Cell::new(false),
            padding: Cell::new(Vec2::<i32>::zero()),
            inner_padding: Cell::new(vec2!(8,4)),
        })
    }

    fn calc_tab_bar_size(&self) -> Vec2<i32> {
        let styles = self.ui.state.styles.borrow();
        let inner_padding = self.inner_padding.get();
        let mut total = Vec2::<i32>::zero();
        for page in self.pages.iter() {
            let tab_size = styles.font.measure(&page.name) + 2 * inner_padding;
            if tab_size.y() > total.y() {
                total.set_y(tab_size.y());
            }
            total.set_x(total.x() + tab_size.x());
        }
        total
    }
}

impl Widget for Book {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);

        //let padding = self.padding.get();

        let tab_bar_size = self.calc_tab_bar_size();

        let mut pr = self.r.get();
        let delta = vec2!(0,tab_bar_size.y());
        pr.set_o(pr.o() + delta);
        pr.set_s(pr.s() - delta);
        for page in self.pages.iter() {
            page.widget.set_rect(pr);
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        // measure size of tab bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let padding = self.padding.get();

        let tab_bar_size = self.calc_tab_bar_size();

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

    fn draw(&self) {

        let hit = self.hit.get();
        let styles = self.ui.state.styles.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let current_index = self.current_index.get();

        // draw tab bar
        let mut tab_rect = Rect::<i32>::zero();
        tab_rect.set_o(padding);
        for i in 0..self.pages.len() {
            let page = &self.pages[i];
            tab_rect.set_s(styles.font.measure(&page.name) + 2 * inner_padding);
            let mut tc = styles.book_tab_color;
            if let BookHit::Tab(n) = hit {
                if n == i {
                    tc = styles.book_tab_hover_color;
                }
            }
            self.ui.state.draw_rectangle(tab_rect,tc,BlendMode::Replace);
            self.ui.state.draw_text(tab_rect.o() + inner_padding,&page.name,styles.book_tab_text_color,&styles.font);
            tab_rect.set_ox(tab_rect.ox() + tab_rect.sx());
        }
        self.ui.state.draw_rectangle(rect!(
            tab_rect.o(),
            vec2!(
                self.r.get().sx() - tab_rect.ox(),
                tab_rect.sy()
            )
        ),styles.book_tab_background_color,BlendMode::Replace);

        // draw current page
        if (self.pages.len() > 0) && (current_index < self.pages.len()) {
            let pr = self.pages[current_index].widget.rect();
            self.ui.state.delta_offset(pr.o());
            self.pages[current_index].widget.draw();
            self.ui.state.delta_offset(-pr.o());
        }
    }

    fn handle_mouse_press(&self,p: Vec2<i32>,b: MouseButton) {
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let tab_bar_size = self.calc_tab_bar_size();
            page.widget.handle_mouse_press(p - vec2!(0,tab_bar_size.y()),b);
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
                        let tab_bar_size = self.calc_tab_bar_size();
                        page.widget.handle_mouse_press(p - vec2!(0,tab_bar_size.y()),b);
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
            let tab_bar_size = self.calc_tab_bar_size();
            page.widget.handle_mouse_release(p - vec2!(0,tab_bar_size.y()),b);
        }

        // otherwise, go by hit test
        else {
            match self.hit.get() {

                // over the page area, send to corresponding page
                BookHit::Page => {
                    let i = self.current_index.get();
                    if i < self.pages.len() {
                        let page = &self.pages[i];
                        let tab_bar_size = self.calc_tab_bar_size();
                        page.widget.handle_mouse_release(p - vec2!(0,tab_bar_size.y()),b);
                    }
                },
                _ => { },
            }
        }
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        let styles = self.ui.state.styles.borrow();

        // if the page area is capturing, just pass down the move
        if self.page_capturing.get() {
            let i = self.current_index.get();
            let page = &self.pages[i];
            let tab_bar_size = self.calc_tab_bar_size();
            page.widget.handle_mouse_move(p - vec2!(0,tab_bar_size.y()));
        }

        // update hit test

        // get resources
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        // if there are no pages, don't bother
        if self.pages.len() == 0 {
            self.hit.set(BookHit::Outside);
            return false;
        }

        //calculate height of the tab bar
        let tab_bar_height = (styles.font.measure(&self.pages[0].name) + 2 * inner_padding).y();

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
                let tab_bar_size = self.calc_tab_bar_size();
                return child.widget.handle_mouse_move(p - vec2!(0,tab_bar_size.y()));
            }

            // check each tab
            let mut tab_rect = Rect::<i32>::new_os(r.o(),Vec2::<i32>::zero());
            for i in 0..self.pages.len() {
                let page = &self.pages[i];
                tab_rect.set_s(styles.font.measure(&page.name) + 2 * inner_padding);
                if tab_rect.contains(&p) {

                    // over tab i
                    self.hit.set(BookHit::Tab(i));
                    return true;
                }
                tab_rect.set_ox(tab_rect.sx());
            }
            self.hit.set(BookHit::Outside);
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
