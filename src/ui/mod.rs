// E - UI
// Desmond Germans, 2020

//! UI Widgets.

use crate::*;
use std::rc::Rc;
use gl::types::{
    GLuint,
    GLvoid,
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

/// Hover background color property trait.
pub trait HoverBackColor {
    fn set_hover_back_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T>;
}

#[macro_export]
macro_rules! impl_hover_back_color (
    ($t:ty) => (
        impl ui::HoverBackColor for $t {
            fn set_hover_back_color<T: ColorParameter>(&self,c: T) where pixel::ARGB8: From<T> {
                self.hover_back_color.set(pixel::ARGB8::from(c));
            }
        }
    );
);

/// Padding property trait.
pub trait Padding {
    fn set_padding(&self,p: Vec2<i32>);
}

#[macro_export]
macro_rules! impl_padding {
    ($t:ty) => {
        impl ui::Padding for $t {
            fn set_padding(&self,p: Vec2<i32>) {
                self.padding.set(p);
            }
        }
    }
}

/// Inner padding property trait.
pub trait InnerPadding {
    fn set_inner_padding(&self,p: Vec2<i32>);
}

#[macro_export]
macro_rules! impl_inner_padding {
    ($t:ty) => {
        impl ui::InnerPadding for $t {
            fn set_inner_padding(&self,p: Vec2<i32>) {
                self.inner_padding.set(p);
            }
        }
    }
}

/// Text and font property trait.
pub trait TextFont {
    fn set_text(&self,text: &str);
    fn set_font(&self,font: &Rc<Font>);
}

#[macro_export]
macro_rules! impl_textfont {
    ($t:ty) => {
        impl ui::TextFont for $t {
            fn set_text(&self,text: &str) {
                *self.text.borrow_mut() = String::from(text);
            }

            fn set_font(&self,font: &Rc<ui::Font>) {
                *self.font.borrow_mut() = Rc::clone(font);
            }
        }
    }
}

/// Horizontal alignment property trait.
pub trait HAlign {
    fn set_halign(&self,align: HAlignment);
}

#[macro_export]
macro_rules! impl_halign {
    ($t:ty) => {
        impl ui::HAlign for $t {
            fn set_halign(&self,align: ui::HAlignment) {
                self.halign.set(align);
            }
        }
    }
}

/// Vertical alignment property trait.
pub trait VAlign {
    fn set_valign(&self,align: VAlignment);
}

#[macro_export]
macro_rules! impl_valign {
    ($t:ty) => {
        impl ui::VAlign for $t {
            fn set_valign(&self,align: ui::VAlignment) {
                self.valign.set(align);
            }
        }
    }
}
