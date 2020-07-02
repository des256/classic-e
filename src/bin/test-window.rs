// E - AppWindow test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use std::rc::Rc;
use std::cell::RefCell;
use e::vec2;
use e::prelude::*;
use e::rect;

// App structure holds application-wide state. Here it's just a boolean
// indicating if we're still running.
struct App {
    running: bool,
}

// Event handler. Prints the name of the window, followed by what kind of
// event it is.
fn handler(name: &str,event: Event,app: &mut App) {
    match event {
        Event::KeyPress(k) => {
            println!("{}: KeyPress {}",name,k);
        },
        Event::KeyRelease(k) => {
            println!("{}: KeyRelease {}",name,k);
        },
        Event::MousePress(p,b) => {
            println!("{}: MousePress {},{}",name,p,b);
        },
        Event::MouseRelease(p,b) => {
            println!("{}: MouseRelease {},{}",name,p,b);
        },
        Event::MouseMove(p) => {
            println!("{}: MouseMove {}",name,p);
        },
        Event::MouseWheel(b) => {
            println!("{}: MouseWheel {}",name,b);
        },
        Event::Resize(s) => {
            println!("{}: Resize {}",name,s);
        },
        Event::Paint(_,s) => {
            println!("{}: Paint {}",name,s);
        },
        Event::Close => {
            println!("{}: Close",name);
            app.running = false;
        }
    }
}

fn main() {
    // initialize UI
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(_) => { panic!("Cannot open UI."); },
    };

    // create application state
    let app = Rc::new(RefCell::new(App { running: true, }));

    // clone pointer to give to window
    let cloned_app = app.clone();

    // create the window
    ui.create_window(
        rect!(50,50,640,360),
        "Test Window",
        move |event| {

            // borrow the app state
            let mut app = cloned_app.borrow_mut();

            // pass down to handler
            handler("Test Window",event,&mut *app);
        }
    );

    // event loop
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}
