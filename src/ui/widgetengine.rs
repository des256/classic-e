// E - UI - WidgetEngine
// Desmond Germans, 2020

use crate::*;

#[doc(hidden)]
pub struct WidgetEngine {
    pub ha: ui::HAlignment,
    pub va: ui::VAlignment,
    pub padding: Vec2<f32>,
}

impl WidgetEngine {
    pub fn new() -> WidgetEngine {
        WidgetEngine {
            ha: ui::HAlignment::Center,
            va: ui::VAlignment::Center,
            padding: vec2!(0.0,0.0),
        }
    }
}