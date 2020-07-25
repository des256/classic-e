// E - Pixel
// Desmond Germans, 2020

//! Pixel formats.

use crate::*;

/*#[doc(hidden)]
trait Clamp1 {
    fn clamp1(self) -> Self;
}

macro_rules! impl_clamp1 (
    ($t:ty) => (
        impl Clamp1 for $t {
            fn clamp1(self) -> $t {
                if self < 0.0 { return 0.0; }
                if self > 1.0 { return 1.0; }
                self
            }            
        }
    )
);

impl_clamp1!(f32);
impl_clamp1!(f64);*/

/// 8-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct R3G3B2 {
    d: u8,
}

impl Zero for R3G3B2 {
    /// Return black R3G3B2 pixel.
    fn zero() -> R3G3B2 {
        R3G3B2 { d: 0x00, }
    }
}

impl PartialEq<R3G3B2> for R3G3B2 {
    /// Check if two R3G3B2 pixels are equal.
    fn eq(&self,other: &R3G3B2) -> bool {
        self.d == other.d
    }
}

impl From<R3G3B2> for u32 {
    /// Convert R3G3B2 pixel to binary ARGB code.
    fn from(c: R3G3B2) -> u32 {
        let mut r = (c.d >> 5) as u32;
        let mut g = ((c.d >> 2) & 0x07) as u32;
        let mut b = (c.d & 0x03) as u32;
        r = (r << 5) | (r << 2) | (r >> 1);
        g = (g << 5) | (g << 2) | (g >> 1);
        b = (b << 6) | (b << 4) | (b << 2) | b;
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for R3G3B2 {
    /// Convert binary ARGB code to R3G3B2 pixel.
    fn from(c: u32) -> R3G3B2 {
        let r = ((c >> 16) & 0xE0) as u8;
        let g = ((c >> 11) & 0x1C) as u8;
        let b = ((c >> 6) & 0x03) as u8;
        R3G3B2 { d: r | g | b, }
    }
}

impl From<R3G3B2> for Vec4<u8> {
    /// Convert R3G3B2 pixel to ARGB 4-vector of bytes.
    fn from(c: R3G3B2) -> Vec4<u8> {
        let mut r = (c.d >> 5) as u8;
        let mut g = ((c.d >> 2) & 0x07) as u8;
        let mut b = (c.d & 0x03) as u8;
        r = (r << 5) | (r << 2) | (r >> 1);
        g = (g << 5) | (g << 2) | (g >> 1);
        b = (b << 6) | (b << 4) | (b << 2) | b;
        vec4!(r,g,b,255)
    }
}

impl From<Vec4<u8>> for R3G3B2 {
    /// Convert ARGB 4-vector of bytes to R3G3B2 pixel.
    fn from(c: Vec4<u8>) -> R3G3B2 {
        let r = (c.x & 0xE0) as u8;
        let g = ((c.y >> 2) & 0x1C) as u8;
        let b = ((c.z >> 6) & 0x03) as u8;
        R3G3B2 { d: r | g | b, }
    }
}

macro_rules! impl_r3g3b2 (
    ($t:ty) => (
        impl From<R3G3B2> for Vec4<$t> {
            /// Convert R3G3B2 pixel to ARGB 4-vector of floats.
            fn from(c: R3G3B2) -> Vec4<$t> {
                let r = ((c.d >> 5) as $t) / 7.0;
                let g = (((c.d >> 2) & 0x07) as $t) / 7.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                vec4!(r,g,b,1.0)
            }
        }
        
        impl From<Vec4<$t>> for R3G3B2 {
            /// Convert ARGB 4-vector of floats to R3G3B2 pixel.
            fn from(c: Vec4<$t>) -> R3G3B2 {
                let r = ((c.x * 7.0) as u8) << 5;
                let g = ((c.y * 7.0) as u8) << 2;
                let b = (c.z * 3.0) as u8;
                R3G3B2 { d: r | g | b, }
            }
        }
    );
);

impl_r3g3b2!(f32);
impl_r3g3b2!(f64);

/// 8-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB2 {
    d: u8,
}

impl Zero for ARGB2 {
    /// Return black transparent ARGB2 pixel.
    fn zero() -> ARGB2 {
        ARGB2 { d: 0x00, }
    }
}

impl PartialEq<ARGB2> for ARGB2 {
    /// Check if two ARGB2 pixels are equal.
    fn eq(&self,other: &ARGB2) -> bool {
        self.d == other.d
    }
}

impl From<ARGB2> for u32 {
    /// Convert ARGB2 pixel to binary ARGB code.
    fn from(c: ARGB2) -> u32 {
        let mut r = ((c.d >> 4) & 0x03) as u32;
        let mut g = ((c.d >> 2) & 0x03) as u32;
        let mut b = (c.d & 0x03) as u32;
        let mut a = (c.d >> 6) as u32;
        r = (r << 6) | (r << 4) | (r << 2) | r;
        g = (g << 6) | (g << 4) | (g << 2) | g;
        b = (b << 6) | (b << 4) | (b << 2) | b;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for ARGB2 {
    /// Convert binary ARGB code to ARGB2 pixel.
    fn from(c: u32) -> ARGB2 {
        let r = ((c >> 22) & 0x03) as u8;
        let g = ((c >> 14) & 0x03) as u8;
        let b = (c & 0x03) as u8;
        let a = ((c >> 30) & 0x03) as u8;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }
}

impl From<ARGB2> for Vec4<u8> {
    /// Convert ARGB2 pixel to 4-vector of bytes.
    fn from(c: ARGB2) -> Vec4<u8> {
        let mut r = ((c.d >> 4) & 0x03) as u8;
        let mut g = ((c.d >> 2) & 0x03) as u8;
        let mut b = (c.d & 0x03) as u8;
        let mut a = (c.d >> 6) as u8;
        r = (r << 6) | (r << 4) | (r << 2) | r;
        g = (g << 6) | (g << 4) | (g << 2) | g;
        b = (b << 6) | (b << 4) | (b << 2) | b;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        vec4!(r,g,b,a)
    }
}

impl From<Vec4<u8>> for ARGB2 {
    /// Convert 4-vector of bytes to ARGB2 pixel.
    fn from(c: Vec4<u8>) -> ARGB2 {
        let r = ((c.x >> 6) & 0x03) as u8;
        let g = ((c.y >> 6) & 0x03) as u8;
        let b = (c.z & 0x03) as u8;
        let a = ((c.w >> 6) & 0x03) as u8;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }
}

macro_rules! impl_argb2 (
    ($t:ty) => (
        impl From<ARGB2> for Vec4<$t> {
            /// Convert ARGB2 pixel to 4-vector of floats.
            fn from(c: ARGB2) -> Vec4<$t> {
                let r = (((c.d >> 4) & 0x03) as $t) / 3.0;
                let g = (((c.d >> 2) & 0x03) as $t) / 3.0;
                let b = ((c.d & 0x03) as $t) / 3.0;
                let a = ((c.d >> 6) as $t) / 3.0;
                vec4!(r,g,b,a)
            }
        }

        impl From<Vec4<$t>> for ARGB2 {
            /// Convert 4-vector of floats to ARGB2 pixel.
            fn from(c: Vec4<$t>) -> ARGB2 {
                let r = ((c.x * 3.0) as u8) << 4;
                let g = ((c.y * 3.0) as u8) << 2;
                let b = (c.z * 3.0) as u8;
                let a = ((c.w * 3.0) as u8) << 6;
                ARGB2 { d: a | r | g | b, }
            }
        }
    );
);

impl_argb2!(f32);
impl_argb2!(f64);

/// 16-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct R5G6B5 {
    d: u16,
}

impl Zero for R5G6B5 {
    /// Return black R5G6B5 pixel.
    fn zero() -> R5G6B5 {
        R5G6B5 { d: 0x0000, }
    }
}

impl PartialEq<R5G6B5> for R5G6B5 {
    /// Check if two R5G6B5 pixels are equal.
    fn eq(&self,other: &R5G6B5) -> bool {
        self.d == other.d
    }
}

impl From<R5G6B5> for u32 {
    /// Convert R5G6B5 pixel to binary ARGB code.
    fn from(c: R5G6B5) -> u32 {
        let mut r = ((c.d >> 11) & 0x001F) as u32;
        let mut g = ((c.d >> 5) & 0x003F) as u32;
        let mut b = (c.d & 0x001F) as u32;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for R5G6B5 {
    /// Convert binary ARGB code to R5G6B5 pixel.
    fn from(c: u32) -> R5G6B5 {
        let r = ((c >> 19) & 0x001F) as u16;
        let g = ((c >> 10) & 0x003F) as u16;
        let b = ((c >> 3) & 0x001F) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }
}

impl From<R5G6B5> for Vec4<u8> {
    /// Convert R5G6B5 pixel to 4-vector of bytes.
    fn from(c: R5G6B5) -> Vec4<u8> {
        let mut r = ((c.d >> 11) & 0x001F) as u8;
        let mut g = ((c.d >> 5) & 0x003F) as u8;
        let mut b = (c.d & 0x001F) as u8;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        vec4!(r,g,b,255)
    }
}

impl From<Vec4<u8>> for R5G6B5 {
    /// Convert 4-vector of bytes to R5G6B5 pixel.
    fn from(c: Vec4<u8>) -> R5G6B5 {
        let r = ((c.x >> 3) & 0x001F) as u16;
        let g = ((c.y >> 2) & 0x003F) as u16;
        let b = ((c.z >> 3) & 0x001F) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }
}

macro_rules! impl_r5g6b5 (
    ($t:ty) => (
        impl From<R5G6B5> for Vec4<$t> {
            /// Convert R5G6B5 pixel to 4-vector of floats.
            fn from(c: R5G6B5) -> Vec4<$t> {
                let r = (((c.d >> 11) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x003F) as $t) / 63.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                vec4!(r,g,b,1.0)
            }
        }

        impl From<Vec4<$t>> for R5G6B5 {
            /// Convert 4-vector of floats to R5G6B5 pixel.
            fn from(c: Vec4<$t>) -> R5G6B5 {
                let r = ((c.x * 31.0) as u16) << 11;
                let g = ((c.y * 63.0) as u16) << 5;
                let b = (c.z * 31.0) as u16;
                R5G6B5 { d: r | g | b, }
            }
        }
    );
);

impl_r5g6b5!(f32);
impl_r5g6b5!(f64);

/// 16-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB4 {
    d: u16,
}

impl Zero for ARGB4 {
    /// Return black transparent ARGB4 pixel.
    fn zero() -> ARGB4 {
        ARGB4 { d: 0x0000, }
    }
}

impl PartialEq<ARGB4> for ARGB4 {
    /// Check if two ARGB4 pixels are equal.
    fn eq(&self,other: &ARGB4) -> bool {
        self.d == other.d
    }
}

impl From<ARGB4> for u32 {
    /// Convert ARGB4 pixel to binary ARGB code.
    fn from(c: ARGB4) -> u32 {
        let mut r = ((c.d >> 8) & 0x000F) as u32;
        let mut g = ((c.d >> 4) & 0x000F) as u32;
        let mut b = (c.d & 0x000F) as u32;
        let mut a = ((c.d >> 12) & 0x000F) as u32;
        a = (a << 4) | a;
        r = (r << 4) | r;
        g = (g << 4) | g;
        b = (b << 4) | b;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for ARGB4 {
    /// Convert binary ARGB code to ARGB4 pixel.
    fn from(c: u32) -> ARGB4 {
        let r = ((c >> 20) & 0x000F) as u16;
        let g = ((c >> 12) & 0x000F) as u16;
        let b = ((c >> 4) & 0x000F) as u16;
        let a = ((c >> 28) & 0x000F) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }
}

impl From<ARGB4> for Vec4<u8> {
    /// Convert ARGB4 pixel to 4-vector of bytes.
    fn from(c: ARGB4) -> Vec4<u8> {
        let mut r = ((c.d >> 8) & 0x000F) as u8;
        let mut g = ((c.d >> 4) & 0x000F) as u8;
        let mut b = (c.d & 0x000F) as u8;
        let mut a = ((c.d >> 12) & 0x000F) as u8;
        a = (a << 4) | a;
        r = (r << 4) | r;
        g = (g << 4) | g;
        b = (b << 4) | b;
        vec4!(r,g,b,a)
    }
}

impl From<Vec4<u8>> for ARGB4 {
    /// Convert 4-vector of bytes to ARGB4 pixel.
    fn from(c: Vec4<u8>) -> ARGB4 {
        let r = ((c.x >> 4) & 0x000F) as u16;
        let g = ((c.y >> 4) & 0x000F) as u16;
        let b = ((c.z >> 4) & 0x000F) as u16;
        let a = ((c.w >> 4) & 0x000F) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }
}

macro_rules! impl_argb4 (
    ($t:ty) => (
        impl From<ARGB4> for Vec4<$t> {
            /// Convert ARGB4 pixel to 4-vector of floats.
            fn from(c: ARGB4) -> Vec4<$t> {
                let r = (((c.d >> 8) & 0x000F) as $t) / 15.0;
                let g = (((c.d >> 4) & 0x000F) as $t) / 15.0;
                let b = ((c.d & 0x000F) as $t) / 15.0;
                let a = (((c.d >> 12) & 0x000F) as $t) / 15.0;
                vec4!(r,g,b,a)
            }
        }

        impl From<Vec4<$t>> for ARGB4 {
            /// Convert 4-vector of floats to ARGB4 pixel.
            fn from(c: Vec4<$t>) -> ARGB4 {
                let r = ((c.x * 15.0) as u16) << 8;
                let g = ((c.y * 15.0) as u16) << 4;
                let b = (c.z * 15.0) as u16;
                let a = ((c.w * 15.0) as u16) << 12;
                ARGB4 { d: a | r | g | b, }
            }
        }
    );
);

impl_argb4!(f32);
impl_argb4!(f64);

/// 16-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct A1RGB5 {
    d: u16,
}

impl Zero for A1RGB5 {
    /// Return black transparent A1RGB5 pixel.
    fn zero() -> A1RGB5 {
        A1RGB5 { d: 0x0000, }
    }
}

impl PartialEq<A1RGB5> for A1RGB5 {
    /// Check if two A1RGB5 pixels are equal.
    fn eq(&self,other: &A1RGB5) -> bool {
        self.d == other.d
    }
}

impl From<A1RGB5> for u32 {
    /// Convert A1RGB5 pixel to binary ARGB code.
    fn from(c: A1RGB5) -> u32 {
        let mut r = ((c.d >> 10) & 0x001F) as u32;
        let mut g = ((c.d >> 5) & 0x001F) as u32;
        let mut b = (c.d & 0x001F) as u32;
        let mut a = ((c.d >> 15) & 0x0001) as u32;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        a = if a != 0 {
            0xFF
        }
        else {
            0x00
        };
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for A1RGB5 {
    /// Convert binary ARGB code to A1RGB5 pixel.
    fn from(c: u32) -> A1RGB5 {
        let r = ((c >> 19) & 0x001F) as u16;
        let g = ((c >> 11) & 0x001F) as u16;
        let b = ((c >> 3) & 0x001F) as u16;
        let a = ((c >> 31) & 0x0001) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }
}

impl From<A1RGB5> for Vec4<u8> {
    /// Convert A1RGB5 pixel to 4-vector of bytes.
    fn from(c: A1RGB5) -> Vec4<u8> {
        let mut r = ((c.d >> 10) & 0x001F) as u8;
        let mut g = ((c.d >> 5) & 0x001F) as u8;
        let mut b = (c.d & 0x001F) as u8;
        let mut a = ((c.d >> 15) & 0x0001) as u8;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        a = if a != 0 {
            0xFF
        }
        else {
            0x00
        };
        vec4!(r,g,b,a)
    }
}

impl From<Vec4<u8>> for A1RGB5 {
    /// Convert 4-vector of bytes to A1RGB5 pixel.
    fn from(c: Vec4<u8>) -> A1RGB5 {
        let r = ((c.x >> 3) & 0x001F) as u16;
        let g = ((c.y >> 3) & 0x001F) as u16;
        let b = ((c.z >> 3) & 0x001F) as u16;
        let a = ((c.w >> 7) & 0x0001) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }
}

macro_rules! impl_a1rgb5 (
    ($t:ty) => (
        impl From<A1RGB5> for Vec4<$t> {
            /// Convert A1RGB5 pixel to 4-vector of floats.
            fn from(c: A1RGB5) -> Vec4<$t> {
                let r = (((c.d >> 10) & 0x001F) as $t) / 31.0;
                let g = (((c.d >> 5) & 0x001F) as $t) / 31.0;
                let b = ((c.d & 0x001F) as $t) / 31.0;
                let a = if (c.d & 0x8000) == 0x8000 { 1.0 } else { 0.0 };
                vec4!(r,g,b,a)
            }
        }

        impl From<Vec4<$t>> for A1RGB5 {
            /// Convert 4-vector of floats to A1RGB5 pixel.
            fn from(c: Vec4<$t>) -> A1RGB5 {
                let r = ((c.x * 31.0) as u16) << 10;
                let g = ((c.y * 31.0) as u16) << 5;
                let b = (c.z * 31.0) as u16;
                let a = if c.w >= 0.5 { 0x8000u16 } else { 0x0000u16 };
                A1RGB5 { d: a | r | g | b, }
            }
        }
    );
);

impl_a1rgb5!(f32);
impl_a1rgb5!(f64);

/// 24-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct RGB8 {
    r: u8,
    g: u8,
    b: u8,
}

impl Zero for RGB8 {
    /// Return black RGB8 pixel.
    fn zero() -> RGB8 {
        RGB8 { r: 0x00,g: 0x00,b: 0x00, }
    }
}

impl PartialEq<RGB8> for RGB8 {
    /// Check if two RGB8 pixels are equal.
    fn eq(&self,other: &RGB8) -> bool {
        (self.r == other.r) &&
        (self.g == other.g) &&
        (self.b == other.b)
    }
}

impl From<RGB8> for u32 {
    /// Convert RGB8 pixel to binary ARGB code.
    fn from(c: RGB8) -> u32 {
        let r = c.r as u32;
        let g = c.g as u32;
        let b = c.b as u32;
        0xFF000000 | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for RGB8 {
    /// Convert binary ARGB code to RGB8 pixel.
    fn from(c: u32) -> RGB8 {
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8) & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        RGB8 { r: r,g: g,b: b, }
    }
}

impl From<RGB8> for Vec4<u8> {
    /// Convert RGB8 pixel to 4-vector of bytes.
    fn from(c: RGB8) -> Vec4<u8> {
        vec4!(c.r,c.g,c.b,255)
    }
}

impl From<Vec4<u8>> for RGB8 {
    /// Convert 4-vector of bytes to RGB8 pixel.
    fn from(c: Vec4<u8>) -> RGB8 {
        RGB8 { r: c.x,g: c.y,b: c.z, }
    }
}

macro_rules! impl_rgb8 (
    ($t:ty) => (
        impl From<RGB8> for Vec4<$t> {
            /// Convert RGB8 pixel to 4-vector of floats.
            fn from(c: RGB8) -> Vec4<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                vec4!(r,g,b,1.0)
            }
        }

        impl From<Vec4<$t>> for RGB8 {
            /// Convert 4-vector of floats to RGB8 pixel.
            fn from(c: Vec4<$t>) -> RGB8 {
                let r = (c.x * 255.0) as u8;
                let g = (c.y * 255.0) as u8;
                let b = (c.z * 255.0) as u8;
                RGB8 { r: r,g: g,b: b, }
            }
        }
    );
);

impl_rgb8!(f32);
impl_rgb8!(f64);

/// 32-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct ARGB8 {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl Zero for ARGB8 {
    /// Return black transparent ARGB8 pixel.
    fn zero() -> ARGB8 {
        ARGB8 { r: 0x00,g: 0x00,b: 0x00,a: 0x00, }
    }
}

impl PartialEq<ARGB8> for ARGB8 {
    /// Check if two ARGB8 pixels are equal.
    fn eq(&self,other: &ARGB8) -> bool {
        (self.r == other.r) &&
        (self.g == other.g) &&
        (self.b == other.b) &&
        (self.a == other.a)
    }
}

impl From<ARGB8> for u32 {
    /// Convert ARGB8 pixel to binary ARGB code.
    fn from(c: ARGB8) -> u32 {
        let r = c.r as u32;
        let g = c.g as u32;
        let b = c.b as u32;
        let a = c.a as u32;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for ARGB8 {
    /// Convert binary ARGB code to ARGB8 pixel.
    fn from(c: u32) -> ARGB8 {
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8) & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        let a = ((c >> 24) & 0xFF) as u8;
        ARGB8 { a: a,r: r,g: g,b: b, }
    }
}

impl From<ARGB8> for Vec4<u8> {
    /// Convert ARGB8 pixel to 4-vector of bytes.
    fn from(c: ARGB8) -> Vec4<u8> {
        vec4!(c.r,c.g,c.b,c.a)
    }
}

impl From<Vec4<u8>> for ARGB8 {
    /// Convert 4-vector of bytes to ARGB8 pixel.
    fn from(c: Vec4<u8>) -> ARGB8 {
        ARGB8 { a: c.w,r: c.x,g: c.y,b: c.z, }
    }
}

macro_rules! impl_argb8 (
    ($t:ty) => (
        impl From<ARGB8> for Vec4<$t> {
            /// Convert ARGB8 pixel to 4-vector of floats.
            fn from(c: ARGB8) -> Vec4<$t> {
                let r = (c.r as $t) / 255.0;
                let g = (c.g as $t) / 255.0;
                let b = (c.b as $t) / 255.0;
                let a = (c.a as $t) / 255.0;
                vec4!(r,g,b,a)
            }
        }

        impl From<Vec4<$t>> for ARGB8 {
            /// Convert 4-vector of floats to ARGB8 pixel.
            fn from(c: Vec4<$t>) -> ARGB8 {
                let r = (c.x * 255.0) as u8;
                let g = (c.y * 255.0) as u8;
                let b = (c.z * 255.0) as u8;
                let a = (c.w * 255.0) as u8;
                ARGB8 { a: a,r: r,g: g,b: b, }
            }
        }
    );
);

impl_argb8!(f32);
impl_argb8!(f64);

/// 32-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct A2RGB10 {
    d: u32,
}

impl Zero for A2RGB10 {
    /// Return black transparent A2RGB10 pixel.
    fn zero() -> A2RGB10 {
        A2RGB10 { d: 0x00000000, }
    }
}

impl PartialEq<A2RGB10> for A2RGB10 {
    /// Check if two A2RGB10 pixels are equal.
    fn eq(&self,other: &A2RGB10) -> bool {
        self.d == other.d
    }
}

impl From<A2RGB10> for u32 {
    /// Convert A2RGB10 pixel to binary ARGB code.
    fn from(c: A2RGB10) -> u32 {
        let r = ((c.d >> 22) & 0x000000FF) as u32;
        let g = ((c.d >> 12) & 0x000000FF) as u32;
        let b = ((c.d >> 2) & 0x000000FF) as u32;
        let mut a = ((c.d >> 30) & 0x00000003) as u32;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<u32> for A2RGB10 {
    /// Convert binary ARGB code to A2RGB10 pixel.
    fn from(c: u32) -> A2RGB10 {
        let mut r = (c >> 16) & 0x000000FF;
        let mut g = (c >> 8) & 0x000000FF;
        let mut b = c & 0x000000FF;
        let mut a = (c >> 24) & 0x00000003;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        a = a >> 6;
        A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
    }
}

macro_rules! impl_a2rgb10 (
    ($t:ty) => (
        impl From<A2RGB10> for Vec4<$t> {
            /// Convert A2RGB10 pixel to 4-vector of floats.
            fn from(c: A2RGB10) -> Vec4<$t> {
                let r = (((c.d >> 20) & 0x000003FF) as $t) / 1023.0;
                let g = (((c.d >> 10) & 0x000003FF) as $t) / 1023.0;
                let b = ((c.d & 0x000003FF) as $t) / 1023.0;
                let a = (((c.d >> 30) & 0x00000003) as $t) / 3.0;
                vec4!(r,g,b,a)
            }
        }

        impl From<Vec4<$t>> for A2RGB10 {
            /// Convert 4-vector of floats to A2RGB10 pixel.
            fn from(c: Vec4<$t>) -> A2RGB10 {
                let r = ((c.x * 1023.0) as u32) << 20;
                let g = ((c.y * 1023.0) as u32) << 10;
                let b = (c.z * 1023.0) as u32;
                let a = ((c.w * 3.0) as u32) << 30;
                A2RGB10 { d: a | r | g | b, }
            }
        }
    );
);

impl_a2rgb10!(f32);
impl_a2rgb10!(f64);