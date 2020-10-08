// E - UI - Phase II - Tree
// Desmond Germans, 2020

// A tree is a vertical list of items with subitems. TBD.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum TreeHit {
    Nothing
}

pub struct TreeItem {
    
}

/// Tree.
pub struct Tree {
    r: Cell<Rect<i32>>,
    hit: Cell<TreeHit>,
    items: Vec<TreeItem>,
    // TBD: current or multiple currents
}

const DEFAULT_TREE_ITEMS: i32 = 3;

impl Tree {
    pub fn new() -> Result<Tree,SystemError> {
        Ok(Tree {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(TreeHit::Nothing),
            items: Vec::new(),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> TreeHit {
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

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        let size = styles.font.measure("Text Item");
        vec2!(size.x(),size.y() * DEFAULT_TREE_ITEMS)
    }

    fn draw(&self,draw: &Draw) {
        // TODO: draw the tree items
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}
