// E - UI - Phase II - Tree
// Desmond Germans, 2020

// A tree is a vertical list of items with subitems. TBD.

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
pub enum TreeHit {
    Nothing
}

/// Tree item.
pub struct TreeItem {
    
}

/// Tree.
pub struct Tree {
    _ui: Rc<UI>,
    style: RefCell<style::Tree>,
    r: Cell<Rect<i32>>,
    _hit: Cell<TreeHit>,
    _items: Vec<TreeItem>,
    // TBD: current or multiple currents
}

const DEFAULT_TREE_ITEMS: i32 = 3;

impl Tree {
    pub fn new(ui: &Rc<UI>) -> Result<Rc<Tree>,SystemError> {
        Ok(Rc::new(Tree {
            _ui: Rc::clone(&ui),
            style: RefCell::new(style::Tree {
                font: Rc::clone(&ui.font),
            }),
            r: Cell::new(rect!(0,0,0,0)),
            _hit: Cell::new(TreeHit::Nothing),
            _items: Vec::new(),
        }))
    }

    fn _find_hit(&self,_p: Vec2<i32>) -> TreeHit {
        TreeHit::Nothing  // Should be Item(i) once we know how Scroller works
    }
}

impl Widget for Tree {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let size = style.font.measure("Text Item");
        vec2!(size.x,size.y * DEFAULT_TREE_ITEMS)
    }

    fn draw(&self) {
        // TODO: draw the tree items
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
