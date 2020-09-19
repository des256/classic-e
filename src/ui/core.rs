// E - UI - Core
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::Cell,
};

pub struct NamedWidget {
    pub name: String,
    pub widget: Box<dyn ui::Widget>,
}

pub trait ChildType {
    fn get(&self) -> &Box<dyn ui::Widget>;
    fn get_mut(&mut self) -> &mut Box<dyn ui::Widget>;
}

impl ChildType for Box<dyn ui::Widget> {
    fn get(&self) -> &Box<dyn ui::Widget> {
        self
    }

    fn get_mut(&mut self) -> &mut Box<dyn ui::Widget> {
        self
    }
}

impl ChildType for NamedWidget {
    fn get(&self) -> &Box<dyn ui::Widget> {
        &self.widget
    }

    fn get_mut(&mut self) -> &mut Box<dyn ui::Widget> {
        &mut self.widget
    }
}

/// Widget core.
pub struct Core<T> {
    pub state: Rc<ui::UIState>,
    pub r: Cell<i32r>,
    pub children: Vec<T>,
    pub capturing_child: Cell<Option<usize>>,
}

impl<T: ChildType> Core<T> {

    pub fn new(state: &Rc<ui::UIState>) -> Core<T> {
        Core {
            state: Rc::clone(state),
            r: Cell::new(i32r::zero()),
            children: Vec::new(),
            capturing_child: Cell::new(None),
        }
    }

    pub fn new_from_vec(state: &Rc<ui::UIState>,children: Vec<T>) -> Core<T> {
        Core {
            state: Rc::clone(state),
            r: Cell::new(i32r::zero()),
            children: children,
            capturing_child: Cell::new(None),
        }        
    }

    /*pub fn draw_rectangle<C: ColorParameter>(&self,r: i32r,color: C,blend_mode: gpu::BlendMode) {
        self.ui.draw_rectangle(r,color,blend_mode);
    }

    pub fn draw_text<C: ColorParameter>(&self,p: i32x2,text: &str,color: C,font: &ui::Font) {
        self.ui.draw_text(p,text,color,font);
    }*/

    pub fn capturing_mouse_press(&self,p: i32x2,b: MouseButton) -> bool {
        if let Some(i) = self.capturing_child.get() {
            let child = &self.children[i].get();
            let r = child.get_rect();
            child.handle_mouse_press(p - r.o,b);
            return true;
        }
        false
    }

    pub fn other_mouse_press(&self,p: i32x2,b: MouseButton) {
        for i in 0..self.children.len() {
            let child = self.children[i].get();
            let r = child.get_rect();
            if r.contains(&p) {
                child.handle_mouse_press(p - r.o,b);
            }
        }
    }

    pub fn capturing_mouse_release(&self,p: i32x2,b: MouseButton) -> bool {
        if let Some(i) = self.capturing_child.get() {
            let child = self.children[i].get();
            let r = child.get_rect();
            child.handle_mouse_release(p - r.o,b);
            return true;
        }
        false
    }

    pub fn other_mouse_release(&self,p: i32x2,b: MouseButton) {
        for i in 0..self.children.len() {
            let child = self.children[i].get();
            let r = child.get_rect();
            if r.contains(&p) {
                child.handle_mouse_release(p - r.o,b);
            }
        }
    }

    pub fn capturing_mouse_move(&self,p: i32x2) -> bool {
        if let Some(i) = self.capturing_child.get() {
            let child = self.children[i].get();
            let r = child.get_rect();
            if child.handle_mouse_move(p - r.o) {
                return true;
            }
            else {
                self.capturing_child.set(None);
                return false;
            }
        }
        false
    }

    pub fn other_mouse_move(&self,p: i32x2) -> bool {
        for i in 0..self.children.len() {
            let child = &self.children[i].get();
            let r = child.get_rect();
            if r.contains(&p) {
                if child.handle_mouse_move(p - r.o) {
                    self.capturing_child.set(Some(i));
                    return true;
                }
                else {
                    return false;
                }
            }
        }
        false
    }
}

#[macro_export]
macro_rules! widgets {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Box::new($x) as Box<dyn ui::Widget>);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! named_widgets {
    ($($n:expr,$x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(ui::NamedWidget { name: $n,widget: Box::new($x) as Box<dyn ui::Widget>, });
            )*
            temp_vec
        }
    };
}
