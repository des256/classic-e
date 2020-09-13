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

pub fn other_mouse_press(&self,p: Vec2<i32>,b: MouseButton) {
    for i in 0..self.children.len() {
        let child = self.children[i].get();
        let r = child.get_rect();
        if r.contains(&p) {
            child.handle_mouse_press(p - r.o,b);
        }
    }
}

pub fn capturing_mouse_release(&self,p: Vec2<i32>,b: MouseButton) -> bool {
    if let Some(i) = self.capturing_child.get() {
        let child = self.children[i].get();
        let r = child.get_rect();
        child.handle_mouse_release(p - r.o,b);
        return true;
    }
    false
}

pub fn other_mouse_release(&self,p: Vec2<i32>,b: MouseButton) {
    for i in 0..self.children.len() {
        let child = self.children[i].get();
        let r = child.get_rect();
        if r.contains(&p) {
            child.handle_mouse_release(p - r.o,b);
        }
    }
}

pub fn capturing_mouse_move(&self,p: Vec2<i32>) -> bool {
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

pub fn other_mouse_move(&self,p: Vec2<i32>) -> bool {
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
*/
