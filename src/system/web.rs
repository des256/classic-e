// E - Web
// Desmond Germans, 2020

use crate::*;
use crate::prelude::*;

pub struct Video {
}

impl Video {
    pub fn new() -> Video {
        Video {
        }
    }
}

impl Drop for Video {
    fn drop(&mut self) {
    }
}
