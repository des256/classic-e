// E - UI - Action
// Desmond Germans, 2020

use {
    crate::*,
};

/// Action.
pub struct Action {
    name: String,
    icon: Texture2D<pixel::ARGB8>,
}

impl Action {
    pub fn new(graphics: &Graphics,name: &str,icon: Mat<pixel::ARGB8>) -> Result<Action,SystemError> {
        Ok(Action {
            name: name.to_string(),
            icon: graphics.create_texture2d_from_mat(icon)?,
        })
    }
}