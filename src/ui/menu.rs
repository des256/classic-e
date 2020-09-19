// E - UI - Menu
// Desmond Germans, 2020

/*use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Menu item.
pub struct MenuItem {
    pub(crate) text: String,
}

/// Menu widget.
pub struct Menu {
    
    /// Reference to UI context.
    ui: Rc<ui::UI>,

    /// Menu item padding.
    pub inner_padding: Cell<i32x2>,

    /// Menu item color.
    pub color: Cell<u32>,

    /// The items.
    pub items: RefCell<Vec<MenuItem>>,

    /// Font to use when drawing the items.
    pub font: RefCell<Rc<ui::Font>>,

    pub pressed: Cell<bool>,
}

impl Menu {
    /// Create new menu.
    /// ## Arguments
    /// * `ui` - UI context to create this menu for.
    /// ## Returns
    /// The menu.
    pub fn new(ui: &Rc<ui::UI>,font: &Rc<ui::Font>) -> Result<Menu,SystemError> {
        Ok(Menu {
            ui: Rc::clone(ui),
            r: Cell::new(rect!(0,0,1,1)),
            inner_padding: Cell::new(vec2!(0,0)),
            color: Cell::new(0xFFFFFFFF),
            items: RefCell::new(Vec::new()),
            font: RefCell::new(Rc::clone(font)),
            pressed: Cell::new(false),
        })
    }
}

impl ui::Widget for Menu {
    fn measure(&self) -> i32x2 {
        let font = self.font.borrow();
        let inner_padding = self.inner_padding.get();
        let mut total_size = vec2!(0i32,0i32);
        for item in self.items.borrow().iter() {
            let size = font.measure(&item.text) + 2 * inner_padding;
            if size.x > total_size.x {
                total_size.x = size.x;
            }
            total_size.y += size.y;
        }
        total_size
    }

    fn get_rect(&self) -> i32r {
        self.r.get()
    }

    fn set_rect(&self,r: i32r) {
        self.r.set(r);
    }

    fn draw(&self) {
        let r = self.r.get();
        let font = self.font.borrow();
        let color = self.color.get();
        let inner_padding = self.inner_padding.get();
        let mut o = r.o;
        for item in self.items.borrow().iter() {
            let size = font.measure(&item.text) + 2 * inner_padding;
            self.ui.draw_text(o + inner_padding,&item.text,color,&font);
            o.y += size.y;
        }
    }

    fn mouse_press(&self,_pos: i32x2,_button: Mouse) -> bool {
        self.pressed.set(true);
        true
    }

    fn mouse_release(&self,_pos: i32x2,_button: Mouse) -> bool {
        self.pressed.set(false);
        false
    }

    fn mouse_move(&self,_pos: i32x2) -> bool {
        self.pressed.get()
   }
}*/