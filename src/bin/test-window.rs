// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

struct MyHandler {
    running: Cell<bool>,
}

impl Handler for MyHandler {
    fn handle(&self,_wc: &WindowContext,event: Event) {
        match event {
            Event::Close => {
                self.running.set(false);
            },
            _ => { },
        }
    }
}

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics
    let _graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create handler object
    let handler = Rc::new(MyHandler { running: Cell::new(true), });

    // open window
    let handler_clone = Rc::clone(&handler);
    let window_id = system.open_frame_window(
        rect!(50,50,640,360),
        "Test Window",
        &(handler_clone as Rc<dyn Handler>)
    );

    // run the show
    while handler.running.get() {
        system.wait();
        system.flush();
    }

    // and close the window again
    system.close_window(window_id);
}
