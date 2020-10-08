// E - UI - ToolBar
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

pub struct ToolBarItem {

}

/// Tool bar.
pub struct ToolBar {
    r: Cell<Rect<i32>>,
    items: Vec<ToolBarItem>,
}

impl ToolBar {
    pub fn new(items: Vec<ToolBarItem>) -> Result<ToolBar,SystemError> {
        Ok(ToolBar {
            r: Cell::new(rect!(0,0,0,0)),
            items: items,
        })
    }
}

impl Widget for ToolBar {
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
