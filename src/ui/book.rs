// E - UI - Book
// Desmond Germans, 2020

/*use crate::*;
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

    /// Page is capturing the mouse.
    page_is_capturing: Cell<bool>,
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
            page_is_capturing: Cell::new(false),
        })
    }

    fn test_hit(&self,r: Rect<i32>,pos: Vec2<i32>) -> BookHit {

        let pages = self.pages.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();
        let font = self.font.borrow();

        if pages.len() == 0 {
            return BookHit::Outside;
        }
        let tab_bar_height = (font.measure(&pages[0].0) + 2 * inner_padding).y;

        r.o += padding;
        r.s -= 2 * padding;

        if r.contains(&pos) {
            let mut pr = r;
            pr.o.y += tab_bar_height;
            pr.s.y -= tab_bar_height;
            if pr.contains(&pos) {
                return BookHit::Page;
            }
            let mut tab_rect = rect!(r.o,vec2!(0,0));
            for i in 0..pages.len() {
                let page = &pages[i];
                let title = &page.0;
                tab_rect.s = font.measure(&title) + 2 * inner_padding;
                if tab_rect.contains(&pos) {
                    return BookHit::Tab(i);
                }
                tab_rect.o.x += tab_rect.s.x;
            }
        }

        BookHit::Outside
    }
}

impl ui::Widget for Book {
    fn measure(&self) -> Vec2<i32> {
        // measure size of tab bar
        // PROBLEM LATER: When the tab bar becomes too long, there should
        // be more lines below each other, or some sort of scrolling
        // mechanism to reach the tabs that don't fit.
        let pages = self.pages.borrow();
        let font = self.font.borrow();
        let padding = self.padding.get();
        let inner_padding = self.inner_padding.get();

        let mut tab_bar_size = vec2!(0i32,0i32);
        for page in pages.iter() {
            let title = &page.0;
            let tab_size = font.measure(&title) + 2 * inner_padding;
            if tab_size.y > tab_bar_size.y {
                tab_bar_size.y = tab_size.y;
            }
            tab_bar_size.x += tab_size.x;
        }

        // measure largest page
        let mut page_size = vec2!(0i32,0i32);
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
        let mut book_size = tab_bar_size;
        if page_size.x > book_size.x {
            book_size.x = page_size.x;
        }
        book_size.y += page_size.y;

        book_size + 2 * padding
    }

    fn draw(&self,r: Rect<i32>) {

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

        // draw tab bar
        let mut tab_rect = rect!(0i32,0i32,0i32,0i32);
        tab_rect.o = r.o + padding;
        for i in 0..pages.len() {
            let page = &pages[i];
            let title = &page.0;
            tab_rect.s = font.measure(&title) + 2 * inner_padding;
            let mut tc = tab_color;
            if let BookHit::Tab(n) = hit {
                if n == i {
                    tc = hover_tab_color;
                }
            }
            self.ui.draw_rectangle(tab_rect,tc,gpu::BlendMode::Replace);
            self.ui.draw_text(tab_rect.o + inner_padding,&title,color,&font);
            tab_rect.o.x += tab_rect.s.x;
        }
        self.ui.draw_rectangle(rect!(tab_rect.o,vec2!(r.s.x - tab_rect.o.x,tab_rect.s.y)),tab_back_color,gpu::BlendMode::Replace);

        // draw current page
        if (pages.len() > 0) && (current_index < pages.len()) {
            let mut pr = r;
            pr.o.y += tab_rect.s.y;
            pr.s.y -= tab_rect.s.y;
            pages[current_index].1.draw(pr);
        }
    }

    fn mouse_press(&self,pos: Vec2<i32>,button: Mouse) -> bool {
        let pages = self.pages.borrow();
        if pages.len() == 0 {
            return false;
        }
        let font = self.font.borrow();
        let current_index = self.current_index.get();
        let inner_padding = self.inner_padding.get();
        let tab_bar_height = (font.measure(&pages[0].0) + 2 * inner_padding).y;
        let mut pr = r;
        pr.o.y += tab_bar_height;
        pr.s.y -= tab_bar_height;
        if self.page_is_capturing.get() {
            if current_index < pages.len() {
                if pages[current_index].1.mouse_press(pos - pr.o,button) {
                    return true;
                }
            }
        }
        self.page_is_capturing.set(false);
        match self.hit.get() {
            BookHit::Tab(i) => {
                if let Mouse::Left = button {
                    self.current_index.set(i);
                    return true;
                }    
            },
            BookHit::Page => {
                if current_index < pages.len() {
                    if pages[current_index].1.mouse_press(pos - pr.o,button) {
                        self.page_is_capturing.set(true);
                        return true;
                    }
                }
            },
            _ => { },
        }
        false
    }

    fn mouse_release(&self,pos: Vec2<i32>,button: Mouse) -> bool {
        let pages = self.pages.borrow();
        let current_index = self.current_index.get();
        let page_rect = self.page_rect.get();
        if self.page_is_capturing.get() {
            if pages.len() > 0 {
                if current_index < pages.len() {
                    if pages[current_index].1.mouse_release(pos - page_rect.o,button) {
                        return true;
                    }
                }
            }
        }
        self.page_is_capturing.set(false);
        match self.hit.get() {
            BookHit::Tab(i) => {
                return true;
            },
            BookHit::Page => {
                if pages.len() > 0 {
                    if current_index < pages.len() {
                        if pages[current_index].1.mouse_release(pos - page_rect.o,button) {
                            self.page_is_capturing.set(true);
                            return true;
                        }
                    }
                }        
            },
            _ => { },
        }
        false
    }

    fn mouse_move(&self,pos: Vec2<i32>) -> bool {
        let pages = self.pages.borrow();
        let current_index = self.current_index.get();
        let page_rect = self.page_rect.get();
        if self.page_is_capturing.get() {
            if pages.len() > 0 {
                if current_index < pages.len() {
                    if pages[current_index].1.mouse_move(pos - page_rect.o) {
                        return true;
                    }
                }
            }
        }
        self.page_is_capturing.set(false);
        let hit = self.test_hit(pos);
        self.hit.set(hit);
        match hit {
            BookHit::Tab(i) => {
                return true;
            },
            BookHit::Page => {
                if pages.len() > 0 {
                    if current_index < pages.len() {
                        if pages[current_index].1.mouse_move(pos - page_rect.o) {
                            return true;
                        }
                    }
                }        
            },
            _ => { },
        }
        false
    }
}
*/