// E - UI - Book
// Desmond Germans, 2020

// A book is an area with tabs at the top. The currently selected tab dictates
// the widgets in the rest of the area.

use {
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

#[derive(Copy,Clone)]
pub enum BookHit {
    Nothing,
    Tab(usize),
    Page,
}

pub struct BookItem {
    name: String,
    child: Rc<dyn Widget>,
}

/// Book.
pub struct Book {
    r: Cell<Rect<i32>>,
    hit: Cell<BookHit>,
    items: Vec<BookItem>,
    current: Cell<Option<usize>>,
}

impl Book {
    pub fn new(items: Vec<BookItem>) -> Result<Book,SystemError> {
        Ok(Book {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(BookHit::Nothing),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> BookHit {
        let styles = draw.styles.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let size = styles.font.measure(&item.name);
            r.set_s(size);
            if r.contains(&p) {
                return BookHit::Tab(i);
            }
            r.set_ox(r.ox() + size.x());
        }
        r.set_o(vec2!(0,r.sy()));
        r.set_s(self.r.get().s() - vec2!(0,r.oy()));
        if r.contains(&p) {
            return BookHit::Page;
        }
        BookHit::Nothing
    }
}

impl Widget for Book {
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
            let size = styles.font.measure(&item.name);
            total_size += vec2!(size.x(),0);
            if size.y() > total_size.y() {
                total_size.set_y(size.y());
            }
        }
        let mut page_size = vec2!(0i32,0i32);
        for item in self.items.iter() {
            let size = item.child.calc_min_size(draw);
            if size.x() > page_size.x() {
                page_size.set_x(size.x());
            }
            if size.y() > page_size.y() {
                page_size.set_y(size.y());
            }
        }
        if page_size.x() > total_size.x() {
            total_size.set_x(page_size.x());
        }
        total_size += vec2!(0,page_size.y());
        total_size
    }

    fn draw(&self,draw: &Draw) {
        let styles = draw.styles.borrow();
        let widget_size = self.r.get().s();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.items.len() {
            let item = &self.items[i];
            let size = styles.font.measure(&item.name);
            let mut color = if let BookHit::Tab(n) = self.hit.get() {
                if n == i {
                    styles.book_tab_hover_color
                }
                else {
                    styles.book_tab_color
                }
            } else {
                if let Some(n) = self.current.get() {
                    if n == i {
                        styles.book_tab_current_color
                    }
                    else {
                        styles.book_tab_color
                    }
                }
                else {
                    styles.book_tab_color
                }
            };
            draw.draw_rectangle(r,color,BlendMode::Replace);
            draw.draw_text(r.o(),&item.name,styles.book_tab_text_color,&styles.font);
            r.set_ox(r.ox() + size.x());
        }
        r.set_o(vec2!(0,r.oy() + styles.font.measure("E").y()));
        r.set_s(self.r.get().s() - vec2!(0,r.oy()));
        if let Some(n) = self.current.get() {
            // TODO: localize
            self.items[n].child.draw(draw);
            // TODO: unlocalize
        }
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {
                // TODO: if page is capturing, pass down
            },
            Event::MouseRelease(p,b) => {
                // TODO: if page is capturing, pass down
            },
            Event::MouseMove(p) => {
                // TODO: if page is capturing, pass down, otherwise
                self.hit.set(self.find_hit(draw,p));
                if let BookHit::Page = self.hit.get() {
                    // TODO: pass down
                }
            },
            _ => { },
        }
    }
}