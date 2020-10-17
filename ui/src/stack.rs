// E - UI - Stack
// Desmond Germans, 2020

// A stack is a horizontal or vertical bunch of widgets.

use{
    crate::*,
    std::{
        cell::{
            Cell,
            RefCell,
        },
        rc::Rc,
    },
};

#[doc(hidden)]
#[derive(Copy,Clone,Debug)]
pub enum StackHit {
    Nothing,
    Child(usize),
}

/// Horizontal or vertical stack style.
pub struct StackStyle {

}

/// Horizontal or vertical stack of child widgets.
pub struct Stack {
    ui: Rc<UI>,
    orientation: Orientation,
    _style: RefCell<StackStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<StackHit>,
    capturing: Cell<bool>,
    children: Vec<Rc<dyn Widget>>,
}

impl Stack {
    pub fn new_horizontal(ui: &Rc<UI>,children: Vec<Rc<dyn Widget>>) -> Result<Rc<Stack>,SystemError> {
        Ok(Rc::new(Stack {
            ui: Rc::clone(&ui),
            orientation: Orientation::Horizontal,
            _style: RefCell::new(StackStyle { }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(StackHit::Nothing),
            capturing: Cell::new(false),
            children: children,
        }))
    }

    pub fn new_vertical(ui: &Rc<UI>,children: Vec<Rc<dyn Widget>>) -> Result<Rc<Stack>,SystemError> {
        Ok(Rc::new(Stack {
            ui: Rc::clone(&ui),
            orientation: Orientation::Vertical,
            _style: RefCell::new(StackStyle { }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(StackHit::Nothing),
            capturing: Cell::new(false),
            children: children,
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> StackHit {
        for i in 0..self.children.len() {
            let child = &self.children[i];
            if child.rect().contains(&p) {
                return StackHit::Child(i);
            }
        }
        StackHit::Nothing
    }
}

impl Widget for Stack {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
        match self.orientation {
            Orientation::Horizontal => {
                let mut ox = 0i32;
                for child in self.children.iter() {
                    let size = child.calc_min_size();
                    child.set_rect(rect!(vec2!(ox,0),vec2!(size.x,r.s.y)));
                    ox += size.x;
                }
            },
            Orientation::Vertical => {
                let mut oy = 0i32;
                for child in self.children.iter() {
                    let size = child.calc_min_size();
                    child.set_rect(rect!(vec2!(0,oy),vec2!(r.s.x,size.y)));
                    oy += size.y;
                }
            },
        }
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        match self.orientation {
            Orientation::Horizontal => {
                let mut total_size = vec2!(0i32,0i32);
                for child in self.children.iter() {
                    let size = child.calc_min_size();
                    total_size += vec2!(size.x,0);
                    if size.y > total_size.y {
                        total_size.y = size.y;
                    }
                }
                total_size
            },
            Orientation::Vertical => {
                let mut total_size = vec2!(0i32,0i32);
                for child in self.children.iter() {
                    let size = child.calc_min_size();
                    if size.x > total_size.x {
                        total_size.x = size.x;
                    }
                    total_size += vec2!(0,size.y);
                }
                total_size
            },
        }
    }

    fn draw(&self) {
        for child in self.children.iter() {
            let offset = child.rect().o;
            self.ui.delta_offset(offset);
            child.draw();
            self.ui.delta_offset(-offset);
        }
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                StackHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                StackHit::Child(n) => {
                    let result = self.children[n].mousepress(ui,window,p - self.children[n].rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            match self.hit.get() {
                StackHit::Nothing => {
                    false
                },
                StackHit::Child(n) => {
                    let result = self.children[n].mousepress(ui,window,p - self.children[n].rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                StackHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                StackHit::Child(n) => {
                    let result = self.children[n].mouserelease(ui,window,p - self.children[n].rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            match self.hit.get() {
                StackHit::Nothing => {
                    false
                }
                StackHit::Child(n) => {
                    let result = self.children[n].mouserelease(ui,window,p - self.children[n].rect().o,b);
                    self.capturing.set(result);
                    result
                },
            }
        }
    }
    
    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                StackHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                StackHit::Child(n) => {
                    let result = self.children[n].mousemove(ui,window,p - self.children[n].rect().o);
                    self.capturing.set(result);
                    result
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                StackHit::Nothing => {
                    false
                },
                StackHit::Child(n) => {
                    let result = self.children[n].mousemove(ui,window,p - self.children[n].rect().o);
                    self.capturing.set(result);
                    result
                }
            }
        }
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
