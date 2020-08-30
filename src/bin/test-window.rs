// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

fn handler(event: Event) -> bool {
    if let Event::Close = event {
        return false;
    }
    true
}

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics
    let _graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create window
    let window = Rc::new(system.open_frame_window(
        rect!(50,50,640,360),
        "Test Window"
    ).expect("Cannot create window."));

    let running = Rc::new(Cell::new(true));
    let closure_running = Rc::clone(&running);
    window.set_handler(move |event| closure_running.set(handler(event)) );

    while running.get() {
        system.wait();
        system.flush();
    }
}
