// E - UI - FilePicker
// Desmond Germans, 2020

// A file picker allows the user to select one or more files.

use{
    crate::*,
    std::cell::Cell,
};

/// File picker.
pub struct FilePicker {
    r: Cell<Rect<i32>>,
}

// TBD after other components are finished