// E - Font test
// Desmond Germans, 2020

use e::*;
use e::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// App structure holds application-wide state. Here it's just a boolean
// indicating if we're still running, and a font reference to be used during
// drawing of the window.
struct App {
    running: bool,
    font: Rc<Font>,
}

// Event handler. Draws 3 words in orange on a blue background.
fn handler(event: Event,app: &mut App) {
    match event {
        Event::Paint(graphics,_) => {

            // clear the space to dark blue
            graphics.clear(ARGB8::from(0xFF003F4F));

            // set color to orange
            graphics.set_color(ARGB8::from(0xFFFF7F00));

            // set blend mode to Over
            graphics.set_blend(BlendMode::Over);

            // draw the texts
            graphics.draw_text(vec2!(10.0,132.0),"WHO",&app.font);
            graphics.draw_text(vec2!(10.0,76.0),"ARE",&app.font);
            graphics.draw_text(vec2!(10.0,20.0),"YOU?",&app.font);
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
    
    // create application state
    let font = ui.graphics().get_font("font.fnt",vec2!(44.0,44.0),0.0).expect("what?");
    let app = Rc::new(RefCell::new(App {
        running: true,
        font: font,
    }));

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
            handler(event,&mut *app);
        }
    );

    // event loop
    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}