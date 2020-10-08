// E - UI - Menu
// Desmond Germans, 2020

// A menu is a popup window containing a list of menu items. TBD.

use{
    crate::*,
    std::cell::Cell,
};

pub struct MenuItem {

}

/// Menu.
pub struct Menu {
    r: Cell<Rect<i32>>,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Result<Menu,SystemError> {
        Ok(Menu {
            r: Cell::new(rect!(0,0,0,0)),
            items: items,
        })
    }
}

impl Widget for Menu {
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
