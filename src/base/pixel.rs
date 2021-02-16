// E - PixelFormat
// Desmond Germans, 2020

//! Pixel formats.

use crate::*;

pub trait Pixel: Copy + Clone + Zero {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8);
    fn get(&self) -> (u8,u8,u8,u8);
}

#[derive(Copy,Clone)]
pub struct R5G6B5UN { d: u16, }
impl Zero for R5G6B5UN { fn zero() -> Self { R5G6B5UN { d: 0x0000 } } }
impl Pixel for R5G6B5UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let r = (r >> 3) as u16;
        let g = (g >> 2) as u16;
        let b = (b >> 3) as u16;
        self.d = (r << 11) | (g << 5) | b;
    }
    fn get(&self) -> (u8,u8,u8,u8) {
        let r = self.d >> 11;
        let g = (self.d >> 5) & 0x3F;
        let b = self.d & 0x1F;
        let r = ((r << 3) | (r >> 2)) as u8;
        let g = ((g << 2) | (g >> 4)) as u8;
        let b = ((b << 3) | (b >> 2)) as u8;
        (r,g,b,255)
    }
}

#[derive(Copy,Clone)]
pub struct A1RGB5UN { d: u16, }
impl Zero for A1RGB5UN { fn zero() -> Self { A1RGB5UN { d: 0x0000 } } }
impl Pixel for A1RGB5UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let r = (r >> 3) as u16;
        let g = (g >> 3) as u16;
        let b = (b >> 3) as u16;
        let a = (a >> 7) as u16;
        self.d = (a << 15) | (r << 10) | (g << 5) | b;
    }
    fn get(&self) -> (u8,u8,u8,u8) {
        let r = (self.d >> 10) & 0x1F;
        let g = (self.d >> 5) & 0x1F;
        let b = self.d & 0x1F;
        let a = if (self.d & 0x8000) != 0 { 0xFFu8 } else { 0x00u8 };
        let r = ((r << 3) | (r >> 2)) as u8;
        let g = ((g << 3) | (g >> 2)) as u8;
        let b = ((b << 3) | (b >> 2)) as u8;
        (r,g,b,a)
    }
}

pub struct R8UN { r: u8, }
pub struct R8IN { r: i8, }
pub struct R8U { r: u8, }
pub struct R8I { r: i8, }
pub struct RG8UN { r: u8,g: u8, }
pub struct RG8IN { r: i8,g: i8, }
pub struct RG8U { r: u8,g: u8, }
pub struct RG8I { r: i8,g: i8, }

#[derive(Copy,Clone)]
pub struct RGB8UN { r: u8,g: u8,b: u8, }
impl Zero for RGB8UN { fn zero() -> Self { RGB8UN { r: 0x00,g: 0x00,b: 0x00, } } }
impl Pixel for RGB8UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) { self.r = r; self.g = g; self.b = b; }
    fn get(&self) -> (u8,u8,u8,u8) { (self.r,self.g,self.b,0xFF) }
}

pub struct RGB8IN { r: i8,g: i8,b: i8, }
pub struct RGB8U { r: u8,g: u8,b: u8, }
pub struct RGB8I { r: i8,g: i8,b: i8, }

#[derive(Copy,Clone)]
pub struct BGR8UN { b: u8,g: u8,r: u8, }
impl Zero for BGR8UN { fn zero() -> Self { BGR8UN { b: 0x00,g: 0x00,r: 0x00, } } }
impl Pixel for BGR8UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) { self.r = r; self.g = g; self.b = b; }
    fn get(&self) -> (u8,u8,u8,u8) { (self.r,self.g,self.b,0xFF) }
}

pub struct BGR8IN { b: i8,g: i8,r: i8, }
pub struct BGR8U { b: u8,g: u8,r: u8, }
pub struct BGR8I { b: i8,g: i8,r: i8, }

#[derive(Copy,Clone)]
pub struct RGBA8UN { r: u8,g: u8,b: u8,a: u8, }
impl Zero for RGBA8UN { fn zero() -> Self { RGBA8UN { r: 0x00,g: 0x00,b: 0x00,a: 0x00, } } }
impl Pixel for RGBA8UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) { self.r = r; self.g = g; self.b = b; self.a = a; }
    fn get(&self) -> (u8,u8,u8,u8) { (self.r,self.g,self.b,self.a) }
}

pub struct RGBA8IN { r: i8,g: i8,b: i8,a: i8, }
pub struct RGBA8U { r: u8,g: u8,b: u8,a: u8, }
pub struct RGBA8I { r: i8,g: i8,b: i8,a: i8, }

#[derive(Copy,Clone)]
pub struct BGRA8UN { b: u8,g: u8,r: u8,a: u8, }
impl Zero for BGRA8UN { fn zero() -> Self { BGRA8UN { b: 0x00,g: 0x00,r: 0x00,a: 0x00, } } }
impl Pixel for BGRA8UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) { self.r = r; self.g = g; self.b = b; }
    fn get(&self) -> (u8,u8,u8,u8) { (self.r,self.g,self.b,self.a) }
}

pub struct BGRA8IN { b: i8,g: i8,r: i8,a: i8, }
pub struct BGRA8U { b: u8,g: u8,r: u8,a: u8, }
pub struct BGRA8I { b: i8,g: i8,r: i8,a: i8, }

#[derive(Copy,Clone)]
pub struct ABGR8UN { a: u8,b: u8,g: u8,r: u8, }
impl Zero for ABGR8UN { fn zero() -> Self { ABGR8UN { a: 0x00,b: 0x00,g: 0x00,r: 0x00, } } }
impl Pixel for ABGR8UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) { self.r = r; self.g = g; self.b = b; }
    fn get(&self) -> (u8,u8,u8,u8) { (self.r,self.g,self.b,self.a) }
}

pub struct ABGR8IN { a: i8,b: i8,g: i8,r: i8, }
pub struct ABGR8U { a: u8,b: u8,g: u8,r: u8, }
pub struct ABGR8I { a: i8,b: i8,g: i8,r: i8, }

#[derive(Copy,Clone)]
pub struct A2RGB10UN { d: u32, }
impl Zero for A2RGB10UN { fn zero() -> Self { A2RGB10UN { d: 0x00000000, } } }
impl Pixel for A2RGB10UN {
    fn set(&mut self,r: u8,g: u8,b: u8,a: u8) {
        let r = ((r << 2) | (r >> 6)) as u32;
        let g = ((g << 2) | (g >> 6)) as u32;
        let b = ((b << 2) | (b >> 6)) as u32;
        let a = (a >> 6) as u32;
        self.d = (a << 30) | (r << 20) | (g << 10) | b;
    }
    fn get(&self) -> (u8,u8,u8,u8) {
        let r = ((self.d >> 20) & 1023) >> 2;
        let g = ((self.d >> 10) & 1023) >> 2;
        let b = (self.d & 1023) >> 2;
        let a = self.d >> 30;
        let a = (a << 6) | (a << 4) | (a << 2) | a;
        (r as u8,g as u8,b as u8,a as u8)
    }
}

pub struct R16UN { r: u16, }
pub struct R16IN { r: i16, }
pub struct R16U { r: u16, }
pub struct R16I { r: i16, }
pub struct RG16UN { r: u16,g: u16, }
pub struct RG16IN { r: i16,g: i16, }
pub struct RG16U { r: u16,g: u16, }
pub struct RG16I { r: i16,g: i16, }
pub struct RGB16UN { r: u16,g: u16,b: u16, }
pub struct RGB16IN { r: i16,g: i16,b: i16, }
pub struct RGB16U { r: u16,g: u16,b: u16, }
pub struct RGB16I { r: i16,g: i16,b: i16, }
pub struct RGBA16UN { r: u16,g: u16,b: u16,a: u16, }
pub struct RGBA16IN { r: i16,g: i16,b: i16,a: i16, }
pub struct RGBA16U { r: u16,g: u16,b: u16,a: u16, }
pub struct RGBA16I { r: i16,g: i16,b: i16,a: i16, }
pub struct RG11B10F { d: u32, }
pub struct RGB9E5F { d: u32, }
