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

    /// Measure the widget.
    /// ## Returns
    /// Minimum dimensions for this widget.
    fn measure(&self) -> Vec2<i32>;

    /// Handle an event on the widget.
    /// ## Arguments
    /// * `event` - Event to process.
    /// * `space` - The widget's rectangle.
    fn handle(&self,event: &Event,space: Rect<i32>);

    /// Draw the widget.
    /// ## Arguments
    /// * `window_size` - Size of the window to draw in.
    /// * `space` - Space to put this widget in.
    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>);
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
