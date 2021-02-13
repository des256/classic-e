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

    // get attached GPUs
    let gpus = system.find_gpus();

    // get attached screens
    let screens = system.find_screens(&gpus);

    // choose first screen
    let screen = &screens[0];

    // the running variable
    let running = Rc::new(Cell::new(true));
    let window_running = Rc::clone(&running);

    // create application window
    let window = screen.create_frame(rect!(50,50,640,350),"Test Window").expect("Unable to create window.");
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
