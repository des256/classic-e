// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use crate::*;
use std::rc::Rc;

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

    /// Draw the widget in the given rectangle.
    /// # Arguments
    /// * `dc` - DC to use.
    /// * `r` - Rectangle to draw the widget in.
    fn draw(&self,dc: &Rc<DC>,r: Rect<f32>);

    /// Measure the widget.
    /// # Returns
    /// Minimum dimensions for this widget.
    fn measure(&self) -> Vec2<f32>;
}

mod font;
pub use font::*;

mod ui;
pub use ui::*;

mod dc;
pub use dc::*;

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

mod page;
pub use page::*;
