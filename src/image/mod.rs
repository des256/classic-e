// E - image
// Desmond Germans, 2020

use crate::*;

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
pub fn decode<T: Copy + Clone + Zero>(src: &[u8]) -> Option<Mat<T>> where T: From<Vec4<u8>>,Vec4<u8>: From<T> {
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
