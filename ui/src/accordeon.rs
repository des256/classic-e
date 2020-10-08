// E - UI - Accordeon
// Desmond Germans, 2020

// An Accordeon is a group of horizontal or vertical tabs of which one is
// open, showing the widget contents. Very similar to a Book.

use{
    crate::*,
    std::{
        cell::Cell,
        rc::Rc,
    },
};

#[derive(Copy,Clone)]
pub enum AccordeonHit {
    Nothing,
    Tab(usize),
    Page,
}

pub struct AccordeonItem {
    name: String,
    child: Rc<dyn Widget>,
}

/// Accordeon.
pub struct Accordeon {
    r: Cell<Rect<i32>>,
    hit: Cell<AccordeonHit>,
    items: Vec<AccordeonItem>,
    current: Cell<Option<usize>>,
}

impl Accordeon {
    pub fn new(items: Vec<AccordeonItem>) -> Result<Accordeon,SystemError> {
        Ok(Accordeon {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(AccordeonHit::Nothing),
            items: items,
            current: Cell::new(None),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> AccordeonHit {
        AccordeonHit::Nothing  // Should be Tab(i) or Page, once we know which page is open
    }
}

impl Widget for Accordeon {
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
            if size.x() > total_size.x() {
                total_size.set_x(size.x());
            }
            total_size += vec2!(0,size.y());
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
            let mut color = if let AccordeonHit::Tab(n) = self.hit.get() {
                if n == i {
                    styles.accordeon_tab_hover_color
                }
                else {
                    styles.accordeon_tab_color
                }
            } else {
                if let Some(n) = self.current.get() {
                    if n == i {
                        styles.accordeon_tab_current_color
                    }
                    else {
                        styles.accordeon_tab_color
                    }
                }
                else {
                    styles.accordeon_tab_color
                }
            };
            draw.draw_rectangle(r,color,BlendMode::Replace);
            draw.draw_text(r.o(),&item.name,styles.accordeon_tab_text_color,&styles.font);
            r.set_oy(r.oy() + size.y());
            if let Some(n) = self.current.get() {
                if n == i {
                    // TODO: localize
                    self.items[i].child.draw(draw);
                    // TODO: unlocalize
                }
                r.set_oy(r.oy() + self.items[i].child.rect().sy());
            }
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
                if let AccordeonHit::Page = self.hit.get() {
                    // TODO: pass down
                }
            },
            _ => { },
        }
    }
}