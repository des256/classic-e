// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

fn main() {

    // initialize system
    let system = System::new().expect("Unable to access system.");

    // the running variable
    let running = Rc::new(Cell::new(true));
    let window_running = Rc::clone(&running);

    // create application window
    let window = Window::new_frame(&system,rect!(50,50,640,350),"Test Window").expect("Unable to create window.");
    window.set_handler(move |event| {
        match event {
            Event::Close => {
                window_running.set(false);
            },
            _ => {
                println!("{}",event);
            },
        }
    });

    // run the show
    while running.get() {
        system.wait();
        system.flush();
    }
}
