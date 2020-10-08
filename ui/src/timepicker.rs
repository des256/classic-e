// E - UI - TimePicker
// Desmond Germans, 2020

// A time picker allows the user to select a date.

use{
    crate::*,
    std::cell::Cell,
};

/// Time picker.
pub struct TimePicker {
    r: Cell<Rect<i32>>,
}
