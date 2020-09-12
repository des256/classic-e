// E - image - WEBP
// Desmond Germans, 2020

//! WebP

use crate::*;

pub fn test(_src: &[u8]) -> Option<(u32,u32)> {
    None
}

pub fn decode<T: pixel::Pixel>(_src: &[u8]) -> Option<Mat<T>> {
    None
}

pub fn encode<T: pixel::Pixel>(_src: &Mat<T>) -> Option<Vec<u8>> {
    None
}
