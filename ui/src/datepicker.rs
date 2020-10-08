// E - UI - DatePicker
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Date picker.
pub struct DatePicker {
    r: Cell<Rect<i32>>,
}
