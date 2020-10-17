// E - UI - ToolTip
// Desmond Germans, 2020

// ToolTip is an informative popup that appears when the mouse hovers over a
// widget a certain time.

use crate::*;
use std::{
    cell::{
        Cell,
        RefCell,
    },
    rc::Rc,
};

/// Tool tip style.
pub struct ToolTipStyle {
    pub font: Rc<Font>,
    pub text_color: u32,
}

/// Tool tip.
pub struct ToolTip {
    ui: Rc<UI>,
    style: RefCell<ToolTipStyle>,
    p: Cell<Vec2<i32>>,
    text: String,
}

impl ToolTip {
    pub fn new(ui: &Rc<UI>,text: &str) -> Result<Rc<ToolTip>,SystemError> {
        Ok(Rc::new(ToolTip {
            ui: Rc::clone(&ui),
            style: RefCell::new(ToolTipStyle {
                font: Rc::clone(&ui.font),
                text_color: 0xAAAAAA,
            }),
            p: Cell::new(vec2!(0,0)),
            text: text.to_string(),
        }))
    }
}

impl Widget for ToolTip {
    fn rect(&self) -> Rect<i32> {
        rect!(self.p.get(),vec2!(0,0))
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.p.set(r.o);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        style.font.measure(self.text.as_str())
    }

    fn draw(&self) {
        let style = self.style.borrow();
        self.ui.draw_text(vec2!(0,0),&self.text,style.text_color,&style.font);
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

// widget interface might not even be necessary for tooltips...