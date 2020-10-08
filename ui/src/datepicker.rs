// E - UI - DatePicker
// Desmond Germans, 2020

// A date picker allows the user to select a date.

use{
    crate::*,
    std::cell::Cell,
};

/// Date picker.
pub struct DatePicker {
    r: Cell<Rect<i32>>,
}

// TBD after other components are finished