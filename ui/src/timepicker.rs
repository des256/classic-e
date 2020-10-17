// E - UI - TimePicker
// Desmond Germans, 2020

// A time picker allows the user to select a date.

use{
    crate::*,
    std::cell::Cell,
};

/// Time picker.
pub struct TimePicker {
    _r: Cell<Rect<i32>>,
}

// TBD after other components are finished