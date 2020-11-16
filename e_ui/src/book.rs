// E - UI - Book
// Desmond Germans, 2020

// A book is an area with tabs at the top. The currently selected tab dictates
// the widgets in the page area under the tabs.

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

#[derive(Copy,Clone,Debug)]
enum BookHit {
    Nothing,
    Tab(usize),
    Page,
}

/// Book page.
pub struct BookPage {
    pub name: String,
    pub child: Rc<dyn Widget>,
    pub enabled: Cell<bool>,
}

impl BookPage {
    pub fn new(name: String,child: Rc<dyn Widget>) -> BookPage {
        BookPage {
            name: name,
            child: child,
            enabled: Cell::new(true),
        }
    }
}

/// Book.
pub struct Book {
    ui: Rc<UI>,
    style: RefCell<style::Book>,
    r: Cell<Rect<i32>>,
    hit: Cell<BookHit>,
    capturing: Cell<bool>,
    pages: Vec<BookPage>,
    page: Cell<Option<usize>>,
}

impl Book {
    /// Create new book widget.
    ///
    /// A book is an area with tabs at the top. The currently selected tab dictates
    /// the widgets in the page area under the tabs.
    ///
    /// **Arguments**
    ///
    /// * `ui` - UI context.
    /// * `pages` - Book pages.
    ///
    /// **Returns**
    ///
    /// New book widget.
    pub fn new(ui: &Rc<UI>,pages: Vec<BookPage>) -> Result<Rc<Book>,SystemError> {
        Ok(Rc::new(Book {
            ui: Rc::clone(&ui),
            style: RefCell::new(style::Book {
                font: Rc::clone(&ui.font),
                text_color: 0xAAAAAA,
                disabled_text_color: 0x888888,
                color: 0x444444,
                hover_color: 0x224488,
                current_color: 0x112244,
                background_color: 0x111111,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(BookHit::Nothing),
            capturing: Cell::new(false),
            pages: pages,
            page: Cell::new(None),
        }))
    }

    fn find_hit(&self,p: Vec2<i32>) -> BookHit {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.pages.len() {
            let item = &self.pages[i];
            let size = style.font.measure(&item.name);
            r.s = size;
            if r.contains(&p) {
                return BookHit::Tab(i);
            }
            r.o.x += size.x;
        }
        r.o = vec2!(0,r.s.y);
        r.s = self.r.get().s - vec2!(0,r.o.y);
        if r.contains(&p) {
            return BookHit::Page;
        }
        BookHit::Nothing
    }

    /// Set current page.
    ///
    /// **Arguments**
    ///
    /// `page` - New current page.
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
        let page_rect = rect!(vec2!(0,std_text_size.y),vec2!(self.r.get().s.x,self.r.get().s.y - std_text_size.y));
        for page in self.pages.iter() {
            page.child.set_rect(page_rect);
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.pages.iter() {
            let size = style.font.measure(&item.name);
            total_size += vec2!(size.x,0);
            if size.y > total_size.y {
                total_size.y = size.y;
            }
        }
        let mut page_size = vec2!(0i32,0i32);
        for item in self.pages.iter() {
            let size = item.child.calc_min_size();
            if size.x > page_size.x {
                page_size.x = size.x;
            }
            if size.y > page_size.y {
                page_size.y = size.y;
            }
        }
        if page_size.x > total_size.x {
            total_size.x = page_size.x;
        }
        total_size += vec2!(0,page_size.y);
        total_size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut r = rect!(0i32,0i32,0i32,0i32);
        for i in 0..self.pages.len() {
            let item = &self.pages[i];
            let size = style.font.measure(&item.name);
            r.s = size;
            let mut color = style.color;
            if let Some(n) = self.page.get() {
                if n == i {
                    color = style.current_color;
                }
            }
            if let BookHit::Tab(n) = self.hit.get() {
                if n == i {
                    color = style.hover_color;
                }
            }
            let mut text_color = style.disabled_text_color;
            if item.enabled.get() {
                text_color = style.text_color;
            }
            self.ui.draw.draw_rectangle(r,color,BlendMode::Replace);
            self.ui.draw.draw_text(r.o,&item.name,text_color,&style.font);
            r.o.x += size.x;
        }
        r.o = vec2!(0,r.o.y + style.font.measure("E").y);
        r.s = self.r.get().s - vec2!(0,r.o.y);
        if let Some(n) = self.page.get() {
            let offset = r.o;
            self.ui.draw.delta_offset(offset);
            self.pages[n].child.draw();
            self.ui.draw.delta_offset(-offset);
        }
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                BookHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                BookHit::Tab(_n) => {
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousepress(ui,window,p - self.pages[n].child.rect().o,b);
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
                    self.capturing.set(true);
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousepress(ui,window,p - self.pages[n].child.rect().o,b);
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

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                BookHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                BookHit::Tab(n) => {
                    println!("Book: stop clicking on tab {}",n);
                    self.set_page(n);
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mouserelease(ui,window,p - self.pages[n].child.rect().o,b);
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
                BookHit::Tab(_n) => {
                    false
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mouserelease(ui,window,p - self.pages[n].child.rect().o,b);
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

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
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
                        let result = self.pages[n].child.mousemove(ui,window,p - self.pages[n].child.rect().o);
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
                BookHit::Tab(_n) => {
                    true
                },
                BookHit::Page => {
                    if let Some(n) = self.page.get() {
                        let result = self.pages[n].child.mousemove(ui,window,p - self.pages[n].child.rect().o);
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

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}