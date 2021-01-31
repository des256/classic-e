// E - image
// Desmond Germans, 2020

//! Image formats.

use {
    crate::*,
    std::{
        fs::File,
        io::prelude::*,
    }
};

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod tga;
pub mod gif;
pub mod pbm;
pub mod tiff;
pub mod xbm;
pub mod webp;

/// Test if a slice can be decoded.
/// # Arguments
/// * `src` - Slice to test.
/// # Returns
/// * `None` - Slice cannot be decoded.
/// * `Some((width,height))` - Slice can be decoded and `width`,`height` are the dimensions of the image.
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

/// Decode a slice.
/// # Generic
/// * `T` - The resulting pixel format. 
/// # Arguments
/// * `src` - Slice to decode.
/// # Returns
/// * `None` - Slice could not be decoded.
/// * `Some(mat)` - Slice is decoded into `mat`.
#[allow(dead_code)]
pub fn decode<T: pixel::Pixel>(src: &[u8]) -> Option<Mat<T>> {
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

/// Load and decode
pub fn load<T: pixel::Pixel>(filename: &str) -> Result<Mat<T>,SystemError> {
    //println!("opening {}",filename);
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => { return Err(SystemError::Generic); },
    };
    //println!("reading {}",filename);
    let mut buffer: Vec<u8> = Vec::new();
    if let Ok(_) = file.read_to_end(&mut buffer) {
        //println!("decoding {}",filename);
        match decode::<T>(&buffer) {
            Some(mat) => Ok(mat),
            None => { println!("decoding failed"); Err(SystemError::Generic) },
        }
    }
    else {
        Err(SystemError::Generic)
    }
}