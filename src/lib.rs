// E
// Desmond Germans, 2020

//! # E
//!
//! It's E. E for everything.

mod zeroone;
pub use zeroone::*;

mod mat;
pub use mat::*;

mod ten;
pub use ten::*;

#[macro_use]
mod vector;
pub use vector::*;

mod matrix;
pub use matrix::*;

#[macro_use]
mod rect;
pub use rect::*;

mod platform;
pub use platform::*;

pub mod pixel;

pub mod image;

pub mod gpu;

pub mod ui;
