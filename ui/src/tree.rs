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

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum TreeHit {
    Nothing
}

/// Tree style.
pub struct TreeStyle {
    pub font: Rc<Font>,
}

/// Tree item.
pub struct TreeItem {
    
}

/// Tree.
pub struct Tree {
    ui: Rc<UI>,
    style: RefCell<TreeStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<TreeHit>,
    items: Vec<TreeItem>,
    // TBD: current or multiple currents
}

const DEFAULT_TREE_ITEMS: i32 = 3;

impl Tree {
    pub fn new(ui: &Rc<UI>) -> Result<Rc<Tree>,SystemError> {
        Ok(Rc::new(Tree {
            ui: Rc::clone(&ui),
            style: RefCell::new(TreeStyle {
                font: Rc::clone(&ui.font),
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(TreeHit::Nothing),
            items: Vec::new(),
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> TreeHit {
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
        self.hit.set(self.find_hit(p));
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
