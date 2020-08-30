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

pub enum MouseResult {
    Unprocessed,
    Processed,
    ProcessedCapture,
}

/// Widget abstraction trait.
pub trait Widget {
    fn get_rect(&self) -> Rect<i32>;
    fn set_rect(&mut self,r: Rect<i32>);
    fn calc_min_size(&self) -> Vec2<i32>;
    fn draw(&self,context: Vec2<i32>);
    fn handle_mouse_press(&mut self,b: MouseButton) -> MouseResult;
    fn handle_mouse_release(&mut self,b: MouseButton) -> MouseResult;
    fn handle_mouse_move(&mut self,p: Vec2<i32>) -> MouseResult;
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