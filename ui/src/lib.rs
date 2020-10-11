// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use base::*;
use platform::*;
use gpu::*;
use std::rc::Rc;

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
    fn calc_min_size(&self) -> Vec2<i32>;

    /// Draw the widget.
    fn draw(&self);

    /// Handle widget events.
    fn keypress(&self,ui: &UI,window: &Rc<UIWindow>,k: u8);
    fn keyrelease(&self,ui: &UI,window: &Rc<UIWindow>,k: u8);
    fn mousepress(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool;
    fn mouserelease(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>,b: MouseButton) -> bool;
    fn mousemove(&self,ui: &UI,window: &Rc<UIWindow>,p: Vec2<i32>) -> bool;
    fn mousewheel(&self,ui: &UI,window: &Rc<UIWindow>,w: MouseWheel) -> bool;
}

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

mod text;
pub use text::*;

mod timepicker;
pub use timepicker::*;

mod toggle;
pub use toggle::*;

mod toolbar;
pub use toolbar::*;

mod tooltip;
pub use tooltip::*;

mod tree;
pub use tree::*;

mod ui;
pub use ui::*;
