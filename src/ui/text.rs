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

pub struct Text {
    engine: WidgetEngine,
    font: Rc<Font>,
    text: String,
}

impl Text {
    pub fn new(text: &str,font: Rc<Font>) -> Text {
        Text {
            engine: WidgetEngine::new(),
            font: font,
            text: String::from(text),
        }
    }
}

impl Widget for Text {
    fn draw(&self,_graphics: &Graphics,_r: Rect<f32>) {
        // TODO: draw in rect according to alignment and padding
    }

    fn measure(&self) -> Vec2<f32> {
        self.engine.padding + self.font.measure(&self.text)
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
        self.engine.padding += vec2!(10.0,10.0);
        self
    }

}