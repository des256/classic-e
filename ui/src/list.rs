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

#[derive(Copy,Clone,Debug)]
enum ListHit {
    Nothing,
    _Item(usize),
}

/// List item.
pub struct ListItem {
    
}

/// List.
pub struct List {
    _ui: Rc<UI>,
    style: RefCell<style::List>,
    r: Cell<Rect<i32>>,
    _hit: Cell<ListHit>,
    _capturing: Cell<bool>,
    _items: Vec<ListItem>,
    // TBD current, or multiple currents
}

const DEFAULT_LIST_ITEMS: i32 = 3;

impl List {
    pub fn new(ui: &Rc<UI>) -> Result<Rc<List>,SystemError> {
        Ok(Rc::new(List {
            _ui: Rc::clone(&ui),
            style: RefCell::new(style::List {
                font: Rc::clone(&ui.font),
            }),
            r: Cell::new(rect!(0,0,0,0)),
            _hit: Cell::new(ListHit::Nothing),
            _capturing: Cell::new(false),
            _items: Vec::new(),
        }))
    }

    fn _find_hit(&self,_p: Vec2<i32>) -> ListHit {
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

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>) -> bool {
        false
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
