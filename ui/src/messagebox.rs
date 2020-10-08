// E - UI - MessageBox
// Desmond Germans, 2020

use{
    crate::*,
    std::cell::Cell,
};

/// Message box.
pub struct MessageBox {
    r: Cell<Rect<i32>>,
}
