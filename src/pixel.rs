// E - Pixel
// Desmond Germans, 2020

//! Pixel formats.

use crate::*;

pub trait Pixel: Copy + Clone + Zero {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self;
    fn from_vec4(v: Vec4<u8>) -> Self;
    fn as_vec4(&self) -> Vec4<u8>;
    // could also consider from_vec4f and as_vec4f for more exotic formats
}

/// 8-bit R pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct R8 {
    pub d: u8,
}

impl Pixel for R8 {
    fn from_rgba(r: u8,g: u8,b: u8,_a: u8) -> Self {
        R8 { d: (((r as u16) + (b as u16) + (g as u16)) / 3) as u8, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        R8 { d: v.x(), }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        Vec4::<u8>::new(self.d,self.d,self.d,255)
    }
}

impl Zero for R8 {
    /// Return black R8 pixel.
    fn zero() -> R8 {
        R8 { d: 0x00, }
    }
}

impl PartialEq<R8> for R8 {
    /// Check if two R8 pixels are equal.
    fn eq(&self,other: &R8) -> bool {
        self.d == other.d
    }
}

/// 8-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct R3G3B2 {
    d: u8,
}

impl Pixel for R3G3B2 {
    fn from_rgba(r: u8,g: u8,b: u8,_a: u8) -> Self {
        let r = (r & 0xE0) as u8;
        let g = ((g >> 2) & 0x1C) as u8;
        let b = ((b >> 6) & 0x03) as u8;
        R3G3B2 { d: r | g | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let r = (v.x() & 0xE0) as u8;
        let g = ((v.y() >> 2) & 0x1C) as u8;
        let b = ((v.z() >> 6) & 0x03) as u8;
        R3G3B2 { d: r | g | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let mut r = (self.d >> 5) as u32;
        let mut g = ((self.d >> 2) & 0x07) as u32;
        let mut b = (self.d & 0x03) as u32;
        r = (r << 5) | (r << 2) | (r >> 1);
        g = (g << 5) | (g << 2) | (g >> 1);
        b = (b << 6) | (b << 4) | (b << 2) | b;
        Vec4::<u8>::new(r as u8,g as u8,b as u8,255)
    }
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

/// 8-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct ARGB2 {
    d: u8,
}

impl Pixel for ARGB2 {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self {
        let r = ((r >> 6) & 0x03) as u8;
        let g = ((g >> 6) & 0x03) as u8;
        let b = (b & 0x03) as u8;
        let a = ((a >> 6) & 0x03) as u8;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let r = ((v.x() >> 6) & 0x03) as u8;
        let g = ((v.y() >> 6) & 0x03) as u8;
        let b = (v.z() & 0x03) as u8;
        let a = ((v.w() >> 6) & 0x03) as u8;
        ARGB2 { d: (a << 6) | (r << 4) | (g << 2) | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let mut r = ((self.d >> 4) & 0x03) as u8;
        let mut g = ((self.d >> 2) & 0x03) as u8;
        let mut b = (self.d & 0x03) as u8;
        let mut a = (self.d >> 6) as u8;
        r = (r << 6) | (r << 4) | (r << 2) | r;
        g = (g << 6) | (g << 4) | (g << 2) | g;
        b = (b << 6) | (b << 4) | (b << 2) | b;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        Vec4::<u8>::new(r,g,b,a)
    }
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

/// 16-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct R5G6B5 {
    d: u16,
}

impl Pixel for R5G6B5 {
    fn from_rgba(r: u8,g: u8,b: u8,_a: u8) -> Self {
        let r = ((r >> 3) & 0x001F) as u16;
        let g = ((g >> 2) & 0x003F) as u16;
        let b = ((b >> 3) & 0x001F) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let r = ((v.x() >> 3) & 0x001F) as u16;
        let g = ((v.y() >> 2) & 0x003F) as u16;
        let b = ((v.z() >> 3) & 0x001F) as u16;
        R5G6B5 { d: (r << 11) | (g << 5) | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let mut r = ((self.d >> 11) & 0x001F) as u8;
        let mut g = ((self.d >> 5) & 0x003F) as u8;
        let mut b = (self.d & 0x001F) as u8;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        Vec4::<u8>::new(r,g,b,255)
    }
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

/// 16-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct ARGB4 {
    d: u16,
}

impl Pixel for ARGB4 {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self {
        let r = ((r >> 4) & 0x000F) as u16;
        let g = ((g >> 4) & 0x000F) as u16;
        let b = ((b >> 4) & 0x000F) as u16;
        let a = ((a >> 4) & 0x000F) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let r = ((v.x() >> 4) & 0x000F) as u16;
        let g = ((v.y() >> 4) & 0x000F) as u16;
        let b = ((v.z() >> 4) & 0x000F) as u16;
        let a = ((v.w() >> 4) & 0x000F) as u16;
        ARGB4 { d: (a << 12) | (r << 8) | (g << 4) | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let mut r = ((self.d >> 8) & 0x000F) as u8;
        let mut g = ((self.d >> 4) & 0x000F) as u8;
        let mut b = (self.d & 0x000F) as u8;
        let mut a = ((self.d >> 12) & 0x000F) as u8;
        a = (a << 4) | a;
        r = (r << 4) | r;
        g = (g << 4) | g;
        b = (b << 4) | b;
        Vec4::<u8>::new(r,g,b,a)
    }
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

/// 16-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct A1RGB5 {
    d: u16,
}

impl Pixel for A1RGB5 {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self {
        let r = ((r >> 3) & 0x001F) as u16;
        let g = ((g >> 3) & 0x001F) as u16;
        let b = ((b >> 3) & 0x001F) as u16;
        let a = ((a >> 7) & 0x0001) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let r = ((v.x() >> 3) & 0x001F) as u16;
        let g = ((v.y() >> 3) & 0x001F) as u16;
        let b = ((v.z() >> 3) & 0x001F) as u16;
        let a = ((v.w() >> 7) & 0x0001) as u16;
        A1RGB5 { d: (a << 15) | (r << 10) | (g << 5) | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let mut r = ((self.d >> 10) & 0x001F) as u8;
        let mut g = ((self.d >> 5) & 0x001F) as u8;
        let mut b = (self.d & 0x001F) as u8;
        let mut a = ((self.d >> 15) & 0x0001) as u8;
        r = (r << 3) | (r >> 2);
        g = (g << 3) | (g >> 2);
        b = (b << 3) | (b >> 2);
        a = if a != 0 {
            0xFF
        }
        else {
            0x00
        };
        Vec4::<u8>::new(r,g,b,a)
    }
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


/// 24-bit RGB pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct RGB8 {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel for RGB8 {
    fn from_rgba(r: u8,g: u8,b: u8,_a: u8) -> Self {
        RGB8 { r: r,g: g,b: b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        RGB8 { r: v.x(),g: v.y(),b: v.z(), }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        Vec4::<u8>::new(self.r,self.g,self.b,255)
    }
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

/// 32-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct ARGB8 {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl Pixel for ARGB8 {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self {
        ARGB8 { a: a,r: r,g: g,b: b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        ARGB8 { a: v.w(),r: v.x(),g: v.y(),b: v.z(), }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        Vec4::<u8>::new(self.r,self.g,self.b,self.a)        
    }
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

/// 32-bit RGBA pixel format.
#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub struct A2RGB10 {
    d: u32,
}

impl Pixel for A2RGB10 {
    fn from_rgba(r: u8,g: u8,b: u8,a: u8) -> Self {
        let mut r = r as u32;
        let mut g = g as u32;
        let mut b = b as u32;
        let mut a = a as u32;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        a = a >> 6;
        A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
    }

    fn from_vec4(v: Vec4<u8>) -> Self {
        let mut r = v.x() as u32;
        let mut g = v.y() as u32;
        let mut b = v.z() as u32;
        let mut a = v.w() as u32;
        r = (r << 2) | (r >> 6);
        g = (g << 2) | (g >> 6);
        b = (b << 2) | (b >> 6);
        a = a >> 6;
        A2RGB10 { d: (a << 30) | (r << 20) | (g << 10) | b, }
    }

    fn as_vec4(&self) -> Vec4<u8> {
        let r = ((self.d >> 22) & 0x000000FF) as u8;
        let g = ((self.d >> 12) & 0x000000FF) as u8;
        let b = ((self.d >> 2) & 0x000000FF) as u8;
        let mut a = ((self.d >> 30) & 0x00000003) as u8;
        a = (a << 6) | (a << 4) | (a << 2) | a;
        Vec4::<u8>::new(r,g,b,a)        
    }
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
