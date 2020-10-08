// E - UI - MenuBar
// Desmond Germans, 2020

// A menu bar is a horizontal bar with drop down menus.

use{
    crate::*,
    std::cell::Cell,
};

pub struct MenuBarItem {

}

/// Menu bar.
pub struct MenuBar {
    r: Cell<Rect<i32>>,
    items: Vec<MenuBarItem>,
}

impl MenuBar {
    pub fn new(items: Vec<MenuBarItem>) -> Result<MenuBar,SystemError> {
        Ok(MenuBar {
            r: Cell::new(rect!(0,0,0,0)),
            items: items,
        })
    }
}

impl Widget for MenuBar {
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
