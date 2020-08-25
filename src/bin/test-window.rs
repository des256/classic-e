// E - AppWindow test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create window
    let window = Rc::new(Window::new(
        &system,
        rect!(50,50,640,360),
        "Test Window"
    ).expect("Cannot create window."));

    // main loop
    let mut running = true;
    while running {

        // wait for event to happen
        system.wait();

        // keep track of graphics changes
        let mut rendered = false;

        // process all current events
        for event in system.poll(&window) {

            println!("Event: {:?}",event);
            match event {
                Event::Render => {
                    graphics.bind_target(&window);
                    graphics.clear(0xFF001122);
                    rendered = true;
                },
                Event::Close => {
                    running = false;
                },
                _ => { },
            }
        }

        // if anything was updated, swap buffers
        if rendered {
            gpu::present(&system,&window);
        }
    }
}
