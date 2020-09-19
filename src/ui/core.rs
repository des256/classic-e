// E - UI - Core
// Desmond Germans, 2020

/*
pub fn capturing_mouse_press(&self,p: Vec2<i32>,b: MouseButton) -> bool {
    if let Some(i) = self.capturing_child.get() {
        let child = &self.children[i].get();
        let r = child.get_rect();
        child.handle_mouse_press(p - r.o,b);
        return true;
    }
    false
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
        }
    }
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
    false
}

    pub fn capturing_mouse_release(&self,p: i32x2,b: MouseButton) -> bool {
        if let Some(i) = self.capturing_child.get() {
            let child = self.children[i].get();
            let r = child.get_rect();
            child.handle_mouse_release(p - r.o,b);
        }
    }
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
    false
}

    pub fn capturing_mouse_move(&self,p: i32x2) -> bool {
        if let Some(i) = self.capturing_child.get() {
            let child = self.children[i].get();
            let r = child.get_rect();
            if child.handle_mouse_move(p - r.o) {
                self.capturing_child.set(Some(i));
                return true;
            }
            else {
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
    false
}
*/
