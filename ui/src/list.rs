// E - UI - List
// Desmond Germans, 2020

// A list is a vertical list of items. TBD.

use{
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
pub enum ListHit {
    Nothing,
    Item(usize),
}

/// List style.
pub struct ListStyle {
    pub font: Rc<Font>,
}

/// List item.
pub struct ListItem {
    
}

/// List.
pub struct List {
    ui: Rc<UI>,
    style: RefCell<ListStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<ListHit>,
    capturing: Cell<bool>,
    items: Vec<ListItem>,
    // TBD current, or multiple currents
}

const DEFAULT_LIST_ITEMS: i32 = 3;

impl List {
    pub fn new(ui: &Rc<UI>) -> Result<Rc<List>,SystemError> {
        Ok(Rc::new(List {
            ui: Rc::clone(&ui),
            style: RefCell::new(ListStyle {
                font: Rc::clone(&ui.font),
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ListHit::Nothing),
            capturing: Cell::new(false),
            items: Vec::new(),
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> ListHit {
        ListHit::Nothing  // Should be Item(i) once we know how Scroller works
    }
}

impl Widget for List {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let size = style.font.measure("Text Item");
        vec2!(size.x,size.y * DEFAULT_LIST_ITEMS)
    }

    fn draw(&self) {
        // TODO: draw the list items
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        // TODO: if capturing, no change, otherwise:
        self.hit.set(self.find_hit(p));
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
