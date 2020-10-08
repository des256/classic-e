// E - UI - Button
// Desmond Germans, 2020

// A button is a rectangle with a text, image or action reference that can be
// clicked.

use{
    crate::*,
    std::cell::Cell,
};

#[derive(Copy,Clone)]
pub enum ButtonHit {
    Nothing,
    Button,
}

/// Button.
pub struct Button {
    r: Cell<Rect<i32>>,
    hit: Cell<ButtonHit>,
    name: String,
    enabled: Cell<bool>,
    pressed: Cell<bool>,
}

impl Button {
    pub fn new(name: &str) -> Result<Button,SystemError> {
        Ok(Button {
            r: Cell::new(rect!(0,0,0,0)),
            hit: Cell::new(ButtonHit::Nothing),
            name: name.to_string(),
            enabled: Cell::new(true),
            pressed: Cell::new(false),
        })
    }

    pub fn find_hit(&self,draw: &Draw,p: Vec2<i32>) -> ButtonHit {
        ButtonHit::Button
    }
}

impl Widget for Button {
    fn rect(&self) -> Rect<i32> {
        self.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.r.set(r);
    }

    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32> {
        let styles = draw.styles.borrow();
        let size = styles.font.measure(&self.name);
        size
    }

    fn draw(&self,draw: &Draw) {
        let styles = draw.styles.borrow();
        let color = if self.enabled.get() {
            if self.pressed.get() {
                styles.button_pressed_color
            }
            else if let ButtonHit::Button = self.hit.get() {
                styles.button_hover_color
            }
            else {
                styles.button_color
            }
        }
        else {
            styles.button_disabled_color
        };
        let text_color = if self.enabled.get() {
            styles.button_text_color
        }
        else {
            styles.button_disabled_text_color
        };
        let r = self.r.get();
        draw.draw_rectangle(r,color,BlendMode::Replace);
        draw.draw_text(r.o(),&self.name,text_color,&styles.font);
    }

    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event) {
        match event {
            Event::MousePress(p,b) => {

            },
            Event::MouseRelease(p,b) => {

            },
            Event::MouseMove(p) => {
                self.hit.set(self.find_hit(draw,p));
            },
            _ => { },
        }
    }
}