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

/// Widget event.
pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(Vec2<i32>,MouseButton),
    MouseRelease(Vec2<i32>,MouseButton),
    MouseWheel(MouseWheel),
    MouseMove(Vec2<i32>),
}

/// Widget orientation.
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Widget abstraction trait.
pub trait Widget {
    /// Get rectangle for this widget.
    fn rect(&self) -> Rect<i32>;

    /// Set rectangle for this widget. Done by the system for the top-level widgets, or by the parents to the children.
    fn set_rect(&self,r: Rect<i32>);

    /// Calculate minimum size this widget needs. Asked by the parents to organize the children via set_rect.
    fn calc_min_size(&self,draw: &Draw) -> Vec2<i32>;

    /// Draw the widget.
    fn draw(&self,draw: &Draw);

    /// Handle widget event.
    fn handle(&self,ui: &UI,window: &Window,draw: &Draw,event: Event);
}

mod accordeon;
pub use accordeon::*;

mod action;
pub use action::*;

mod book;
pub use book::*;

mod button;
pub use button::*;

mod colorpicker;
pub use colorpicker::*;

mod datepicker;
pub use datepicker::*;

mod draw;
pub use draw::*;

mod field;
pub use field::*;

mod filepicker;
pub use filepicker::*;

mod font;
pub use font::*;

mod grid;
pub use grid::*;

mod image;
pub use image::*;

mod list;
pub use list::*;

mod menu;
pub use menu::*;

mod menubar;
pub use menubar::*;

mod messagebox;
pub use messagebox::*;

mod progress;
pub use progress::*;

mod scrollbar;
pub use scrollbar::*;

mod scroller;
pub use scroller::*;

mod slider;
pub use slider::*;

mod splitter;
pub use splitter::*;

mod stack;
pub use stack::*;

mod stepper;
pub use stepper::*;

mod styles;
pub use styles::*;

mod text;
pub use text::*;

mod timepicker;
pub use timepicker::*;

mod toggle;
pub use toggle::*;

mod toolbar;
pub use toolbar::*;

mod tree;
pub use tree::*;

mod ui;
pub use ui::*;
