// E - image - TIFF
// Desmond Germans, 2020

use crate::*;

pub fn test(_src: &[u8]) -> Option<(u32,u32)> {
    None
}

pub fn decode<T: Copy + Clone + Zero>(_src: &[u8]) -> Option<Mat<T>> {
    None
}

pub fn encode<T: Copy + Clone + Zero>(_src: &Mat<T>) -> Option<Vec<u8>> {
    None
}
