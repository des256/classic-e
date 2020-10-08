// E - UI - ColorPicker
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Color picker.
pub struct ColorPicker {
    r: Cell<Rect<i32>>,
}
