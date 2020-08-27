// E - UI - Book
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Book hit test possibilities.
#[derive(Copy,Clone,Debug)]
pub enum BookHit {

    /// Mouse is somewhere else.
    Outside,

    /// Mouse is over a tab.
    Tab(usize),

    /// Mouse is over the page area.
    Page,
}

/// Tab book widget with pages.
pub struct Book {

    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Hit state.
    hit: Cell<BookHit>,

    /// Pages in the book.
    pages: RefCell<Vec<(String,Rc<dyn ui::Widget>)>>,

    /// (temporary) Index to current page in the book.
    pub current_index: Cell<usize>,

    /// Padding around the book.
    pub padding: Cell<Vec2<i32>>,

    /// Padding around the tab titles.
    pub inner_padding: Cell<Vec2<i32>>,

    /// Font for tab text.
    pub font: RefCell<Rc<ui::Font>>,

    /// Tab text color.
    pub color: Cell<u32>,

    /// Tab background color.
    pub tab_color: Cell<u32>,

    /// Tab background color when mouse hovers over it.
    pub hover_tab_color: Cell<u32>,

    /// Background color in empty part next to tabs.
    pub tab_back_color: Cell<u32>,
}

impl Book {
    pub fn new_from_vec(ui: &Rc<ui::UI>,pages: Vec<(String,Rc<dyn ui::Widget>)>) -> Result<Book,SystemError> {
        // upgrade the page list
        let mut new_pages: Vec<(String,Rc<dyn ui::Widget>)> = Vec::new();
        for page in pages.iter() {
            new_pages.push((String::clone(&page.0),Rc::clone(&page.1)));
        }
        Ok(Book {
            ui: Rc::clone(ui),
            hit: Cell::new(BookHit::Outside),
            pages: RefCell::new(new_pages),
            current_index: Cell::new(0),
            padding: Cell::new(vec2!(0,0)),
            inner_padding: Cell::new(vec2!(8,4)),
            font: RefCell::new(Rc::clone(&ui.font)),
            color: Cell::new(0xFFFFFFFF),
            tab_color: Cell::new(0xFF001133),
            hover_tab_color: Cell::new(0xFF002266),
            tab_back_color: Cell::new(0xFF000000),
        })
    }
}

impl ui::Widget for Book {
    fn measure(&self) -> Vec2<i32> {
        // measure size of tab bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let font = self.font.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        let mut tabbar_size = vec2!(0i32,0i32);
        let pages = self.pages.borrow();
        for page in pages.iter() {
            let title = &page.0;
            let tab_size = font.measure(&title) + 2 * inner_padding;
            if tab_size.y > tabbar_size.y {
                tabbar_size.y = tab_size.y;
            }
            tabbar_size.x += tab_size.x;
        }

        // measure largest page
        let mut page_size = vec2!(0i32,0i32);
        let pages = self.pages.borrow();
        for page in pages.iter() {
            let size = page.1.measure();
            if size.x > page_size.x {
                page_size.x = size.x;
            }
            if size.y > page_size.y {
                page_size.y = size.y;
            }
        }

        // combine both
        let mut book_size = tabbar_size;
        if page_size.x > book_size.x {
            book_size.x = page_size.x;
        }
        book_size.y += page_size.y;

        book_size + 2 * padding
    }

    fn handle(&self,event: &Event,space: Rect<i32>) {

        let pages = self.pages.borrow();
        let font = self.font.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let current_index = self.current_index.get();

        // calculate page rect
        let mut page_rect = space;
        let mut tab_size = vec2!(0i32,0i32);
        if pages.len() > 0 {
            tab_size = font.measure(&pages[0].0) + 2 * inner_padding;
        }
        page_rect.o.y += tab_size.y;
        page_rect.s.y -= tab_size.y;

        // hit test
        match event {
            Event::MousePress(pos,_) | Event::MouseRelease(pos,_) | Event::MouseMove(pos) => {
                let mut hit = BookHit::Outside;
                if space.contains(pos) {
                    let mut tab_rect = rect!(0i32,0i32,0i32,0i32);
                    tab_rect.o = space.o + padding;
                    let mut i = 0usize;
                    for page in pages.iter() {
                        let title = &page.0;
                        tab_rect.s = font.measure(&title) + 2 * inner_padding;
                        if tab_rect.contains(pos) {
                            hit = BookHit::Tab(i);
                            break;
                        }
                        tab_rect.o.x += tab_rect.s.x;
                        i += 1;
                    }
                    if pos.y >= tab_rect.o.y + tab_rect.s.y {
                        hit = BookHit::Page;
                    }
                }
                self.hit.set(hit);
            },

            _ => { },
        }

        // handle event
        // TODO: not sure if this is going to be the sensible way in the end...
        let hit = self.hit.get();
        match hit {

            // pass down to current page, if any
            BookHit::Page => {
                if pages.len() > 0 {
                    if current_index < pages.len() {
                        pages[current_index].1.handle(event,page_rect);
                    }
                }    
            },

            // click on tab
            BookHit::Tab(i) => {
                if let Event::MousePress(_,mouse) = event {
                    if let Mouse::Left = mouse {
                        self.current_index.set(i);
                    }
                }
            },

            _ => { },
        }
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {

        let pages = self.pages.borrow();
        let font = self.font.borrow();
        let hit = self.hit.get();
        let color = self.color.get();
        let tab_color = self.tab_color.get();
        let hover_tab_color = self.hover_tab_color.get();
        let tab_back_color = self.tab_back_color.get();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let current_index = self.current_index.get();

        // calculate page rect
        let mut page_rect = space;
        let mut tab_size = vec2!(0i32,0i32);
        if pages.len() > 0 {
            tab_size = font.measure(&pages[0].0) + 2 * inner_padding;
        }
        page_rect.o.y += tab_size.y;
        page_rect.s.y -= tab_size.y;

        // draw tab bar
        let mut tab_rect = rect!(0i32,0i32,0i32,0i32);
        tab_rect.o = space.o + padding;
        let mut i = 0usize;
        for page in pages.iter() {
            let title = &page.0;
            tab_rect.s = font.measure(&title) + 2 * inner_padding;
            let mut tc = tab_color;
            if let BookHit::Tab(n) = hit {
                if n == i {
                    tc = hover_tab_color;
                }
            }
            self.ui.draw_rectangle(canvas_size,tab_rect,tc,gpu::BlendMode::Replace);
            self.ui.draw_text(canvas_size,tab_rect.o + inner_padding,&title,color,&font);
            tab_rect.o.x += tab_rect.s.x;
            i += 1;
        }
        self.ui.draw_rectangle(canvas_size,rect!(tab_rect.o,vec2!(space.s.x - tab_rect.o.x,tab_rect.s.y)),tab_back_color,gpu::BlendMode::Replace);

        // draw current page
        if pages.len() > 0 {
            if current_index < pages.len() {
                pages[current_index].1.draw(canvas_size,page_rect);
            }
        }
    }
}
