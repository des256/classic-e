// E - UI
// Desmond Germans, 2020

use crate::*;

pub enum HAlignment {
    Left,
    Center,
    Right,
    Fill,
}

pub enum VAlignment {
    Top,
    Center,
    Bottom,
    Fill,
}

pub trait Widget {
    fn draw(&self,gc: &GC,r: Rect<f32>);
    fn measure(&self) -> Vec2<f32>;
    fn halign(self,alignment: HAlignment) -> Self;
    fn valign(self,alignment: VAlignment) -> Self;
    fn padding(self) -> Self;
}

mod font;
pub use font::*;

mod ui;
pub use ui::*;

mod gc;
pub use gc::*;

mod widgetengine;
pub use widgetengine::*;

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
