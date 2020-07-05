// E - UI - WidgetEngine
// Desmond Germans, 2020

use crate::*;

pub struct WidgetEngine {
    pub ha: HAlignment,
    pub va: VAlignment,
    pub padding: Vec2<f32>,
}

impl WidgetEngine {
    pub fn new() -> WidgetEngine {
        WidgetEngine {
            ha: HAlignment::Center,
            va: VAlignment::Center,
            padding: vec2!(0.0,0.0),
        }
    }
}