// E - UI - Toggle
// Desmond Germans, 2020

// A toggle allows the user to select or unselect a control.

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

#[derive(Copy,Clone,Debug)]
enum ToggleHit {
    Nothing,
    Toggle,
}

/// On/off toggle.
pub struct Toggle {
    ui: Rc<UI>,
    style: RefCell<style::Toggle>,
    r: Cell<Rect<i32>>,
    hit: Cell<ToggleHit>,
    capturing: Cell<bool>,
    enabled: Cell<bool>,
    value: Cell<bool>,
    closure: Box<dyn Fn(bool)>,
}

const TOGGLE_SIZE: i32 = 20;
//const TOGGLE_GUTTER_SIZE: i32 = 5;

impl Toggle {
    pub fn new<C: Fn(bool) + 'static>(ui: &Rc<UI>,closure: C) -> Result<Rc<Toggle>,SystemError> {
        Ok(Rc::new(Toggle {
            ui: Rc::clone(&ui),
            style: RefCell::new(style::Toggle {
                color: 0x444444,
                empty_color: 0x222222,
                full_color: 0xCC6633,
                tab_color: 0xAAAAAA,
                tab_hover_color: 0x3366CC,
                disabled_color: 0x888888,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ToggleHit::Nothing),
            capturing: Cell::new(false),
            enabled: Cell::new(true),
            value: Cell::new(false),
            closure: Box::new(closure),
        }))
    }

    fn find_hit(&self,p: Vec2<i32>) -> ToggleHit {
        if rect!(vec2!(0,0),self.r.get().s).contains(&p) {
            ToggleHit::Toggle
        }
        else {
            ToggleHit::Nothing
        }
    }

    pub fn set_value(&self,value: bool) {
        self.value.set(value);
    }
}

impl Widget for Toggle {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        vec2!(TOGGLE_SIZE * 2,TOGGLE_SIZE)
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let mut left_color = style.full_color;
        let mut right_color = style.empty_color;
        if self.enabled.get() {
            if self.value.get() {
                right_color = if let ToggleHit::Toggle = self.hit.get() {
                    style.tab_hover_color
                }
                else {
                    style.tab_color
                };
            }
            else {
                left_color = if let ToggleHit::Toggle = self.hit.get() {
                    style.tab_hover_color
                }
                else {
                    style.tab_color
                };
            }
        }
        else {
            right_color = style.disabled_color;
            left_color = style.disabled_color;
        };
        self.ui.draw.draw_rectangle(rect!(vec2!(0,0),vec2!(TOGGLE_SIZE,TOGGLE_SIZE)),left_color,BlendMode::Replace);
        self.ui.draw.draw_rectangle(rect!(vec2!(TOGGLE_SIZE,0),vec2!(TOGGLE_SIZE,TOGGLE_SIZE)),right_color,BlendMode::Replace);
    }

    fn keypress(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn keyrelease(&self,_ui: &UI,_window: &Rc<UIWindow>,_k: u8) {
    }

    fn mousepress(&self,_ui: &UI,_window: &Rc<UIWindow>,_p: Vec2<i32>,_b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToggleHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToggleHit::Toggle => {
                    true
                },
            }
        }
        else {
            match self.hit.get() {
                ToggleHit::Nothing => {
                    false
                },
                ToggleHit::Toggle => {
                    println!("Toggle: start clicking");
                    self.capturing.set(true);
                    true
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,_b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToggleHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToggleHit::Toggle => {
                    println!("Toggle: stop clicking");
                    self.set_value(!self.value.get());
                    println!("calling closure for {}",self.value.get());
                    (self.closure)(self.value.get());
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
            }
        }
        else {
            match self.hit.get() {
                ToggleHit::Nothing => {
                    false
                },
                ToggleHit::Toggle => {
                    false
                },
            }
        }        
    }

    fn mousemove(&self,_ui: &UI,_window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ToggleHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ToggleHit::Toggle => {
                    println!("Toggle: still clicking");
                    true
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                ToggleHit::Nothing => {
                    false
                },
                ToggleHit::Toggle => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,_ui: &UI,_window: &Rc<UIWindow>,_w: MouseWheel) -> bool {
        false
    }
}
