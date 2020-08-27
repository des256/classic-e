// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use crate::*;
use std::{
    rc::Rc,
};

pub const FONT_TEXTURE_SIZE: u32 = 1024;

/// Horizontal alignment.
#[derive(Copy,Clone)]
pub enum HAlignment {
    Left,
    Center,
    Right,
    Fill,
}

/// Vertical alignment.
#[derive(Copy,Clone)]
pub enum VAlignment {
    Top,
    Center,
    Bottom,
    Fill,
}

/// Widget abstraction trait.
pub trait Widget {

    /// Measure the widget.
    /// ## Returns
    /// Minimum dimensions for this widget.
    fn measure(&self) -> Vec2<i32> { vec2!(0,0) }

    /// Get the widget rect.
    fn get_rect(&self) -> Rect<i32> { rect!(0,0,0,0) }

    /// The widget rect changed.
    fn set_rect(&self,_r: Rect<i32>) { }

    /// Draw the widget.
    fn draw(&self) { }
    
    /// A key was pressed.
    /// ## Arguments
    /// * `key` - The key value.
    fn key_press(&self,key: u8) { }

    /// A key was released.
    /// ## Arguments
    /// * `key` - The key value.
    fn key_release(&self,key: u8) { }

    /// A mouse button was pressed.
    /// ## Arguments
    /// * `pos` - The mouse cursor position.
    /// * `button` - The mouse button.
    /// ## Returns
    /// * `true` - If mouse should be captured.
    /// * `false` - If not.
    fn mouse_press(&self,pos: Vec2<i32>,button: Mouse) -> bool { false }

    /// A mouse button was released.
    /// ## Arguments
    /// * `pos` - The mouse cursor position.
    /// * `button` - The mouse button.
    /// ## Returns
    /// * `true` - If mouse should be captured.
    /// * `false` - If not.
    fn mouse_release(&self,pos: Vec2<i32>,button: Mouse) -> bool { false }

    /// The mouse wheel was moved.
    /// ## Arguments
    /// * `wheel` - The wheel direction.
    fn mouse_wheel(&self,wheel: Wheel) { }

    /// The mouse was moved.
    /// ## Arguments
    /// * `pos` - The mouse cursor position.
    /// ## Returns
    /// * `true` - If mouse should be captured.
    /// * `false` - If not.
    fn mouse_move(&self,pos: Vec2<i32>) -> bool { false }
}

mod font;
pub use font::*;

mod ui;
pub use ui::*;

mod text;
pub use text::*;

mod hstack;
pub use hstack::*;

mod vstack;
pub use vstack::*;

mod image;
pub use image::*;

mod button;
pub use button::*;

mod toggle;
pub use toggle::*;

mod stepper;
pub use stepper::*;

mod slider;
pub use slider::*;

mod progress;
pub use progress::*;

mod field;
pub use field::*;

mod list;
pub use list::*;

mod book;
pub use book::*;
