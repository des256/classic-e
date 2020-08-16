// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use crate::*;

use gl::types::{
    GLuint,
    GLvoid,
};

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

// TODO: the more tightly packed UIRect will come later
#[derive(Copy,Clone)]
#[repr(C)]
pub struct UIRect {
    pub(crate) r: Vec4<f32>,  // x, y, w, h
    pub(crate) t: Vec4<f32>,  // x, y, w, h
    pub(crate) fbdq: Vec4<u32>,  // f, b, d, q
}

// TODO: this means that specifically only OpenGL is supported for now; solve this later
impl gpu::GLVertex for UIRect {
    fn bind() -> Vec<GLuint> {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,48,0 as *const GLvoid);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,4,gl::FLOAT,gl::FALSE,48,16 as *const GLvoid);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribIPointer(2,4,gl::UNSIGNED_INT,48,32 as *const GLvoid);
        }
        vec![0,1,2]
    }

    fn len() -> isize {
        48
    }
}

pub enum HandleResult {
    Unhandled,
    Handled,
    HandledRebuild,
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
    /// ## Returns
    /// * `Unhandled` - The widget has not handled the event.
    /// * `Handled` - The widget handled the event, nothing needs to change.
    /// * `HandledRebuld` - The widget handled the event and needs to be rebuilt.
    fn handle(&self,event: &Event,space: Rect<i32>) -> HandleResult;

    /// Add UIRects to a vertex buffer.
    /// ## Arguments
    /// * `buffer` - Vertex buffer to add to. The current size of this buffer is where this widget's portion of the buffer starts.
    /// * `space` - Space to put this widget in.
    fn build(&self,buffer: &mut Vec<UIRect>,space: Rect<i32>);
}

/// Main color property trait.
pub trait Color {
    fn set_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T>;
}

#[macro_export]
macro_rules! impl_color (
    ($t:ty) => (
        impl ui::Color for $t {
            fn set_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T> {
                self.color.set(pixel::ARGB8::from(c));
            }
        }
    );
);

/// Background color property trait.
pub trait BackColor {
    fn set_back_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T>;
}

#[macro_export]
macro_rules! impl_back_color (
    ($t:ty) => (
        impl ui::BackColor for $t {
            fn set_back_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T> {
                self.back_color.set(pixel::ARGB8::from(c));
            }
        }
    );
);

/// Padding property trait.
pub trait Padding {
    fn set_padding(&self,p: &Vec2<i32>);
}

#[macro_export]
macro_rules! impl_padding {
    ($t:ty) => {
        impl ui::Padding for $t {
            fn set_padding(&self,p: &Vec2<i32>) {
                self.padding.set(*p);
            }
        }
    }
}

mod texture2darrayatlas;
pub use texture2darrayatlas::*;

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

mod page;
pub use page::*;
