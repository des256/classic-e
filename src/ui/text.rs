// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/// Text widget.
pub struct Text {
    ui: Rc<ui::UI>,
    engine: ui::WidgetEngine,
    font: RefCell<Rc<ui::Font>>,
    text: RefCell<String>,
    color: Cell<Vec4<f32>>,
}

impl Text {
    pub fn new(ui: &Rc<ui::UI>,text: &str) -> Result<Text,SystemError> {
        Ok(Text {
            ui: Rc::clone(ui),
            engine: ui::WidgetEngine::new(),
            font: RefCell::new(ui.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font")),
            text: RefCell::new(String::from(text)),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
        })
    }

    pub fn set_font(&self,font: Rc<ui::Font>) {
        *(self.font.borrow_mut()) = font;
    }

    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }
}

impl ui::Widget for Text {
    fn draw(&self,gc: &Rc<ui::GC>,space: Rect<f32>) {
        let (size,offset) = self.font.borrow().measure(&self.text.borrow());
        let size = size + 2.0 * self.engine.padding;
        let p: Vec2<f32> = self.engine.padding + vec2!(
            match self.engine.ha {
                ui::HAlignment::Fill => space.o.x,
                ui::HAlignment::Center => space.o.x + 0.5 * (space.s.x - size.x),
                ui::HAlignment::Left => space.o.x,
                ui::HAlignment::Right => space.o.x + space.s.x - size.x,
            },
            match self.engine.va {
                ui::VAlignment::Fill => space.o.y,
                ui::VAlignment::Center => space.o.y + 0.5 * (space.s.y - size.y),
                ui::VAlignment::Top => space.o.y + space.s.y - size.y,
                ui::VAlignment::Bottom => space.o.y,
            }
        );
        gc.set_color(self.color.get());
        gc.ui.gpu.set_blend(gpu::BlendMode::Over);
        gc.draw_text(p + offset,&self.text.borrow());
    }

    fn measure(&self) -> Vec2<f32> {
        self.engine.padding + self.font.borrow().measure(&self.text.borrow()).0
    }
}
