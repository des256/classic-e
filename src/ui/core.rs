// E - UI - Core
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::RefCell,
};

/// Widget core.
pub struct Core {
    pub(crate) anchor: Rc<ui::UIAnchor>,
    pub r: Rect<i32>,
    pub children: Vec<Rc<RefCell<dyn ui::Widget>>>,
    pub capturing_child: Option<usize>,
}

impl Core {
    pub fn new(anchor: &Rc<ui::UIAnchor>) -> Core {
        Core {
            anchor: Rc::clone(anchor),
            r: rect!(0,0,0,0),
            children: Vec::new(),
            capturing_child: None,
        }
    }

    pub fn new_from_vec(anchor: &Rc<ui::UIAnchor>,children: Vec<Rc<RefCell<dyn ui::Widget>>>) -> Core {
        Core {
            anchor: Rc::clone(anchor),
            r: rect!(0,0,0,0),
            children: children,
            capturing_child: None,
        }        
    }

    /*pub fn draw_rectangle<C: ColorParameter>(&self,r: Rect<i32>,color: C,blend_mode: gpu::BlendMode) {
        self.ui.draw_rectangle(r,color,blend_mode);
    }

    pub fn draw_text<C: ColorParameter>(&self,p: Vec2<i32>,text: &str,color: C,font: &ui::Font) {
        self.ui.draw_text(p,text,color,font);
    }*/

    pub fn handle_mouse_press(&mut self,b: MouseButton) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the press there
        if let Some(i) = self.capturing_child {
            let result = self.children[i].borrow_mut().handle_mouse_press(b);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child = None;
        }
        
        ui::MouseResult::Unprocessed
    }

    pub fn handle_mouse_release(&mut self,b: MouseButton) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the release there
        if let Some(i) = self.capturing_child {
            let result = self.children[i].borrow_mut().handle_mouse_release(b);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child = None;
        }

        ui::MouseResult::Unprocessed
    }

    pub fn handle_mouse_move(&mut self,p: Vec2<i32>) -> ui::MouseResult {
        // if any of the children captures the mouse, handle the move
        if let Some(i) = self.capturing_child {
            let child = &mut self.children[i];
            let r = child.borrow().get_rect();
            let result = child.borrow_mut().handle_mouse_move(p - r.o);
            if let ui::MouseResult::ProcessedCapture = result {
                return result;
            }
            self.capturing_child = None;
        }

        // otherwise, handle the move in any of the children containing the mouse
        for i in 0..self.children.len() {
            let child = &mut self.children[i];
            let r = child.borrow().get_rect();
            if r.contains(&p) {
                let result = child.borrow_mut().handle_mouse_move(p - r.o);
                if let ui::MouseResult::ProcessedCapture = result {
                    self.capturing_child = Some(i);
                }
                return result;
            }
        }

        ui::MouseResult::Unprocessed
    }
}
