// E - Image
// Desmond Germans, 2020

use crate::Pixel;
use std::marker::PhantomData;
use crate::Zero;
use crate::usize_2;

pub struct Image<T> {
    pub size: usize_2,
    pub data: Box<[T]>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Copy + Zero> Image<T> {
    pub fn new(size: usize_2) -> Image<T> {
        Image {
            size: size,
            data: vec![T::zero(); (size.x * size.y) as usize].into_boxed_slice(),
            phantom: PhantomData,
        }
    }

    pub fn pixel(&self,p: usize_2) -> T {
        self.data[(p.y * self.size.x + p.x) as usize]
    }

    pub fn set_pixel(&mut self,p: usize_2,v: T) {
        self.data[(p.y * self.size.x + p.x) as usize] = v;
    }
}

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod tga;
pub mod gif;
pub mod pbm;
pub mod tiff;
pub mod xbm;
pub mod webp;

#[allow(dead_code)]
pub fn test(src: &[u8]) -> Option<(u32,u32)> {
    if let Some(size) = bmp::test(src) {
        Some(size)
    }
    else if let Some(size) = png::test(src) {
        Some(size)
    }
    else if let Some(size) = jpeg::test(src) {
        Some(size)
    }
    else if let Some(size) = gif::test(src) {
        Some(size)
    }
    else if let Some(size) = tga::test(src) {
        Some(size)
    }
    else if let Some(size) = tiff::test(src) {
        Some(size)
    }
    else if let Some(size) = pbm::test(src) {
        Some(size)
    }
    else if let Some(size) = xbm::test(src) {
        Some(size)
    }
    else if let Some(size) = webp::test(src) {
        Some(size)
    }
    else {
        None
    }
}

#[allow(dead_code)]
pub fn decode<T: Pixel>(src: &[u8]) -> Option<Image<T>> {
    if let Some(image) = bmp::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = png::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = jpeg::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = gif::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = tga::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = tiff::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = pbm::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = xbm::decode::<T>(src) {
        Some(image)
    }
    else if let Some(image) = webp::decode::<T>(src) {
        Some(image)
    }
    else {
        None
    }
}
