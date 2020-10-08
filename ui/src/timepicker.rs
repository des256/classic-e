// E - UI - TimePicker
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Time picker.
pub struct TimePicker {
    r: Cell<Rect<i32>>,
}
