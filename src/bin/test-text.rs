// E - Text test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use e::Text;
use e::Widget;
use e::vec2;
use e::prelude::*;
use e::Rect;
use std::rc::Rc;
use e::rect;
use std::cell::RefCell;
use e::HAlignment;
use e::VAlignment;
use e::ARGB8;
use e::vec4;

// App structure holds application-wide state. Here it's just a boolean
// indicating if we're still running, and a text widget.
struct App {
    running: bool,
    tree: Text,  // The root object is a Text widget
}

// Event handler. Draws the text widget.
fn handler(event: Event,app: &mut App) {
    match event {
        Event::Paint(graphics,space) => {

            // clear the space to dark blue
            graphics.clear(ARGB8::from(0xFF003F4F));

            // draw the text widget over the whole space
            app.tree.draw(graphics,Rect::<f32> { o: vec2!(0.0,0.0),s: space, });
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
    let tree = Text::new(ui.graphics(),"Hello, World!")
        .padding()
        .halign(HAlignment::Center)
        .valign(VAlignment::Top)
        .color(ARGB8::from(vec4!(255,191,0,255)))
    ;

    // create application state
    let app = Rc::new(RefCell::new(App {
        running: true,
        tree: tree,
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