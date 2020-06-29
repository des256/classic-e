// e::ui main application
// by Desmond Germans, 2019

use std::time;
use std::thread;

extern crate canvas;

pub struct Application {
    canvas: canvas::Canvas,
}

impl Application {
    pub fn new(width: usize,height: usize,title: &str) -> Application {
        Application {
            canvas: canvas::Canvas::new(width,height,1.0,title),
        }
    }

    pub fn execute(&mut self) {
        loop {
            thread::sleep(time::Duration::from_millis(10));
            if !self.canvas.handle_events() {
                break;
            }
        }
    }
}
