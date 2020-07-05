// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

pub struct Text {
    ui: Rc<UI>,
    engine: WidgetEngine,
    font: RefCell<Rc<Font>>,
    text: RefCell<String>,
    color: Cell<Vec4<f32>>,
}

impl Text {
    pub fn new(ui: &Rc<UI>,text: &str) -> Text {
        Text {
            ui: Rc::clone(ui),
            engine: WidgetEngine::new(),
            font: RefCell::new(ui.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font")),
            text: RefCell::new(String::from(text)),
            color: Cell::new(vec4!(1.0,1.0,1.0,1.0)),
        }
    }

    pub fn set_font(&self,font: Rc<Font>) {
        *(self.font.borrow_mut()) = font;
    }

    pub fn set_color<T>(&self,color: T) where Vec4<f32>: From<T> {
        self.color.set(Vec4::<f32>::from(color));
    }
}

impl Widget for Text {
    fn draw(&self,gc: &GC,space: Rect<f32>) {
        let (size,offset) = self.font.borrow().measure(&self.text.borrow());
        let size = size + 2.0 * self.engine.padding;
        let p: Vec2<f32> = self.engine.padding + vec2!(
            match self.engine.ha {
                HAlignment::Fill => space.o.x,
                HAlignment::Center => space.o.x + 0.5 * (space.s.x - size.x),
                HAlignment::Left => space.o.x,
                HAlignment::Right => space.o.x + space.s.x - size.x,
            },
            match self.engine.va {
                VAlignment::Fill => space.o.y,
                VAlignment::Center => space.o.y + 0.5 * (space.s.y - size.y),
                VAlignment::Top => space.o.y + space.s.y - size.y,
                VAlignment::Bottom => space.o.y,
            }
        );
        gc.set_color(self.color.get());
        gc.ui.system.set_blend(BlendMode::Over);
        gc.draw_text(p + offset,&self.text.borrow());
    }

    fn measure(&self) -> Vec2<f32> {
        self.engine.padding + self.font.borrow().measure(&self.text.borrow()).0
    }

    fn halign(mut self,alignment: HAlignment) -> Self {
        self.engine.ha = alignment;
        self
    }

    fn valign(mut self,alignment: VAlignment) -> Self {
        self.engine.va = alignment;
        self
    }

    fn padding(mut self) -> Self {
        self.engine.padding += vec2!(20.0,20.0);
        self
    }
}
