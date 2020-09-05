// E - UI - Core
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

/// Widget core.
pub struct Core {
    pub(crate) state: Rc<ui::UIState>,
    pub r: Cell<Rect<i32>>,
    pub children: Vec<Box<dyn ui::Widget>>,
    pub capturing_child: Cell<Option<usize>>,
}

impl Core {

    pub fn new(state: &Rc<ui::UIState>) -> Core {
        Core {
            state: Rc::clone(state),
            r: Cell::new(rect!(0,0,0,0)),
            children: Vec::new(),
            capturing_child: Cell::new(None),
        }
    }

    pub fn new_from_vec(state: &Rc<ui::UIState>,children: Vec<Box<dyn ui::Widget>>) -> Core {
        Core {
            state: Rc::clone(state),
            r: Cell::new(rect!(0,0,0,0)),
            children: children,
            capturing_child: Cell::new(None),
        }        
    }

    /*pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: gpu::BlendMode) {
        self.ui.draw_rectangle(r,color,blend_mode);
    }

    pub fn draw_text<C: ColorParameter>(&self,p: Vec2<i32>,text: &str,color: C,font: &ui::Font) {
        self.ui.draw_text(p,text,color,font);
    }*/

    pub fn handle_mouse_press(&self,b: MouseButton) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the press there
        if let Some(i) = self.capturing_child.get() {
            let result = self.children[i].handle_mouse_press(b);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child.set(None);
        }
        
        ui::MouseResult::Unprocessed
    }

    pub fn handle_mouse_release(&self,b: MouseButton) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the release there
        if let Some(i) = self.capturing_child.get() {
            let result = self.children[i].handle_mouse_release(b);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child.set(None);
        }

        ui::MouseResult::Unprocessed
    }

    pub fn handle_mouse_move(&self,p: Vec2<i32>) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the move
        if let Some(i) = self.capturing_child.get() {
            let child = &self.children[i];
            let r = child.get_rect();
            let result = child.handle_mouse_move(p - r.o);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child.set(None);
        }

        // otherwise, handle the move in any of the children containing the mouse
        for i in 0..self.children.len() {
            let child = &self.children[i];
            let r = child.get_rect();
            if r.contains(&p) {
                let result = child.handle_mouse_move(p - r.o);
                if let ui::MouseResult::ProcessedCapture = result {
                    self.capturing_child.set(Some(i));
                }
                return result;
            }
        }

        ui::MouseResult::Unprocessed
    }
}
