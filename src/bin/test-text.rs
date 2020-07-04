// E - Text test
// Desmond Germans, 2020

use e::*;
use e::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// App structure holds application-wide state. Here it's just a boolean
// indicating if we're still running, and a text widget.
struct App {
    running: bool,
    text: Text,  // The root object is a Text widget
}

// Event handler. Draws the text widget.
fn handler(event: Event,app: &mut App) {
    match event {
        Event::Paint(graphics,space) => {

            // clear the space to dark blue
            graphics.clear(ARGB8::from(0xFF003F4F));

            // draw the text widget over the whole space
            app.text.draw(graphics,Rect::<f32> { o: vec2!(0.0,0.0),s: space, });
        },
        Event::Close => {
            app.running = false;
        },
        _ => { },
    }
}

fn main() {
    // initialize UI
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(_) => { panic!("Cannot open UI."); },
    };

    // create widget tree
    let mut text = Text::new(ui.graphics(),"Hello, World!");
    text.padding(vec2!(10.0,10.0));
    text.halign(HAlignment::Center);
    text.valign(VAlignment::Top);
    text.color(ARGB8::from(vec4!(255,191,0,255)));

    // create application state
    let app = Rc::new(RefCell::new(App {
        running: true,
        text: text,
    }));

    // clone pointer to give to window
    let cloned_app = Rc::clone(&app);

    // create the window
    ui.create_window(
        rect!(50,50,640,360),
        "Test Window",
        move |event| {
            let mut app = cloned_app.borrow_mut();
            handler(event,&mut app);
        }
    );

    // event loop
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}