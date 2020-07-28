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

            match event {
                Event::KeyPress(k) => {
                    println!("KeyPress {}",k);
                },
                Event::KeyRelease(k) => {
                    println!("KeyRelease {}",k);
                },
                Event::MousePress(p,b) => {
                    println!("MousePress {},{}",p,b);
                },
                Event::MouseRelease(p,b) => {
                    println!("MouseRelease {},{}",p,b);
                },
                Event::MouseMove(p) => {
                    println!("MouseMove {}",p);
                },
                Event::MouseWheel(b) => {
                    println!("MouseWheel {}",b);
                },
                Event::Resize(s) => {
                    println!("Resize {}",s);
                },
                Event::Render => {
                    println!("Render");
                    graphics.bind_target(&window);
                    graphics.clear(pixel::ARGB8::from(0xFF001122));
                    rendered = true;
                },
                Event::Close => {
                    println!("Close");
                    running = false;
                },
            }
        }

        // if anything was updated, swap buffers
        if rendered {
            graphics.present();
        }
    }
}
