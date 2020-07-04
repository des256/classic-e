// E - UI - HStack
// Desmond Germans, 2020

use crate::*;
use crate::prelude::*;
use std::rc::Rc;

pub struct HStack {
    contents: Vec<Box<dyn Widget>>,
    spacing: f32,
    halign: HAlignment,
    valign: VAlignment,
    padding: Vec2<f32>,
}

impl HStack {
    pub fn new(graphics: Rc<Graphics>,contents: Vec<Box<dyn Widget>>) -> HStack {
        HStack {
            contents: contents,
            spacing: 10.0,
            halign: HAlignment::Center,
            valign: VAlignment::Center,
            padding: vec2!(0.0,0.0),
        }
    }

    pub fn contents(&mut self,c: Vec<Box<dyn Widget>>) {
        self.contents = c;
    }

    pub fn spacing(&mut self,s: f32) {
        self.spacing = s;
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

impl Widget for HStack {
    fn draw(&self,graphics: Rc<Graphics>,space: Rect<f32>) {
    }

    fn measure(&self) -> Vec2<f32> {
        vec2!(0.0,0.0)
    }
}