// E - UI - Field
// Desmond Germans, 2020

// A field is one line of text input.

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
pub enum FieldHit {
    Nothing,
    Character(usize),
}

/// Text input field style.
pub struct FieldStyle {
    pub font: Rc<Font>,
    pub color: u32,
    pub text_color: u32,
    pub disabled_text_color: u32,
}

/// Text input field.
pub struct Field {
    ui: Rc<UI>,
    style: RefCell<FieldStyle>,
    r: Cell<Rect<i32>>,
    hit: Cell<FieldHit>,
    capturing: Cell<bool>,
    text: RefCell<String>,
    enabled: Cell<bool>,
}

impl Field {
    pub fn new(ui: &Rc<UI>) -> Result<Rc<Field>,SystemError> {
        Ok(Rc::new(Field {
            ui: Rc::clone(&ui),
            style: RefCell::new(FieldStyle {
                font: Rc::clone(&ui.font),
                color: 0x222222,
                text_color: 0xAAAAAA,
                disabled_text_color: 0x888888,
            }),
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(FieldHit::Nothing),
            capturing: Cell::new(false),
            text: RefCell::new("Hello, World!".to_string()),
            enabled: Cell::new(true),
        }))
    }

    pub fn find_hit(&self,p: Vec2<i32>) -> FieldHit {
        if rect!(vec2!(0,0),self.r.get().s).contains(&p) {
            // TODO: find which character is being pointed at
            FieldHit::Character(0)
        }
        else {
            FieldHit::Nothing
        }
    }
}

impl Widget for Field {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        let style = self.style.borrow();
        let size = style.font.measure("Field Text");
        size
    }

    fn draw(&self) {
        let style = self.style.borrow();
        let color = style.color;
        let mut text_color = style.disabled_text_color;
        if self.enabled.get() {
            text_color = style.text_color;
        }
        self.ui.draw_rectangle(rect!(vec2!(0,0),self.r.get().s),color,BlendMode::Replace);
        self.ui.draw_text(vec2!(0,0),&self.text.borrow(),text_color,&style.font);
    }

    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8) {
    }

    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool {
        false
    }

    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool {
        // TODO: if capturing, no change, otherwise:
        self.hit.set(self.find_hit(p));
        false
    }

    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool {
        false
    }
}
