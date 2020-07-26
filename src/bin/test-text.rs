// E - Text test
// Desmond Germans, 2020

use e::*;
use e::ui::Widget;
use std::rc::Rc;

fn main() {

    // initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics).expect("Cannot open UI."));

    // create window
    let window = Rc::new(Window::new(
        &system,
        rect!(50,50,640,360),
        "Test Window"
    ).expect("unable to create window"));

    // create UI drawing context
    let dc = Rc::new(ui::DC::new(&ui).expect("what?"));

    // create text widget
    let text = Rc::new(ui::Text::new(&ui,"Hello, World!").expect("Cannot create text."));
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
                    graphics.bind_target(&window);
                    graphics.clear(vec4!(0.0,0.3,0.4,1.0));
                    let size = window.size.get();
                    dc.set_size(vec2!(size.x as f32,size.y as f32));
                    text.draw(&dc,rect!(0.0,0.0,size.x as f32,size.y as f32));
                    graphics.flush();
                },

                Event::Resize(s) => {
                    window.size.set(vec2!(s.x as usize,s.y as usize));
                },

                Event::Close => {
                    running = false;
                },

                _ => { },
            }
        }
    }
}
