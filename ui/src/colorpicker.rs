// E - UI - ColorPicker
// Desmond Germans, 2020

// A color picker allows the user to select a color.

use{
    crate::*,
    std::cell::Cell,
};

/// Color picker.
pub struct ColorPicker {
    r: Cell<Rect<i32>>,
}
