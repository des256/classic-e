// E - UI - Text
// Desmond Germans, 2020

use crate::WidgetEngine;
use crate::Graphics;
use crate::Rect;
use crate::Widget;
use crate::Vec2;
use crate::Font;
use std::rc::Rc;
use crate::HAlignment;
use crate::VAlignment;
use crate::prelude::*;
use crate::ARGB8;
use crate::BlendMode;

pub struct Text {
    engine: WidgetEngine,
    font: Rc<Font>,
    text: String,
    color: ARGB8,
}

impl Text {
    pub fn new(graphics: Rc<Graphics>,text: &str) -> Text {
        Text {
            engine: WidgetEngine::new(),
            font: graphics.get_font("arialn.fnt",vec2!(14.0,14.0),0.0).expect("cannot load font"),
            text: String::from(text),
            color: ARGB8::from(vec4!(255,255,255,255)),
        }
    }

    pub fn font(mut self,font: Rc<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn color(mut self,color: ARGB8) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Text {
    fn draw(&self,graphics: Rc<Graphics>,space: Rect<f32>) {
        let (size,offset) = self.font.measure(&self.text);
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
        graphics.set_color(self.color);
        graphics.set_blend(BlendMode::Over);
        graphics.draw_text(p + offset,&self.text,&self.font);
    }

    fn measure(&self) -> Vec2<f32> {
        self.engine.padding + self.font.measure(&self.text).0
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