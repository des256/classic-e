// E - Text test
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize UI
    let ui = Rc::new(UI::new(&system).expect("Cannot open UI."));

    // create window
    let window = Rc::new(Window::new(
        &system,
        rect!(50,50,640,360),
        "Test Window"
    ).expect("unable to create window"));

    // create text widget
    let text = Rc::new(Text::new(&ui,"Hello, World!").expect("Cannot create text."));
    text.set_color(vec4!(1.0,0.5,0.0,1.0));

    // main loop
    let mut running = true;
    while running {

        // wait for event to happen
        system.wait();

        // process all current events
        for event in system.poll(&window) {
            match event {

                Event::Paint(_) => {
                    window.begin_paint();
                    let gc = Rc::new(GC::new(&ui).expect("what?"));
                    system.clear(vec4!(0.0,0.3,0.4,1.0));
                    let size = window.size.get();
                    gc.set_size(vec2!(size.x as f32,size.y as f32));
                    text.draw(&gc,rect!(0.0,0.0,size.x as f32,size.y as f32));
                    window.end_paint();
                },

                Event::Close => {
                    running = false;
                },

                _ => { },
            }
        }
    }
}
