// E - UI - Book
// Desmond Germans, 2020

// A book is an area with tabs at the top. The currently selected tab dictates
// the widgets in the rest of the area.

use {
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
        rc::Rc,
    },
};

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum BookHit {
    Nothing,
    Tab(usize),
    Page,
}

/// Book style.
pub struct BookStyle {
    pub font: Rc<Font>,
    pub tab_text_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
    pub tab_current_color: u32,
    pub tab_background_color: u32,
}

/// Book page.
pub struct BookPage {
    pub name: String,
    pub child: Rc<dyn Widget>,
}

/// Book.
pub struct Book {
    ui: Rc<UI>,
    style: RefCell<BookStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<BookHit>,
    capturing: Cell<bool>,
    pages: Vec<BookPage>,
    page: Cell<Option<usize>>,
}

impl Book {
    pub fn new(ui: &Rc<UI>,pages: Vec<BookPage>) -> Result<Book,SystemError> {
        Ok(Book {
            ui: Rc::clone(&ui),
            style: RefCell::new(BookStyle {
                font: Rc::clone(&ui.font),
                tab_text_color: 0xAAAAAA,
                tab_color: 0x444444,
                tab_hover_color: 0x224488,
                tab_current_color: 0x112244,
                tab_background_color: 0x111111,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(BookHit::Nothing),
            capturing: Cell::new(false),
            pages: pages,
            page: Cell::new(None),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> BookHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.pages.len() {
            let item = &self.pages[i];
            let size = style.font.measure(&item.name);
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

    pub fn set_page(&self,page: usize) {
        self.page.set(Some(page));
    }
}

impl Widget for Book {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        let style = self.style.borrow();
        let std_text_size = style.font.measure("E");
        let page_rect = rect!(vec2!(0,std_text_size.y()),vec2!(self.r.get().sx(),self.r.get().sy() - std_text_size.y()));
        for page in self.pages.iter() {
            page.child.set_rect(page_rect);
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.pages.iter() {
            let size = style.font.measure(&item.name);
            total_size += vec2!(size.x(),0);
            if size.y() > total_size.y() {
                total_size.set_y(size.y());
            }
        }
        let mut page_size = vec2!(0i32,0i32);
        for item in self.pages.iter() {
            let size = item.child.calc_min_size();
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

    fn draw(&self) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.pages.len() {
            let item = &self.pages[i];
            let size = style.font.measure(&item.name);
            r.set_s(size);
            let color = if let BookHit::Tab(n) = self.hit.get() {
                if n == i {
                    style.tab_hover_color
                }
                else {
                    style.tab_color
                }
            } else {
                if let Some(n) = self.page.get() {
                    if n == i {
                        style.tab_current_color
                    }
                    else {
                        style.tab_color
                    }
                }
                else {
                    style.tab_color
                }
            };
            self.ui.draw_rectangle(r,color,BlendMode::Replace);
            self.ui.draw_text(r.o(),&item.name,style.tab_text_color,&style.font);
            r.set_ox(r.ox() + size.x());
        }
        r.set_o(vec2!(0,r.oy() + style.font.measure("E").y()));
        r.set_s(self.r.get().s() - vec2!(0,r.oy()));
        if let Some(n) = self.page.get() {
            let offset = r.o();
            self.ui.delta_offset(offset);
            self.pages[n].child.draw();
            self.ui.delta_offset(-offset);
        }
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                BookHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                BookHit::Tab(n) => {
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousepress(ui,window,p - self.pages[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        self.capturing.set(false);
                        false
                    }
                }
            }
        }
        else {
            match self.hit.get() {
                BookHit::Nothing => {
                    false
                },
                BookHit::Tab(n) => {
                    println!("Book: start clicking on tab {}",n);
                    self.set_page(n);
                    self.capturing.set(true);
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousepress(ui,window,p - self.pages[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        false
                    }
                }
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                BookHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                BookHit::Tab(n) => {
                    println!("Book: stop clicking on tab {}",n);
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mouserelease(ui,window,p - self.pages[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        self.capturing.set(false);
                        false
                    }
                }
            }
        }
        else {
            match self.hit.get() {
                BookHit::Nothing => {
                    false
                },
                BookHit::Tab(n) => {
                    false
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mouserelease(ui,window,p - self.pages[n].child.rect().o(),b);
                        self.capturing.set(result);
                        result
                    }
                    else {
                        false
                    }
                }
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                BookHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                BookHit::Tab(n) => {
                    println!("Book: still clicking on tab {}",n);
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousemove(ui,window,p - self.pages[n].child.rect().o());
                        self.capturing.set(result);
                        result
                    }
                    else {
                        false
                    }
                }
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                BookHit::Nothing => {
                    false
                },
                BookHit::Tab(n) => {
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousemove(ui,window,p - self.pages[n].child.rect().o());
                        self.capturing.set(result);
                        result
                    }
                    else {
                        println!("Book: mousemove: currentless Page");
                        false
                    }
                }
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}