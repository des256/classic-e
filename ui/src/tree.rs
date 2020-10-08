// E - UI - Tree
// Desmond Germans, 2020

// A tree is a vertical list of items with subitems. TBD.

use{
    crate::*,
    std::cell::Cell,
};

pub struct TreeItem {
    
}

/// Tree.
pub struct Tree {
    r: Cell<Rect<i32>>,
    items: Vec<TreeItem>,
}

impl Tree {
    pub fn new() -> Result<Tree,SystemError> {
        Ok(Tree {
            r: Cell::new(rect!(0,0,0,0)),
            items: Vec::new(),
        })
    }
}

impl Widget for Tree {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,_draw: &Draw) -> Vec2<i32> {
        vec2!(0,0)
    }

    fn draw(&self,_draw: &Draw) {
    }

    fn handle(&self,_ui: &UI,_window: &Window,_event: Event) {
    }
}
