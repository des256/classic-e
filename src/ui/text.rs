// E - UI - Text
// Desmond Germans, 2020

use crate::*;
use crate::prelude::*;
use std::rc::Rc;

pub struct Text {
    text: String,
    color: Vec4<f32>,
    font: Rc<Font>,
    halign: HAlignment,
    valign: VAlignment,
    padding: Vec2<f32>,
}

impl Text {
    pub fn new(graphics: Rc<Graphics>,text: &str) -> Text {
        Text {
            text: String::from(text),
            color: vec4!(1.0,1.0,1.0,1.0),
            font: graphics.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font"),
            halign: HAlignment::Center,
            valign: VAlignment::Center,
            padding: vec2!(0.0,0.0),
        }
    }

    pub fn text(&mut self,t: &str) {
        self.text = String::from(t);
    }

    pub fn color<T>(&mut self,c: T) where Vec4<f32>: From<T> {
        self.color = Vec4::<f32>::from(c);
    }

    pub fn font(&mut self,f: Rc<Font>) {
        self.font = f;
    }

    pub fn halign(&mut self,a: HAlignment) {
        self.halign = a;
    }

    pub fn valign(&mut self,a: VAlignment) {
        self.valign = a;
    }

    pub fn padding(&mut self,p: Vec2<f32>) {
        self.padding = p;
    }
}

impl Widget for Text {
    fn draw(&self,graphics: Rc<Graphics>,space: Rect<f32>) {
        let (size,offset) = self.font.measure(&self.text);
        let size = size + 2.0 * self.padding;
        let p: Vec2<f32> = self.padding + vec2!(
            match self.halign {
                HAlignment::Fill => space.o.x,
                HAlignment::Center => space.o.x + 0.5 * (space.s.x - size.x),
                HAlignment::Left => space.o.x,
                HAlignment::Right => space.o.x + space.s.x - size.x,
            },
            match self.valign {
                VAlignment::Fill => space.o.y,
                VAlignment::Center => space.o.y + 0.5 * (space.s.y - size.y),
                VAlignment::Top => space.o.y + space.s.y - size.y,
                VAlignment::Bottom => space.o.y,
            }
        );
        graphics.set_color(self.color);
        graphics.set_blend(BlendMode::Over);
        graphics.draw_text(p + offset,&self.text,&self.font);
    }

    fn measure(&self) -> Vec2<f32> {
        self.padding + self.font.measure(&self.text).0
    }
}