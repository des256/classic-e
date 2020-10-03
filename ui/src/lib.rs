// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use base::*;
use platform::*;
use gpu::*;

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
    fn rect(&self) -> Rect<i32>;

    /// Set rectangle for this widget. Done by the system for the top-level widgets, or by the parents to the children.
    fn set_rect(&self,r: Rect<i32>);

    /// Calculate minimum size this widget needs. Asked by the parents to organize the children via set_rect.
    fn calc_min_size(&self) -> Vec2<i32>;

    /// Draw the widget.
    fn draw(&self);

    /// Handle mouse button press.
    fn handle_mouse_press(&self,p: Vec2<i32>,b: MouseButton);

    /// Handle mouse button release.
    fn handle_mouse_release(&self,p: Vec2<i32>,b: MouseButton);

    /// Handle mouse pointer move. Returns whether or not widget captures the mouse.
    fn handle_mouse_move(&self,p: Vec2<i32>);

    /// Handle mouse wheel.
    fn handle_mouse_wheel(&self,w: MouseWheel);
}

mod font;
pub use font::*;

mod styles;
pub use styles::*;

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

mod book;
pub use book::*;

mod menubar;
pub use menubar::*;
