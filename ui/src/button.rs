// E - UI - Button
// Desmond Germans, 2020

// A button is a rectangle with a text, image or action reference that can be
// clicked.

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
pub enum ButtonHit {
    Nothing,
    Button,
}

/// Button Style.
pub struct ButtonStyle {
    pub font: Rc<Font>,
    pub text_color: u32,
    pub disabled_text_color: u32,
    pub color: u32,
    pub hover_color: u32,
    pub disabled_color: u32,
    pub pressed_color: u32,
}

/// Button.
pub struct Button {
    ui: Rc<UI>,
    style: RefCell<ButtonStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<ButtonHit>,
    capturing: Cell<bool>,
    name: String,
    enabled: Cell<bool>,
    pressed: Cell<bool>,
}

impl Button {
    pub fn new(ui: &Rc<UI>,name: &str) -> Result<Button,SystemError> {
        Ok(Button {
            ui: Rc::clone(&ui),
            style: RefCell::new(ButtonStyle {
                font: Rc::clone(&ui.font),
                text_color: 0xAAAAAA,
                disabled_text_color: 0x666666,
                color: 0x444444,
                hover_color: 0x224488,
                disabled_color: 0x333333,
                pressed_color: 0x3366CC,    
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ButtonHit::Nothing),
            capturing: Cell::new(false),
            name: name.to_string(),
            enabled: Cell::new(true),
            pressed: Cell::new(false),
        })
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> ButtonHit {
        if rect!(vec2!(0,0),self.r.get().s()).contains(&p) {
            ButtonHit::Button
        }
        else {
            ButtonHit::Nothing
        }
    }
}

impl Widget for Button {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let size = style.font.measure(&self.name);
        size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let color = if self.enabled.get() {
            if self.pressed.get() {
                style.pressed_color
            }
            else if let ButtonHit::Button = self.hit.get() {
                style.hover_color
            }
            else {
                style.color
            }
        }
        else {
            style.disabled_color
        };
        let text_color = if self.enabled.get() {
            style.text_color
        }
        else {
            style.disabled_text_color
        };
        let r = self.r.get();
        self.ui.draw_rectangle(r,color,BlendMode::Replace);
        self.ui.draw_text(r.o(),&self.name,text_color,&style.font);
    }

    fn keypress(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Window,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ButtonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ButtonHit::Button => {
                    true
                },
            }
        }
        else {
            match self.hit.get() {
                ButtonHit::Nothing => {
                    false
                },
                ButtonHit::Button => {
                    println!("Button: start clicking");
                    self.capturing.set(true);
                    true
                },
            }
        }
    }

    fn mouserelease(&self,ui: &UI,window: &Window,p: Vec2<i32>,b: MouseButton) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ButtonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ButtonHit::Button => {
                    println!("Button: stop clicking");
                    self.capturing.set(false);
                    self.mousemove(ui,window,p)
                },
            }
        }
        else {
            match self.hit.get() {
                ButtonHit::Nothing => {
                    false
                },
                ButtonHit::Button => {
                    false
                },
            }
        }
    }

    fn mousemove(&self,ui: &UI,window: &Window,p: Vec2<i32>) -> bool {
        if self.capturing.get() {
            match self.hit.get() {
                ButtonHit::Nothing => {
                    self.capturing.set(false);
                    false
                },
                ButtonHit::Button => {
                    println!("Button: still clicking");
                    true
                },
            }
        }
        else {
            self.hit.set(self.find_hit(p));
            match self.hit.get() {
                ButtonHit::Nothing => {
                    false
                },
                ButtonHit::Button => {
                    true
                },
            }
        }
    }

    fn mousewheel(&self,ui: &UI,window: &Window,w: MouseWheel) -> bool {
        false
    }
}