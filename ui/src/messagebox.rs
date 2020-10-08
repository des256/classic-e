// E - UI - MessageBox
// Desmond Germans, 2020

// A message box is a window frame with a specific message for the user, and
// buttons to accept/cancel/etc. the message.

use{
    crate::*,
    std::cell::Cell,
};

/// Message box.
pub struct MessageBox {
    r: Cell<Rect<i32>>,
}
