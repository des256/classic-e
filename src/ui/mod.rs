// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use crate::*;

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

    /// Get rectangle for this widget.
    fn get_rect(&self) -> i32r;

    /// Set rectangle for this widget.
    /// 
    /// If the widget has children, the rectangles for the children are also updated accordingly.
    fn set_rect(&self,r: i32r);

    /// Calculate minimum size this widget needs.
    fn calc_min_size(&self) -> i32x2;

    /// Draw the widget.
    fn draw(&self,context: i32x2);

    /// Handle mouse button press.
    fn handle_mouse_press(&self,p: i32x2,b: MouseButton);

    /// Handle mouse button release.
    fn handle_mouse_release(&self,p: i32x2,b: MouseButton);

    /// Handle mouse pointer move. Returns whether or not widget captures the mouse.
    fn handle_mouse_move(&self,p: i32x2) -> bool;

    /// Handle mouse wheel.
    fn handle_mouse_wheel(&self,w: MouseWheel);
}

mod font;
pub use font::*;

mod ui;
pub use ui::*;

mod core;
pub use self::core::*;

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

mod book;
pub use book::*;

mod menubar;
pub use menubar::*;

mod menu;
pub use menu::*;