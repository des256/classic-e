// E - UI - Action
// Desmond Germans, 2020

// An Action is a placeholder in the application with a specific function. An
// action can be represented as a menu item, toolbar item or button.

use {
    crate::*,
};

/// Action.
pub struct Action {
    _name: String,
    _icon: Texture2D<pixel::ARGB8>,
}

impl Action {
    pub fn new(graphics: &Rc<Graphics>,name: &str,icon: Mat<pixel::ARGB8>) -> Result<Action,SystemError> {
        Ok(Action {
            _name: name.to_string(),
            _icon: Texture2D::new_from_mat(&graphics,icon)?,
        })
    }
}